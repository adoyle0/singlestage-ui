use icondata::LuMenu;
use leptos::prelude::*;
use singlestage::{icon, Button};

#[component]
pub fn ButtonWithIconExample() -> impl IntoView {
    view! {
        <div class="space-x-2 sm:space-x-12">
            <Button size="small" variant="primary">
                {icon!(LuMenu)}
                "Menu"
            </Button>
            <Button variant="primary">{icon!(LuMenu)} "Menu"</Button>
            <Button size="large" variant="primary">
                {icon!(LuMenu)}
                "Menu"
            </Button>
        </div>
    }
}
