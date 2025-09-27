use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonSizesExample() -> impl IntoView {
    view! {
        <div class="space-x-2 sm:space-x-12">
            <Button size="small" variant="primary">
                "Small"
            </Button>
            <Button size="normal" variant="primary">
                "Normal"
            </Button>
            <Button size="large" variant="primary">
                "Large"
            </Button>
        </div>
    }
}
