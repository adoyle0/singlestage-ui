use crate::components::CodeBlock;
use leptos::prelude::*;
use singlestage::*;

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

        <h2 class="mt-6 text-2xl font-semibold">"Setting the Default Theme and Mode"</h2>
        <p class="my-4">"By default, the theme provider expands to look like this:"</p>

        <CodeBlock code=r#"<pre>
<span><</span><span>ThemeProvider mode="auto" theme=Theme::Default></span>
</pre>"#
            .to_string() />

        <p class="my-4">"ThemeProvider currently has three modes:"</p>
        <ul class="singlestage-ulist">
            <li>"auto"</li>
            <li>"dark"</li>
            <li>"light"</li>
        </ul>

        <p class="my-4">
            "These modes correspond to parts of the theme, in this case Theme::Default.
            Each theme has three parts which are just a collection of CSS variables that override
            the main template:"
        </p>

        <ol class="singlestage-olist">
            <li>"common"</li>
            <li>"dark"</li>
            <li>"light"</li>
        </ol>

        <p class="my-4">
            "Common contains variables common to both light and dark mode, dark contains variables
            only used for dark mode, and light contains variables only used for light mode. You can
            create your own arbitrary Theme using this format and introduce it here. This is also 
            where you can include any arbitrary CSS that you want to load reactively."
        </p>

        <h2 class="mt-6 text-2xl font-semibold">"Updating the Theme and Mode"</h2>
        <p class="my-4">
            "ThemeProvider also provides ThemeProviderContext to its children. Here is an example
            showing how to update the current Mode:"
        </p>

        <CodeBlock code=r#"<pre>
<span>#[component]</span>
<span>pub fn ThemeModeSwitcher() -> impl IntoView {</span>
<span>    let theme_context = expect_context::<</span><span>ThemeProviderContext>();</span>
<span>    let theme_mode = RwSignal::new("auto".to_string());</span>
<span></span>
<span>    Effect::new(move || theme_context.set(theme_mode.get().into()));</span>
<span></span>
<span>    view!{</span>
<span>        <</span><span>RadioGroup value=theme_mode></span>
<span>            <</span><span>legend>"Theme Mode"<</span><span>/legend></span>
<span>            <</span><span>Radio value="auto">"Auto"<</span><span>/Radio></span>
<span>            <</span><span>Radio value="dark">"Dark"<</span><span>/Radio></span>
<span>            <</span><span>Radio value="light">"Light"<</span><span>/Radio></span>
<span>        <</span><span>/RadioGroup></span>
<span>    }</span>
<span>}</span>
</pre>"#
            .to_string() />

        <p class="my-4">
            "For a more in-depth example of how to use the theme system, including cookies, check
            out the source code for "
            <Link href="https://github.com/adoyle0/singlestage-ui/blob/main/docs/src/components/theme_switcher.rs">
                "this docs website"
            </Link>"."
        </p>

        <h2 class="mt-6 text-2xl font-semibold">"Dark Themes and Non-SSR Render Modes"</h2>
        <p class="my-4">
            "Note that if you're doing client-side rendering with trunk, and using a dark theme,
            some browsers may briefly flash a default background (typically white) on page reload
            breaking any dark themes. This is something we can't do anything about at library
            level, but we can do something at app level. A simple workaround is to force a
            background color with CSS in index.html:"
        </p>
        <CodeBlock code=r#"<pre>
<span><</span><span>!-- index.html --></span>
<span>...</span>
<span><</span><span>head></span>
<span>...</span>
<span><</span><span>style></span>
<span>  @media (prefers-color-scheme: dark) {</span>
<span>    :root {</span>
<span>      background-color: #0a0a0a;</span>
<span>    }</span>
<span>  }</span>
<span><</span><span>/style></span>
<span>...</span>
<span><</span><span>/head></span>
<span>...</span>
</pre>"#
            .to_string() />
        <p class="my-4">
            "This isn't magic however, you'll now have the opposite problem, and possibly more
            problems if you have a light/dark toggle feature in your app. When doing CSR you'll
            just have to choose to either deal with the flashing or have only a light or dark theme."
        </p>

        <p class="my-4">
            "Islands has this same issue but it cannot be solved. The background will flash white
            on every page navigation or reload because it triggers a full repaint."
        </p>
    }
}
