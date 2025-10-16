use crate::components::*;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn Command(children: Children) -> impl IntoView {
    view! { <span class="py-1 px-2 shadow-sm rounded-sm bg-(--muted) font-mono">{children()}</span> }
}

#[macro_export]
macro_rules! attr_rows {
    ( $( ($prop:expr, $type:expr, $default:expr, $description:expr) ),* ) => {
        {
            let mut rows = vec![];
            $(rows.push(
                view! {
                    <TableRow>
                        <TableCell>
                            <Command>
                                $prop
                            </Command>
                        </TableCell>
                        <TableCell>
                            <Command>
                                $type
                            </Command>
                        </TableCell>
                        <TableCell>
                            <Command>
                                $default
                            </Command>
                        </TableCell>
                        <TableCell>
                            <p>
                                $description
                            </p>
                        </TableCell>
                    </TableRow>
                });
            )*
            rows.collect_view()
        }
    };
}

#[component]
pub fn Example(
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] description: String,
    #[prop(into)] code: String,
    #[prop(into)] view: AnyView,
) -> impl IntoView {
    view! {
        <div class="my-12">
            {if !name.is_empty() {
                view! { <h1 class="text-2xl font-semibold">{name}</h1> }.into_any()
            } else {
                "".into_any()
            }}
            {if !description.is_empty() {
                view! { <p class="text-(--muted-foreground) pt-3" inner_html=description></p> }
                    .into_any()
            } else {
                "".into_any()
            }} <Tabs class="my-4">
                <TabsList>
                    <TabsTrigger value="preview">"Preview"</TabsTrigger>
                    <TabsTrigger value="code">"Code"</TabsTrigger>
                </TabsList>

                <TabsContent value="preview">
                    <div class="py-12 px-4 sm:px-12 border border-(--muted) rounded-md flex justify-center">
                        {view}
                    </div>
                </TabsContent>

                <TabsContent value="code">
                    <CodeBlock code=code />
                </TabsContent>
            </Tabs>
        </div>
    }
}

#[component]
pub fn Anatomy(#[prop(into)] code: String) -> impl IntoView {
    view! {
        <div class="my-12">
            <h2 class="text-2xl font-semibold">"Anatomy"</h2>
            <p class="pt-3">"Import all parts and piece them together."</p>

            <CodeBlock code=code />
        </div>
    }
}

#[component]
pub fn Reference(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
    #[prop(into)] extra: String,
    #[prop(into)] table: Option<AnyView>,
) -> impl IntoView {
    view! {
        <div class="my-12">
            <h3 class="text-xl font-semibold">
                <Command>{name}</Command>
            </h3>
            <p class="mt-4 mb-2">{description}</p>
            {if table.is_some() {
                view! {
                    <Table class="mt-4">
                        <TableCaption>"Custom Attributes"</TableCaption>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Name"</TableHead>
                                <TableHead>"Type"</TableHead>
                                <TableHead>"Default"</TableHead>
                                <TableHead>"Description"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>{table}</TableBody>
                    </Table>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
            <div class="text-sm" inner_html=extra></div>
        </div>
    }
}

#[component]
pub fn API(children: Children) -> impl IntoView {
    view! {
        <h2 class="my-12 text-2xl font-semibold">"API Reference"</h2>
        {children()}
    }
}

#[component]
pub fn ComponentTemplate(
    children: Children,
    #[prop(into)] name: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="max-w-3xl m-auto">
            <h1 class="text-4xl font-semibold">{name}</h1>
            <p class="my-5 text-(--muted-foreground)">{description}</p>

            {children()}

        </div>
    }
}
