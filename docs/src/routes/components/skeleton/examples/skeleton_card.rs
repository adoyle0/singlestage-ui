use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SkeletonCardExample() -> impl IntoView {
    view! {
        <div class="flex flex-col space-y-3">
            <Skeleton class="h-[125px] w-[250px] rounded-xl" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px] rounded-md" />
                <Skeleton class="h-4 w-[200px] rounded-md" />
            </div>
        </div>
    }
}
