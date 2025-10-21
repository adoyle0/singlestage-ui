use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn Introduction() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Introduction"</h1>
        <p class="mt-4">
            "Singlestage takes the familiar styling of "
            <Link href="https://radix-ui.com">"Radix"</Link>" and "
            <Link href="https://ui.shadcn.com">"shadcn/ui"</Link>
            " and brings it to Leptos. It takes a modular approach, meaning every component is
            feature flagged so you can take as much or as little as you need."
        </p>

        // accordion?
        <p class="mt-4">
            "Each component tries to stay lean, using semantic HTML elements, modern APIs, and CSS
            instead of Rust code wherever possible. This keeps WASM binaries small while staying
            current with emerging accessibility features."
        </p>

        <h2 class="mt-6 text-2xl font-semibold">"Features"</h2>
        <ul class="singlestage-ulist mt-4">
            <li>"Conforms to modern web standards"</li>
            <li>"WAI-ARIA compliant"</li>
            <li>"Focused on minimalism, modularity, and performance"</li>
            <li>"Feature flagged components"</li>
            <li>"Modular theme system"</li>
            <li>"Dark mode that just works"</li>
            <li>"Arbitrary theme support"</li>
            <li>"HTML-like API"</li>
        </ul>

        <h2 class="mt-6 text-2xl font-semibold">"Contributing"</h2>
        <p class="mt-4">"Contributions are welcome."</p>
    }
}
