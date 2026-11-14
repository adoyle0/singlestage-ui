use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyAnatomy() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia />
                <EmptyTitle />
                <EmptyDescription />
            </EmptyHeader>
            <EmptyContent />
        </Empty>
    }
}
