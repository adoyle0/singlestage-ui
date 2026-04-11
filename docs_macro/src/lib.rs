extern crate proc_macro;

mod highlight;

use highlight::*;
use proc_macro::TokenStream;
use rayon::prelude::*;
use serde::Deserialize;
use std::{
    fs::*,
    io::{BufReader, Read},
    path::{Path, PathBuf},
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

impl ComponentPage {
    fn from_path(path: &PathBuf) -> Self {
        let file = File::open(path.join("component.toml")).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .expect("Error reading file");

        toml::from_str(&contents).expect("Error decoding toml")
    }
}

/// Converts string to kebab-case
fn string_to_kebab(input: &str) -> String {
    input
        .chars()
        .map(|mut c| match c {
            ' ' | '_' => '-',
            _ => {
                c.make_ascii_lowercase();
                c
            }
        })
        .collect()
}

/// Converts string To Title Case
fn string_to_title(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    let mut input_iter = input.char_indices();

    while let Some((index, mut char)) = input_iter.next() {
        if index == 0 {
            char.make_ascii_uppercase();
            buf.push(char);
            continue;
        }

        match char {
            ' ' | '-' | '_' => {
                buf.push(' ');
                if let Some((_, mut char)) = input_iter.next() {
                    char.make_ascii_uppercase();
                    buf.push(char)
                } else {
                    break;
                }
            }
            _ => {
                char.make_ascii_lowercase();
                buf.push(char)
            }
        }
    }

    buf
}

/// Converts string to PascalCase.
fn string_to_pascal(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    let mut input_iter = input.char_indices();

    while let Some((index, mut char)) = input_iter.next() {
        if index == 0 {
            char.make_ascii_uppercase();
            buf.push(char);
            continue;
        }

        buf.push(match char {
            ' ' | '-' | '_' => {
                if let Some((_, mut char)) = input_iter.next() {
                    char.make_ascii_uppercase();
                    char
                } else {
                    break;
                }
            }
            _ => {
                char.make_ascii_lowercase();
                char
            }
        })
    }

    buf
}

/// Converts string to snake_case
fn string_to_snake(input: &str) -> String {
    input
        .chars()
        .map(|mut c| {
            if c == ' ' {
                '_'
            } else {
                c.make_ascii_lowercase();
                c
            }
        })
        .collect()
}

fn get_component_paths() -> Vec<PathBuf> {
    let comp_path = Path::new("./docs/src/routes/components");

    let mut buf = vec![];

    read_dir(comp_path).unwrap().into_iter().for_each(|dir| {
        let path = dir.unwrap().path();
        if path.is_dir() {
            if exists(path.join("component.toml")).unwrap() {
                buf.push(path)
            } else {
                println!(
                    "WARN: component.toml not found at {}",
                    path.to_str().unwrap()
                );
            }
        }
    });

    buf
}

#[proc_macro]
pub fn generate_component_links(_input: TokenStream) -> TokenStream {
    let mut names = get_component_paths()
        .par_iter()
        .map(|path| {
            let component = ComponentPage::from_path(path);

            component.name.to_lowercase()
        })
        .collect::<Vec<String>>();

    names.sort();

    let links = names
        .par_iter()
        .map(|name| {
            format!(
                r#"
        <SidebarMenuItem>
            <SidebarMenuButton>
                <Link
                    href="/components/{}">
                    <span>"{}"</span>
                </Link>
            </SidebarMenuButton>
        </SidebarMenuItem>
        "#,
                string_to_kebab(&name),
                string_to_title(&name)
            )
        })
        .collect::<String>();

    format!("view!{{{}}}", links)
        .parse()
        .expect("Error parsing output")
}

#[proc_macro]
pub fn generate_component_routes(_input: TokenStream) -> TokenStream {
    let routes = get_component_paths()
        .par_iter()
        .map(|path| {
            let component = ComponentPage::from_path(path);

            format!(
                r#"
            <Route path=StaticSegment("/{}") view={}Route />
"#,
                string_to_kebab(&component.name),
                string_to_pascal(&component.name)
            )
        })
        .collect::<String>();

    format!(
        r#"
use leptos_router::MatchNestedRoutes;

#[component(transparent)]
pub fn ComponentRoutes() -> impl MatchNestedRoutes + Clone {{
    view! {{
        <ParentRoute
            path=StaticSegment("/components")
            view=move || view! {{ <Outlet /> }}
        >{}
        </ParentRoute>
    }}
    .into_inner()
}}"#,
        &routes
    )
    .parse()
    .expect("Error parsing output")
}

#[proc_macro]
pub fn generate_component_pages(_input: TokenStream) -> TokenStream {
    // Set up some syntax highlighting stuff up front so it's not done per thread
    use std::sync::Arc;
    use syntect::{
        highlighting::ThemeSet, html::start_highlighted_html_snippet, parsing::SyntaxSet,
    };

    let ss = Arc::new(SyntaxSet::load_defaults_newlines());
    let ts = ThemeSet::load_defaults();
    let theme = Arc::new(&ts.themes["Solarized (light)"]);
    let (_, bg) = start_highlighted_html_snippet(&theme);
    let bg = Arc::new(bg);

    get_component_paths()
        .par_iter()
        .map(|path| {
            let component = ComponentPage::from_path(&path);
            let component_module_name = path.iter().last().unwrap().to_str().unwrap();

            // Generate info block
            let info = if let Some(special_info) = component.info {
                format!(
                    r#"<Alert>
                {{icon!(icondata::LuInfo)}}
                <AlertTitle>"{}"</AlertTitle>
                <AlertDescription>"{}"</AlertDescription>
            </Alert>
            "#,
                    special_info.title, special_info.description
                )
            } else {
                String::new()
            };

            // Generate examples
            let examples = if let Some(component_examples) = component.examples {
                Some(
                    component_examples
                        .par_iter()
                        .map(|example| {
                            let title = if let Some(example_title) = example.title.to_owned() {
                                Some(format!(r#"name="{}" "#, example_title))
                            } else {
                                None
                            };

                            let description =
                                if let Some(example_description) = example.description.to_owned() {
                                    Some(format!(r#"description="{}" "#, example_description))
                                } else {
                                    None
                                };

                            let path = format!(
                                "./docs/src/routes/components/{}/examples/{}.rs",
                                component_module_name,
                                string_to_snake(&example.name)
                            );

                            let example_code = highlight_html_from_file(
                                path,
                                ss.clone(),
                                theme.clone(),
                                bg.clone(),
                            );

                            format!(
                                r##"<Example {}{}view={}Example.into_any() code=r#"{}"# />
            "##,
                                title.unwrap_or_default(),
                                description.unwrap_or_default(),
                                string_to_pascal(&example.name),
                                example_code
                            )
                        })
                        .collect::<String>(),
                )
            } else {
                None
            };

            // Generate API references
            let references = component
                .references
                .par_iter()
                .map(|reference| {
                    let attrs = if reference.attrs.is_empty() {
                        None
                    } else {
                        let mut buf = reference
                            .attrs
                            .par_iter()
                            .map(|attr| {
                                format!(
                                    r##"
                            (r#"{}"#,r#"{}"#,r#"{}"#,r#"{}"#),"##,
                                    attr.attr,
                                    attr.attr_type,
                                    attr.default,
                                    attr.description.to_owned().unwrap_or_default()
                                )
                            })
                            .collect::<String>();
                        // Remove trailing comma
                        let _ = buf.pop();
                        Some(buf)
                    };

                    format!(
                        r###"
                <Reference
                    name="{}"
                    description="{}"
                    extra=r##"{}"##
                    table={}
                />
"###,
                        reference.name,
                        reference.description,
                        reference.extra.to_owned().unwrap_or_default(),
                        if let Some(attrs) = attrs {
                            format!("Some(attr_rows!({}).into_any())", attrs)
                        } else {
                            "None".into()
                        },
                    )
                })
                .collect::<String>();

            // Generate anatomy snippet
            let path = format!(
                "./docs/src/routes/components/{}/anatomy.rs",
                component_module_name
            );
            let anatomy = highlight_html_from_file(path, ss.clone(), theme.clone(), bg.clone());

            format!(
                r##"
mod {};
use {}::*;

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
                string_to_pascal(&component.name),
                string_to_title(&component.name),
                component.description,
                info,
                examples.unwrap_or_default(),
                anatomy,
                references
            )
        })
        .collect::<String>()
        .parse()
        .expect("Error parsing output")
}
