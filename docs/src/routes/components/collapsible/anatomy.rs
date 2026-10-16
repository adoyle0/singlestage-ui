use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CollapsibleAnatomy() -> impl IntoView {
    view! {
        <Collapsible>
            <CollapsibleTrigger>
                <Button />
            </CollapsibleTrigger>
            <CollapsibleContent />
        </CollapsibleTrigger>
    }
}
