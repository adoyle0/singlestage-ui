use leptos::prelude::*;

#[component]
pub fn CodeBlock(code: String, #[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <div
            class=move || {
                format!(
                    "p-4 border border-(--muted) rounded-md
                    bg-(--muted) overflow-scroll text-sm sm:text-base {}",
                    class.get(),
                )
            }
            inner_html=code
        ></div>
    }
}
