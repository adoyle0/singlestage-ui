use crate::components::CodeBlock;
use leptos::prelude::*;

#[component]
pub fn IconMacroRoute() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Icon Macro"</h1>
        <p class="my-4">"Singlestage includes a macro to make using icons a bit easier:"</p>
        <CodeBlock code=r#"<pre>
<span>...</span>
<span><</span><span>A href="/icon-macro"</span><span>></span>
    {icon!(icondata::LuImage)} <span>"Icon Macro"</span>
<span><</span><span>/A</span><span>></span>
<span>...</span>
</pre>"#
            .to_string() />
    }
}
