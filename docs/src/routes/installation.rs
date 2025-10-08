use crate::components::CodeBlock;
use leptos::prelude::*;

#[component]
pub fn Installation() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Install"</h1>

        <p class="my-4">"Add singlestage with cargo:"</p>
        <CodeBlock code="<pre><span>cargo add singlestage</span></pre>".to_string() />

        <p class="my-4">"Or add singlestage to your Cargo.toml:"</p>
        <CodeBlock code=r#"<pre>
<span># Cargo.toml</span>
<span></span>
<span>[dependencies]</span>
<span>leptos = "0.8"</span>
<span>...</span>
<span>singlestage = "0.2"</span>
</pre>"#
            .to_string() />

        <h2 class="my-6 text-2xl font-semibold">"Nightly"</h2>
        <p class="my-4">
            "Add the nightly feature to singlestage just as you do with Leptos, if using nightly."
        </p>
        <CodeBlock code=r#"<pre>
<span># Cargo.toml</span>
<span></span>
<span>[dependencies]</span>
<span>leptos = { version = "0.8", features = ["nightly"] }</span>
<span>...</span>
<span>singlestage = {version = "0.2", features = ["nightly"] }</span>
</pre>"#
            .to_string() />

        <h2 class="my-6 text-2xl font-semibold">"SSR"</h2>
        <p class="my-4">"Nothing special needs to happen whether you you use SSR or not."</p>
    }
}
