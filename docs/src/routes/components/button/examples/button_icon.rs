use icondata::LuMenu;
use leptos::prelude::*;
use singlestage::{icon, Button};

#[component]
pub fn ButtonIconExample() -> impl IntoView {
    view! {
        <div class="space-x-2 sm:space-x-12">
            <Button size="sm-icon" variant="primary">
                {icon!(LuMenu)}
            </Button>
            <Button size="icon" variant="primary">
                {icon!(LuMenu)}
            </Button>
            <Button size="lg-icon" variant="primary">
                {icon!(LuMenu)}
            </Button>
        </div>
    }
}
