use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerButtonExample() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-4">
            <Button disabled=true size="sm">
                <Spinner />
                "Loading..."
            </Button>
            <Button variant="outline" disabled=true size="sm">
                <Spinner />
                "Please wait"
            </Button>
            <Button variant="secondary" disabled=true size="sm">
                <Spinner />
                "Processing"
            </Button>
        </div>
    }
}
