/// Pre-build stuff
///
/// This script snatches CSS per enabled component, merges it all to one file,
/// runs it through tailwind, and then minifies it.
///
use std::{
    env,
    fs::{self, File, exists},
    io::{BufRead, BufReader, Write, prelude::*},
    path::{Path, PathBuf},
    process::Command,
};

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
    let file = File::open(&input).expect(format!("Error opening {}", &input.display()).as_str());
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader
        .read_to_string(&mut contents)
        .expect(format!("Error reading {}", input.display()).as_str());

    output
        .write_all(&contents.as_bytes())
        .expect("Error writing bundle");
}

fn download_file(download_url: &str, file_path: &PathBuf) {
    File::create(&file_path).expect("Error creating file");

    let mut file = File::options()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Error opening file");

    let response = reqwest::blocking::get(download_url).expect("Error getting response");
    let content = response.bytes().expect("Error getting bytes from response");
    file.write_all(&content).expect("Error writing to file");
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env");
    let bundle_path = Path::new(&out_dir).join("bundle.css");
    let singlestage_path = Path::new(&out_dir).join("singlestage.css");

    // Build list of css files to include
    let features = features!(
        "accordion",
        "alert",
        "avatar",
        "badge",
        "breadcrumb",
        "button",
        "card",
        "checkbox",
        "dialog",
        "dropdown",
        "input",
        "label",
        "link",
        "pagination",
        "popover",
        "radio",
        "select",
        "separator",
        "sidebar",
        "skeleton",
        "slider",
        "switch",
        "table",
        "tabs",
        "textarea",
        "toast",
        "tooltip"
    );

    // Start merging css for selected features
    let bundle = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bundle_path)
        .expect("Error opening test file");

    // Theme provider goes first
    #[cfg(feature = "theme_provider")]
    let main_css_path = Path::new("src")
        .join("components")
        .join("theme_provider")
        .join("main.css");
    #[cfg(feature = "theme_provider")]
    bundle_css(main_css_path, &bundle);

    // Bundle css for each feature
    for feature in features {
        let feature_flag = format!("CARGO_FEATURE_{}", feature.to_uppercase());

        if env::var(&feature_flag).is_ok() {
            let feature_css = Path::new("src")
                .join("components")
                .join(&feature)
                .join(format!("{}.css", &feature));
            bundle_css(feature_css, &bundle);
        }
    }

    // Tailwind
    //
    // Try system tailwind
    if Command::new("tailwindcss")
        .arg("-i")
        .arg(&bundle_path)
        .arg("-o")
        .arg(&singlestage_path)
        .arg("-m")
        .output()
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
        _ => panic!("This platform is not supported at this time."),
    };

    match env::consts::ARCH {
        "x86_64" => filename.push_str("-x64"),
        "aarch64" => filename.push_str("-arm64"),
        _ => panic!("This platform is not supported at this time."),
    }

    // TODO: handle no windows aarch64
    if env::consts::OS == "windows" {
        filename.push_str(".exe")
    }

    println!("Filename: {}", filename);

    // Try downloaded tailwind
    if Command::new(
        Path::new(&env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env")).join(&filename),
    )
    .arg("-i")
    .arg(&bundle_path)
    .arg("-o")
    .arg(&singlestage_path)
    .arg("-m")
    .output()
    .is_ok()
    {
        // Downloaded tailwind worked, bail
        return;
    }

    let url_base = "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.14/";

    let tailwind =
        Path::new(&env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env")).join(&filename);

    if !exists(&tailwind).expect("Error checking for tailwind") {
        let file_url = format!("{}{}", &url_base, &filename);
        download_file(&file_url, &tailwind);
    }

    let checksums = Path::new(&env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env"))
        .join("sha256sums.txt");

    if !exists(&checksums).expect("Error checking for checksums") {
        let sums_url = format!("{}sha256sums.txt", &url_base);
        download_file(&sums_url, &checksums);
    }

    let sums = File::open(&checksums).expect("Error opening checksums");
    let buf_reader = BufReader::new(sums);

    let mut expected_checksum = "".to_string();

    for line in buf_reader.lines() {
        if let Ok(line) = line {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            if format!("./{}", filename) == split_line[1] {
                expected_checksum = split_line[0].into()
            }
        }
    }

    let calculated_checksum = sha256::try_digest(tailwind).expect("Error calculating checksum");

    println!("Expected Checksum: {}", expected_checksum);
    println!("Calculated Checksum: {}", calculated_checksum);

    if expected_checksum == calculated_checksum {
        println! {"Checksum match"};
    } else {
        println! {"Checksum mismatch"};
        panic!("Checksum mismatch");
    }

    if env::consts::FAMILY == "unix" {
        Command::new("chmod")
            .arg("+x")
            .arg(
                Path::new(&env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env"))
                    .join(&filename),
            )
            .output()
            .expect("Error running chmod +x");
    }

    // Run downloaded tailwind
    Command::new(Path::new(&env::var_os("OUT_DIR").expect("Error reading OUT_DIR from env")).join(&filename))
        .arg("-i")
        .arg(&bundle_path)
        .arg("-o")
        .arg(&singlestage_path)
        .arg("-m")
        .output()
        .expect(
            "\n`tailwindcss` not found in PATH. Is it installed? See:\n https://tailwindcss.com/docs/installation/tailwind-cli or\n https://github.com/tailwindlabs/tailwindcss/releases/latest\nOr install using your package manager.\n\n",
        );
}
