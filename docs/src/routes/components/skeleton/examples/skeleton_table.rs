use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SkeletonTableExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-sm flex-col gap-2">
            <For each=move || 0..5 key=|i| i.to_string() let(_)>
                <div class="flex gap-4">
                    <Skeleton class="h-4 flex-1" />
                    <Skeleton class="h-4 w-24" />
                    <Skeleton class="h-4 w-20" />
                </div>
            </For>
        </div>
    }
}
