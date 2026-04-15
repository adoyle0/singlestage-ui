use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleExample() -> impl IntoView {
    let pressed = RwSignal::new(false);

    view! {
        <Toggle aria_label="Toggle bookmark" pressed size="sm" variant="outline">
            <Show
                when=move || pressed.get()
                fallback=move || view! { {icon!(icondata::LuBookmark)} }
            >
                {icon!(icondata::LuBookmark, class="fill-(--foreground)")}
            </Show>
            Bookmark
        </Toggle>
    }
}
