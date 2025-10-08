use crate::components::*;
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

        <h2 class="my-6 text-2xl font-semibold">"Tailwind BYOB"</h2>
        <p class="my-4">
            "Singlestage uses tailwind during the build process. It first checks for "
            <Command>"tailwindcss"</Command> " in "<Command>"PATH"</Command> " (system install,
            typical for Linux). If that doesn't work then it tries to download the latest tailwind
            binary from their github release page. If for some reason this fails, or for whatever
            reason you want to use your own tailwind executable, you can bring your own binary by
            setting the " <Command>"SINGLESTAGE_TAILWIND_PATH"</Command> " environment variable to
            the full path (from root) to your tailwind binary. Note that if you download the binary
            from github on Linux or MacOS then you'll probably have to make it executable
            ("<Command>"chmod +x"</Command>")."
        </p>

        <p>"Example:"</p>
        <CodeBlock code=r#"<pre>
<span>SINGLESTAGE_TAILWIND_PATH=/path/to/tailwindcss cargo leptos watch</span>
</pre>"#
            .to_string() />
    }
}
