use leptos::prelude::*;
use singlestage::{Separator, scroll_area::*};

#[component]
pub fn ScrollAreaExample() -> impl IntoView {
    let versions = RwSignal::new((0..13).collect::<Vec<usize>>());

    view! {
        <ScrollArea class="h-72 w-48 rounded-md border" orientation="horizontal">
            <div class="p-4">
                <h4 class="mb-4 text-sm leading-none font-medium">Tags</h4>
                <For
                    each=move || versions.get()
                    key=|version| version.clone()
                    children=move |version| {
                        view! {
                            <div class="text-sm">"v1.2.0-beta."{version.to_string()}</div>
                            <Separator class="my-2" />
                        }
                    }
                />
            </div>
        // <ScrollAreaScrollbar orientation="horizontal" />
        </ScrollArea>
    }
}
