use crate::components::CodeBlock;
use leptos::prelude::*;

#[component]
pub fn ThemeProviderRoute() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Theme Provider"</h1>

        <p class="my-4">"Wrap your app, probably at the router, with ThemeProvider:"</p>
        <CodeBlock code=r#"<pre>
<span>...</span>
<span><</span><span>Stylesheet id="leptos" href="/pkg/my_app.css" </span><span>/></span>
<span><</span>ThemeProvider</span><span>></span>
<span>    <</span><span>Router</span><span>></span>
<span>        ...</span>
<span>    <</span><span>/Router</span><span>></span>
<span><</span><span>/ThemeProvider</span><span>></span>
<span>...</span>
</pre>"#
            .to_string() />

        <p class="my-4">"Load your own stylesheets before ThemeProvider to avoid conflicts."</p>
    }
}
