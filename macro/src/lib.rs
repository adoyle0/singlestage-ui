extern crate proc_macro;
mod highlight;
use highlight::*;
use proc_macro::TokenStream;
use serde::Deserialize;
use std::{
    fs::*,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Deserialize)]
struct ComponentSpecialInfo {
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct ComponentExample {
    name: String,
    title: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct ReferenceAttribute {
    attr: String,
    attr_type: String,
    default: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct ComponentReference {
    name: String,
    description: String,
    extra: Option<String>,
    attrs: Vec<ReferenceAttribute>,
}

#[derive(Deserialize)]
struct ComponentPage {
    name: String,
    description: String,
    info: Option<ComponentSpecialInfo>,
    examples: Option<Vec<ComponentExample>>,
    references: Vec<ComponentReference>,
}

/// Converts string to kebab-case
fn string_to_kebab(input: &str) -> String {
    input.to_lowercase().replace(" ", "-").replace("_", "-")
}

/// Converts word to Title case
fn word_to_title(input: &str) -> String {
    if input.is_empty() {
        String::new()
    } else {
        let (head, tail) = input.split_at(1);
        let mut buf = head.to_uppercase();
        buf.push_str(tail.to_lowercase().as_str());
        buf
    }
}

/// Converts string To Title Case
fn string_to_title(input: &str) -> String {
    let buf = input
        .split(" ")
        .flat_map(|word| word.split("_"))
        .flat_map(|word| word.split("-"))
        .map(|word| format!("{} ", word_to_title(word)))
        .collect::<String>();

    buf.trim().to_string()
}

/// Converts string to PascalCase.
fn string_to_pascal(input: &str) -> String {
    input
        .split(" ")
        .flat_map(|word| word.split("_"))
        .flat_map(|word| word.split("-"))
        .map(word_to_title)
        .collect::<String>()
}

/// Converts string to snake_case
fn string_to_snake(input: &str) -> String {
    input.to_lowercase().replace(" ", "_")
}

#[proc_macro]
pub fn generate_component_links(_input: TokenStream) -> TokenStream {
    let mut output = String::from("view!{");
    let comp_path = Path::new("./docs/src/routes/components");
    let mut names = vec![];

    for cur_dir in read_dir(comp_path).expect("Error reading dir") {
        let path = cur_dir.expect("Error reading cur dir").path();

        if !path.is_dir() {
            continue;
        }

        // Skip directories marked with a leading "_"
        let component_module_name = path
            .to_str()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .to_owned();
        if component_module_name.chars().next().unwrap() == "_".chars().next().unwrap() {
            continue;
        }

        if let Ok(file) = File::open(path.join("component.toml")) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            buf_reader
                .read_to_string(&mut contents)
                .expect("Error reading file");

            let decoded: ComponentPage = toml::from_str(&contents).expect("Error decoding toml");
            names.push(decoded.name);
        } else {
            println!(
                "WARN: component.toml not found at {}",
                path.to_str().unwrap()
            );
        }
    }

    names.sort();
    for name in names {
        output.push_str(
            format!(
                r#"

        <SidebarMenuItem>
            <SidebarMenuButton>
                <A href="/components/{}">
                    <span>"{}"</span>
                </A>
            </SidebarMenuButton>
        </SidebarMenuItem>
        "#,
                string_to_kebab(&name),
                string_to_title(&name)
            )
            .as_str(),
        )
    }

    output.push('}');

    output.parse().expect("Error parsing output")
}

#[proc_macro]
pub fn generate_component_routes(_input: TokenStream) -> TokenStream {
    let mut output = String::from(
        r#"
use leptos_router::MatchNestedRoutes;

#[component(transparent)]
pub fn ComponentRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute
            path=StaticSegment("/components")
            view=move || view! { <Outlet /> }
        >"#,
    );

    let comp_path = Path::new("./docs/src/routes/components");

    for cur_dir in read_dir(comp_path).unwrap() {
        let path = cur_dir.unwrap().path();
        if !path.is_dir() {
            continue;
        }

        // Skip directories marked with a leading "_"
        let component_module_name = path
            .to_str()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .to_owned();
        if component_module_name.chars().next().unwrap() == "_".chars().next().unwrap() {
            continue;
        }

        if let Ok(file) = File::open(path.join("component.toml")) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            buf_reader.read_to_string(&mut contents).unwrap();

            let decoded: ComponentPage = toml::from_str(&contents).unwrap();

            output.push_str(
                format!(
                    r#"
            <Route path=StaticSegment("/{}") view={}Route />"#,
                    string_to_kebab(&decoded.name),
                    string_to_pascal(&decoded.name)
                )
                .as_str(),
            )
        } else {
            println!(
                "WARN: component.toml not found at {}",
                path.to_str().unwrap()
            );
        }
    }

    output.push_str(
        r#"
        </ParentRoute>
    }
    .into_inner()
}"#,
    );

    output.parse().expect("Error parsing output")
}

