#![allow(clippy::vec_init_then_push)]
/// Pre-build stuff
///
/// This script snatches CSS per enabled component, merges it all to one file,
/// runs it through tailwind, and then minifies it.
///
use std::{
    env,
    fs::{self, File, exists, remove_file},
    io::{BufRead, BufReader, Write, prelude::*},
    path::{Path, PathBuf},
    process::Command,
};

const TAILWIND_URL: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.17/";

macro_rules! features {
    ( $( $x:expr ),* ) => {
        {
            let mut features = vec![];
            $(
                #[cfg(feature = $x)]
                features.push($x);
            )*
            features
        }
    };
}

fn bundle_css(input: PathBuf, mut output: &File) {
    let file = File::open(&input);

    if file.is_err() {
        return;
    }

    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();

    buf_reader
        .read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Error reading {}", input.display()));

    output
        .write_all(contents.as_bytes())
        .expect("Error writing bundle");
}

fn download_file(download_url: &str, file_path: &PathBuf) {
    File::create(file_path).expect("Error creating file");

    let mut file = File::options()
        .append(true)
        .open(file_path)
        .expect("Error opening file");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3600))
        .build()
        .expect("Error building client");

    let response = client
        .get(download_url)
        .send()
        .expect("Error getting response");

    let content = response.bytes().expect("Error getting bytes from response");

    file.write_all(&content).expect("Error writing to file");
}

fn run_tailwind(
    tailwind_path: Option<&Path>,
    bundle_path: &PathBuf,
    bundle_dark_path: &PathBuf,
    singlestage_path: &PathBuf,
    singlestage_dark_path: &PathBuf,
) -> Result<(), ()> {
    let output = Command::new(tailwind_path.unwrap_or(Path::new("tailwindcss")))
        .arg("-i")
        .arg(bundle_path)
        .arg("-o")
        .arg(singlestage_path)
        .arg("-m")
        .output();

    if let Ok(output) = output {
        if !output.status.success() {
            let error = String::from_utf8(output.stderr).unwrap();
            panic!("{}", error);
        }
    } else {
        return Err(());
    }

    let output_dark = Command::new(tailwind_path.unwrap_or(Path::new("tailwindcss")))
        .arg("-i")
        .arg(bundle_dark_path)
        .arg("-o")
        .arg(singlestage_dark_path)
        .arg("-m")
        .output();

    if let Ok(output_dark) = output_dark {
        if !output_dark.status.success() {
            let error = String::from_utf8(output_dark.stderr).unwrap();
            panic!("{}", error);
        }
    } else {
        return Err(());
    }

    Ok(())
}

