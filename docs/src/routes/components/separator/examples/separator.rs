use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SeparatorExample() -> impl IntoView {
    view! {
        <div>
            <div class="space-y-1">
                <h4 class="text-sm leading-none font-medium">"Radix Primitives"</h4>
                <p class="text-muted-foreground text-sm">"An open-source UI component library."</p>
            </div>
            <Separator class="my-4" />
            <div class="flex h-5 items-center space-x-4 text-sm">
                <div>"Blog"</div>
                <Separator vertical=true />
                <div>"Docs"</div>
                <Separator vertical=true />
                <div>"Source"</div>
            </div>
        </div>
    }
}