#[proc_macro]
pub fn generate_component_pages(_input: TokenStream) -> TokenStream {
    let mut output = String::new();
    let comp_path = Path::new("./docs/src/routes/components");

    for cur_dir in read_dir(comp_path)
        .unwrap_or_else(|_| panic!("Error reading dir {}", &comp_path.to_str().unwrap()))
    {
        let path = cur_dir.unwrap().path();
        if !path.is_dir() {
            continue;
        }

        let component_module_name = path
            .to_str()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .to_owned();

        // Skip directories marked with a leading "_"
        if component_module_name.chars().next().unwrap() == "_".chars().next().unwrap() {
            continue;
        }

        // Read component.toml
        let component_toml = File::open(path.join("component.toml"));
        if component_toml.is_err() {
            println!(
                "WARN: component.toml not found at {}",
                path.to_str().unwrap()
            );
            continue;
        }
        let component_toml = component_toml.unwrap();
        let mut contents = String::new();
        let mut buf_reader = BufReader::new(component_toml);
        buf_reader
            .read_to_string(&mut contents)
            .expect("Error reading file");

        let decoded: ComponentPage = toml::from_str(&contents).expect("Error reading toml");

        // Generate info block
        let mut use_alert_icon = String::new();
        let mut info = String::new();

        if let Some(special_info) = decoded.info {
            use_alert_icon.push_str(
                r#"use singlestage::{alert::*, icon};
"#,
            );
            info.push_str(
                format!(
                    r#"<Alert>
                {{icon!(icondata::LuInfo)}}
                <AlertTitle>"{}"</AlertTitle>
                <AlertDescription>"{}"</AlertDescription>
            </Alert>
            "#,
                    special_info.title, special_info.description
                )
                .as_str(),
            );
        }

        // Generate examples
        let mut examples = String::new();

        if let Some(decoded_examples) = decoded.examples {
            for example in decoded_examples {
                let mut title = String::new();
                if let Some(example_title) = example.title {
                    title = format!(r#"name="{}" "#, example_title);
                }

                let mut description = String::new();
                if let Some(example_description) = example.description {
                    description = format!(r#"description="{}" "#, example_description);
                }

                let path = format!(
                    "./docs/src/routes/components/{}/examples/{}.rs",
                    component_module_name,
                    string_to_snake(&example.name)
                );

                let example_code = highlight_html_from_file(path);

                examples.push_str(
                    format!(
                        r##"<Example {}{}view={}Example.into_any() code=r#"{}"# />
            "##,
                        title.as_str(),
                        description.as_str(),
                        string_to_pascal(&example.name),
                        example_code
                    )
                    .as_str(),
                );
            }
        }

        // Generate API references
        let mut references = String::new();
        for reference in decoded.references {
            let mut attrs = String::new();

            // HACK: do this right, don't duplicate
            if reference.attrs.is_empty() {
                references.push_str(
                    format!(
                        r###"
                <Reference
                    name="{}"
                    description="{}"
                    extra=r##"{}"##
                    table=None
                />
"###,
                        reference.name,
                        reference.description,
                        reference.extra.unwrap_or_default(),
                    )
                    .as_str(),
                )
            } else {
                for attr in reference.attrs {
                    attrs.push_str(
                        format!(
                            r##"
                            (r#"{}"#,r#"{}"#,r#"{}"#,r#"{}"#),"##,
                            attr.attr,
                            attr.attr_type,
                            attr.default,
                            attr.description.unwrap_or_default()
                        )
                        .as_str(),
                    )
                }
                // Remove trailing comma
                let _ = attrs.pop();

                references.push_str(
                    format!(
                        r###"
                <Reference
                    name="{}"
                    description="{}"
                    extra=r##"{}"##
                    table=Some(attr_rows!({})
                        .into_any())
                />
"###,
                        reference.name,
                        reference.description,
                        reference.extra.unwrap_or_default(),
                        attrs,
                    )
                    .as_str(),
                )
            }
        }

        // Generate anatomy snippet
        let path = format!(
            "./docs/src/routes/components/{}/anatomy.rs",
            component_module_name
        );
        let anatomy = highlight_html_from_file(path);

        output.push_str(
            format!(
                r##"
mod {};
use {}::*;
{}
#[component]
pub fn {}Route() -> impl IntoView {{
    view! {{
        <ComponentTemplate name="{}" description="{}">
            {}{}
            <Anatomy code=r#"{}"# />

            <API>{}
            </API>
        </ComponentTemplate>
    }}
}}
"##,
                component_module_name,
                component_module_name,
                use_alert_icon,
                string_to_pascal(&decoded.name),
                string_to_title(&decoded.name),
                decoded.description,
                info,
                examples,
                anatomy,
                references
            )
            .as_str(),
        );
    }
    output.parse().expect("Error parsing output")
}