fn main() {
    // Skip css bundling and tailwind if the user doesn't use theme_provider
    if cfg!(not(feature = "theme_provider")) {
        return;
    }

    let out_dir = env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env. (1)\n");
    let bundle_path = Path::new(&out_dir).join("bundle.css");
    let bundle_dark_path = Path::new(&out_dir).join("bundle_dark.css");
    let singlestage_path = Path::new(&out_dir).join("singlestage.css");
    let singlestage_dark_path = Path::new(&out_dir).join("singlestage_dark.css");

    // Skip css bundling and tailwind for docs.rs
    if env::var("DOCS_RS").is_ok() {
        File::create(&singlestage_path).expect("\nError creating dummy file.\n");
        return;
    }

    // Build list of css files to include
    let features = features!(
        "accordion",
        "alert",
        "aspect_ratio",
        "avatar",
        "badge",
        "breadcrumb",
        "button",
        "button_group",
        "card",
        "carousel",
        "checkbox",
        "dialog",
        "dropdown",
        "empty",
        "field",
        "input",
        "input_group",
        "item",
        "kbd",
        "label",
        "link",
        "pagination",
        "popover",
        "progress",
        "radio",
        "scroll_area",
        "select",
        "separator",
        "sidebar",
        "skeleton",
        "slider",
        "spinner",
        "switch",
        "toggle",
        "table",
        "tabs",
        "textarea",
        "tooltip"
    );

    // Start merging css for selected features
    let bundle = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bundle_path)
        .expect("\nError opening bundle file.\n");

    let bundle_dark = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bundle_dark_path)
        .expect("\nError opening bundle file.\n");

    // Theme provider goes first
    #[cfg(feature = "theme_provider")]
    bundle_css(
        Path::new("src")
            .join("components")
            .join("theme_provider")
            .join("main.css"),
        &bundle,
    );
    #[cfg(feature = "theme_provider")]
    bundle_css(
        Path::new("src")
            .join("components")
            .join("theme_provider")
            .join("main.css"),
        &bundle_dark,
    );
    #[cfg(feature = "theme_provider")]
    bundle_css(
        Path::new("src")
            .join("components")
            .join("theme_provider")
            .join("main_dark.css"),
        &bundle_dark,
    );

    // Bundle css for each feature
    for feature in features {
        let feature_flag = format!("CARGO_FEATURE_{}", feature.to_uppercase());

        if env::var(&feature_flag).is_ok() {
            let feature_base_css = Path::new("src")
                .join("components")
                .join(feature)
                .join("style")
                .join("base")
                .join("vega")
                .join(format!("{}.css", &feature));

            let feature_base_dark_css = Path::new("src")
                .join("components")
                .join(feature)
                .join("style")
                .join("base")
                .join("vega")
                .join(format!("{}_dark.css", &feature));

            let feature_css = Path::new("src")
                .join("components")
                .join(feature)
                .join("style")
                .join(format!("{}.css", &feature));

            let feature_dark_css = Path::new("src")
                .join("components")
                .join(feature)
                .join("style")
                .join(format!("{}_dark.css", &feature));

            bundle_css(feature_base_css, &bundle);
            bundle_css(feature_base_dark_css, &bundle_dark);
            bundle_css(feature_css, &bundle);
            bundle_css(feature_dark_css, &bundle_dark);
        }
    }

    // Tailwind

    let _cleanup = remove_file(&singlestage_path);

    // User brought their own tailwind
    if let Ok(tailwind_path) = env::var("SINGLESTAGE_TAILWIND_PATH") {
        if run_tailwind(
            Some(Path::new(&tailwind_path)),
            &bundle_path,
            &bundle_dark_path,
            &singlestage_path,
            &singlestage_dark_path,
        )
        .is_ok()
        {
            // BYOT tailwind worked, bail
            return;
        }
        panic!(
            "\nRunning tailwind at `{}` didn't work.\nIs it executable? (sudo chmod +x)\nIs this a full path?\n",
            tailwind_path
        )
    }

    // Try system tailwind
    if run_tailwind(
        None,
        &bundle_path,
        &bundle_dark_path,
        &singlestage_path,
        &singlestage_dark_path,
    )
    .is_ok()
    {
        // System tailwind worked, bail
        return;
    }

    let mut filename: String = String::from("tailwindcss");

    match env::consts::OS {
        "linux" => filename.push_str("-linux"),
        "windows" => filename.push_str("-windows"),
        "macos" => filename.push_str("-macos"),
        _ => panic!("\nThis platform is not supported at this time.\n"),
    };

    match env::consts::ARCH {
        "x86_64" => filename.push_str("-x64"),
        "aarch64" => filename.push_str("-arm64"),
        _ => panic!("\nThis platform is not supported at this time.\n"),
    }

    // TODO: handle no windows aarch64
    if env::consts::OS == "windows" {
        filename.push_str(".exe")
    }

    println!("Filename: {}", filename);

    // Try downloaded tailwind
    let downloaded_tailwind_path =
        Path::new(&env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env. (2)\n"))
            .join(&filename);
    if run_tailwind(
        Some(&downloaded_tailwind_path),
        &bundle_path,
        &bundle_dark_path,
        &singlestage_path,
        &singlestage_dark_path,
    )
    .is_ok()
    {
        // Downloaded tailwind worked, bail
        return;
    }

    let tailwind =
        Path::new(&env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env. (3)\n"))
            .join(&filename);

    if !exists(&tailwind).expect("\nError checking for tailwind.\n") {
        let file_url = format!("{}{}", &TAILWIND_URL, &filename);
        download_file(&file_url, &tailwind);
    }

    let checksums =
        Path::new(&env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env. (4)\n"))
            .join("sha256sums.txt");

    if !exists(&checksums).expect("\nError checking for checksums.\n") {
        let sums_url = format!("{}sha256sums.txt", &TAILWIND_URL);
        download_file(&sums_url, &checksums);
    }

    let sums = File::open(&checksums).expect("\nError opening checksums.\n");
    let buf_reader = BufReader::new(sums);

    let mut expected_checksum = "".to_string();

    for line in buf_reader.lines().map_while(Result::ok) {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        if format!("./{}", filename) == split_line[1] {
            expected_checksum = split_line[0].into()
        }
    }

    let calculated_checksum =
        sha256::try_digest(&tailwind).expect("\nError calculating checksum.\n");

    println!("Expected Checksum: {}", expected_checksum);
    println!("Calculated Checksum: {}", calculated_checksum);

    if expected_checksum == calculated_checksum {
        println! {"Checksum match"};
    } else {
        println! {"Checksum mismatch!"};
        let _idc_if_this_fails = remove_file(tailwind);
        panic!("\nChecksum mismatch!\n");
    }

    if env::consts::FAMILY == "unix" {
        Command::new("chmod")
            .arg("+x")
            .arg(
                Path::new(
                    &env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env. (5)\n"),
                )
                .join(&filename),
            )
            .output()
            .expect("\nError running chmod +x\n");
    }

    // Run downloaded tailwind
    let _ = run_tailwind(
        Some(&downloaded_tailwind_path),
        &bundle_path,
        &bundle_dark_path,
        &singlestage_path,
        &singlestage_dark_path,
    );
}
