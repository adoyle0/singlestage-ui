use leptos::prelude::*;
use singlestage::kbd::*;

#[component]
pub fn KbdExample() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-4">
            <KbdGroup>
                <Kbd>"⌘"</Kbd>
                <Kbd>"⇧"</Kbd>
                <Kbd>"⌥"</Kbd>
                <Kbd>"⌃"</Kbd>
            </KbdGroup>
            <KbdGroup>
                <Kbd>"Ctrl"</Kbd>
                <span>"+"</span>
                <Kbd>"B"</Kbd>
            </KbdGroup>
        </div>
    }
}
