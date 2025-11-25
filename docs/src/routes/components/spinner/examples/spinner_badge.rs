use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerBadgeExample() -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 [--radius:1.2rem]">
            <Badge>
                <Spinner />
                "Syncing"
            </Badge>
            <Badge variant="secondary">
                <Spinner />
                "Updating"
            </Badge>
            <Badge variant="outline">
                <Spinner />
                "Processing"
            </Badge>
        </div>
    }
}
