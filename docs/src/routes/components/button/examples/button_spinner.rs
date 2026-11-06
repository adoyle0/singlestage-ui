use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonSpinnerExample() -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <Button variant="outline" disabled=true>
                <Spinner />
                "Generating"
            </Button>
            <Button variant="secondary" disabled=true>
                "Downloading"
                <Spinner />
            </Button>
        </div>
    }
}
