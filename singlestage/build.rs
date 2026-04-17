use rayon::prelude::*;
use std::{
    env,
    fs::{self, File, exists, remove_file},
    io::{BufRead, BufReader, Write, prelude::*},
    path::{Path, PathBuf},
    process::Command,
};

const TAILWIND_URL: &str = "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/";

fn tailwind_ready(path: &PathBuf) -> bool {
    let output = Command::new(path).output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}

fn download_file(download_url: &str, file_path: &Path) {
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
    tailwind_path: &PathBuf,
    bundle_path: &PathBuf,
    output_path: &PathBuf,
    css: String,
) {
    let mut bundle = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bundle_path)
        .expect("\nError opening bundle file.\n");
    let _ = bundle.write_all(css.as_bytes()).unwrap();

    let _cleanup = remove_file(&output_path);

    let output = Command::new(&tailwind_path)
        .arg("-i")
        .arg(bundle_path)
        .arg("-o")
        .arg(output_path)
        .arg("-m")
        .output();

    if let Ok(output) = output
        && !output.status.success()
    {
        let error = String::from_utf8(output.stderr).unwrap();
        panic!("{}", error);
    }
}

fn main() {
    // Skip css bundling and tailwind if the user doesn't use theme_provider
    if cfg!(not(feature = "theme_provider")) {
        return;
    }

    let out_dir = env::var_os("OUT_DIR").expect("\nError reading OUT_DIR from env.\n");
    let singlestage_path = Path::new(&out_dir).join("singlestage.css");
    // Skip css bundling and tailwind for docs.rs
    if env::var("DOCS_RS").is_ok() {
        File::create(&singlestage_path).expect("\nError creating dummy file.\n");
        return;
    }

    // Tailwind
    println!("Checking for tailwind...");
    let mut tailwind_path: PathBuf = Path::new("tailwindcss").to_path_buf();

    // User brought their own tailwind
    if let Ok(user_path) = env::var("SINGLESTAGE_TAILWIND_PATH") {
        println!("User supplied their own tailwind path...");

        let path = Path::new(&user_path).to_path_buf();
        if tailwind_ready(&path) {
            tailwind_path = path;
        } else {
            panic!(
                "\nRunning tailwind at `{}` didn't work.\nIs it executable? (sudo chmod +x)\nIs this a full path?\n",
                user_path
            );
        }
    };

    // Try system tailwind
    if !tailwind_ready(&tailwind_path) {
        println!("System installed tailwind not found...");

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

        // Try possibly already downloaded tailwind (rebuild)
        tailwind_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join(&filename);
        if !tailwind_ready(&tailwind_path) {
            println!("Cached downloaded tailwind not found...");

            if !exists(&tailwind_path).expect("\nError checking for tailwind.\n") {
                println!("Downloading tailwind...");

                let file_url = format!("{}{}", &TAILWIND_URL, &filename);
                download_file(&file_url, &tailwind_path);
            }

            let checksums = Path::new(&env::var_os("OUT_DIR").unwrap()).join("sha256sums.txt");
            if !exists(&checksums).expect("\nError checking for checksums.\n") {
                println!("Downloading checksums...");

                let sums_url = format!("{}sha256sums.txt", &TAILWIND_URL);
                download_file(&sums_url, &checksums);
            }

            let sums = File::open(&checksums).expect("\nError opening checksums.\n");
            let buf_reader = BufReader::new(sums);
            let mut expected_checksum = String::new();

            for line in buf_reader.lines().map_while(Result::ok) {
                let split_line = line.split_whitespace().collect::<Vec<&str>>();
                if format!("./{}", filename) == split_line[1] {
                    expected_checksum = split_line[0].into()
                }
            }

            let calculated_checksum =
                sha256::try_digest(&tailwind_path).expect("\nError calculating checksum.\n");

            println!("Expected Checksum: {}", expected_checksum);
            println!("Calculated Checksum: {}", calculated_checksum);

            if expected_checksum == calculated_checksum {
                println! {"Checksums match!"};
            } else {
                println! {"Checksum mismatch!"};
                let _idc_if_this_fails = remove_file(tailwind_path);
                panic!("\nChecksum mismatch!\n");
            }

            println!("Making tailwind executable...");
            if env::consts::FAMILY == "unix" {
                Command::new("chmod")
                    .arg("+x")
                    .arg(
                        Path::new(
                            &env::var_os("OUT_DIR")
                                .expect("\nError reading OUT_DIR from env. (5)\n"),
                        )
                        .join(&filename),
                    )
                    .output()
                    .expect("\nError running chmod +x!\n");
            }
        }
    }

    if tailwind_ready(&tailwind_path) {
        println!("Tailwind is ready!")
    } else {
        panic!(
            "\nRunning tailwind at `{}` didn't work.\nIs it executable? (sudo chmod +x)\nIs this a full path?\n",
            tailwind_path.display()
        );
    }

    // Bundling
    println!("Bundling CSS...");

    let features = [
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
        "sheet",
        "sidebar",
        "skeleton",
        "slider",
        "spinner",
        "switch",
        "toggle",
        "table",
        "tabs",
        "textarea",
        "tooltip",
    ];

    let base_styles = ["luma", "lyra", "maia", "mira", "nova", "sera", "vega"];

    let mut css_raw = String::new();
    let _ = File::open(
        &Path::new("src")
            .join("components")
            .join("theme_provider")
            .join("main.css"),
    )
    .unwrap()
    .read_to_string(&mut css_raw);

    let mut buf = String::new();
    let _ = File::open(
        &Path::new("src")
            .join("components")
            .join("theme_provider")
            .join("main_dark.css"),
    )
    .unwrap()
    .read_to_string(&mut buf);
    let css_dark_raw = format!("{}{}", &css_raw, buf);

    ["", "_dark"].par_iter().for_each(|mode| {
        let component_css = format!(
            "{}{}",
            match *mode {
                "" => &css_raw,
                _ => &css_dark_raw,
            },
            features
                .par_iter()
                .map(|feature| {
                    let feature_flag = format!("CARGO_FEATURE_{}", feature.to_uppercase());
                    if env::var(&feature_flag).is_ok()
                        && let Ok(mut file) = File::open(
                            &Path::new("src")
                                .join("components")
                                .join(feature)
                                .join("style")
                                .join(format!("{}{}.css", &feature, mode)),
                        )
                    {
                        let mut buf = String::new();

                        if let Ok(read) = file.read_to_string(&mut buf)
                            && read > 0
                        {
                            buf
                        } else {
                            "".into()
                        }
                    } else {
                        "".into()
                    }
                })
                .collect::<String>()
        );
        let bundle_path = Path::new(&out_dir).join(format!("bundle{}.css", mode));
        let output_path = Path::new(&out_dir).join(format!("singlestage{}.css", mode));
        run_tailwind(&tailwind_path, &bundle_path, &output_path, component_css);
        // TODO: Reduce dark style bloat
        println!("Bundled css{}...", mode);

        base_styles.par_iter().for_each(|base_style| {
            if env::var(&format!(
                "CARGO_FEATURE_STYLE_{}",
                base_style.to_uppercase()
            ))
            .is_ok()
            {
                let base_css = format!(
                    "{}{}",
                    match *mode {
                        "" => &css_raw,
                        _ => &css_dark_raw,
                    },
                    features
                        .par_iter()
                        .map(|feature| {
                            let feature_flag = format!("CARGO_FEATURE_{}", feature.to_uppercase());
                            if env::var(&feature_flag).is_ok()
                                && let Ok(mut file) = File::open(
                                    &Path::new("src")
                                        .join("components")
                                        .join(feature)
                                        .join("style")
                                        .join("base")
                                        .join(base_style)
                                        .join(format!("{}{}.css", &feature, mode)),
                                )
                            {
                                let mut buf = String::new();
                                if let Ok(read) = file.read_to_string(&mut buf)
                                    && read > 0
                                {
                                    buf
                                } else {
                                    "".into()
                                }
                            } else {
                                "".into()
                            }
                        })
                        .collect::<String>()
                );
                let bundle_path =
                    Path::new(&out_dir).join(format!("bundle_{}{}.css", base_style, mode));
                let output_path =
                    Path::new(&out_dir).join(format!("singlestage_{}{}.css", base_style, mode));
                run_tailwind(&tailwind_path, &bundle_path, &output_path, base_css);
                // TODO: Reduce dark style bloat
                // TODO: Reduce base style bloat
                println!("Bundled base_{}{}...", base_style, mode);
            }
        });
    });

    println!("Finished!");
}
