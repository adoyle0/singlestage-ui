use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn BadgeIconExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge variant="secondary">{icon!(icondata::LuCheck)}<span>Verified</span></Badge>
            <Badge variant="outline">
                <span>Bookmark</span>
                {icon!(icondata::LuBookmark)}
            </Badge>
        </div>
    }
}
