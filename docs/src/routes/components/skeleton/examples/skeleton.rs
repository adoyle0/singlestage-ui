use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SkeletonExample() -> impl IntoView {
    view! {
        <div class="flex items-center space-x-4">
            <Skeleton class="h-12 w-12 rounded-full" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px] rounded-md" />
                <Skeleton class="h-4 w-[200px] rounded-md" />
            </div>
        </div>
    }
}
