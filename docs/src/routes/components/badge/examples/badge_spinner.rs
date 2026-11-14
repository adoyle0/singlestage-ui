use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BadgeSpinnerExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge variant="destructive">
                <Spinner />
                <span>"Deleting"</span>
            </Badge>
            <Badge variant="secondary">
                <span>"Generating"</span>
                <Spinner />
            </Badge>
        </div>
    }
}
