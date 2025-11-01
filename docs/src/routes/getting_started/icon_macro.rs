// use crate::components::CodeBlock;
// use crate::components::Reference;
use crate::components::*;
use leptos::prelude::*;
use singlestage::*;

macro_rules! attr_row {
    ($prop:expr, $note:expr) => {
        view! {
            <TableRow>
                <TableCell>
                    <Command>
                        $prop
                    </Command>
                </TableCell>
                <TableCell>
                    <p>
                        $note
                    </p>
                </TableCell>
            </TableRow>
        }
    };
}

#[component]
pub fn IconMacroRoute() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-s/emibold">"SVG Icon Macro"</h1>
        <p class="my-4">
            "For users of icondata, Singlestage includes a macro to make using icons a bit easier. We use this macro internally and although it's not officially part of the library, we're happy to provide it to you. The ids for these icons, as well as a search function, can be found on "
            <Link href="https://carloskiki.github.io/icondata/">"Carloskiki's Icondata"</Link>
            " page."
        </p>
        <p class="my-4">
            "More information regarding the usage of SVG, and its attributes, can be found "
            <Link href="https://developer.mozilla.org/en-US/docs/Web/SVG">here</Link> "."
        </p>
        <p class="my-4">"Basic usage:"</p>
        <CodeBlock code=r#"<pre>
<span>...</span>
<span><</span><span>A href="/icon-macro"</span><span>></span>
    {icon!(icondata::LuImage)} <span>"Icon Macro"</span>
<span><</span><span>/A</span><span>></span>
<span>...</span>
</pre>"#
            .to_string() />
        <p class="my-4">"Advanced usage:"</p>
        <CodeBlock code=r#"<pre>
<span>...</span>
<span><</span><span>A href="/icon-macro"</span><span>></span>
    {icon!(icondata::LuImage, width=100, class="stroke-1", style="stroke:red")} <span>"Icon Macro"</span>
<span><</span><span>/A</span><span>></span>
<span>...</span>
</pre>"#
            .to_string() />
        <p class="my-4">
            "In both cases the first argument is required, and must be an icondata id."
        </p>
        <p class="my-4">
            "The icon macro will accept, in no particular order, any or none of the following arguments:"
        </p>
        <Table class="mt-4">
            <TableHeader>
                <TableRow>
                    <TableHead>"Name"</TableHead>
                    <TableHead>"Notes"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {attr_row!("width", "Default : 100%")} {attr_row!("height", "Default: 100%")}
                {attr_row!("stroke_linecap", "Notice the underscore")}
                {attr_row!("stroke_linejoin", "Notice the underscore")}
                {attr_row!("stroke_width", "Notice the underscore")} {attr_row!("stroke", "")}
                {attr_row!("fill", "")} {attr_row!("style", "")} {attr_row!("class", "")}
            </TableBody>
        </Table>
    }
}
