use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonWithIconExample() -> impl IntoView {
    view! {
        <Button variant="outline" size="sm">
            {icon!(icondata::LuGitBranch)}
            <span>"New Branch"</span>
        </Button>
    }
}
