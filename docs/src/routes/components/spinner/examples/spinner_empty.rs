use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerEmptyExample() -> impl IntoView {
    view! {
        <Empty class="w-full">
            <EmptyHeader>
                <EmptyMedia variant="icon">
                    <Spinner />
                </EmptyMedia>
                <EmptyTitle>"Processing your request"</EmptyTitle>
                <EmptyDescription>
                    "Please wait while we process your request. Do not refresh the page."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant="outline" size="sm">
                    "Cancel"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
