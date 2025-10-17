use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToggleExample() -> impl IntoView {
    let x = RwSignal::new(false);
    view! {
            <div>
                <Toggle
                    variant="default"
                    title="Toggle bold text"
                    attr:aria-label="Toggle bold text"
                    pressed=x
                >
                    <span class="font-semibold leading-none">"B"</span>
                </Toggle>
                <p on:click=move |_| x.set(!x.get_untracked())> {move || if x.get() {"On"} else {"Off"}} </p>
            </div>
    }
}
