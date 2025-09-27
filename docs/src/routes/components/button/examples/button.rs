use leptos::prelude::*;
use singlestage::button::*;

#[component]
pub fn ButtonExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Button>"Primary"</Button>
            <Button variant="secondary">"Secondary"</Button>
            <Button variant="outline">"Outline"</Button>
            <Button variant="ghost">"Ghost"</Button>
            <Button variant="link">"Link"</Button>
            <Button variant="destructive">"Destructive"</Button>
        </div>
    }
}
