use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyOutlineExample() -> impl IntoView {
    view! {
        <Empty class="border border-dashed">
            <EmptyHeader>
                <EmptyMedia variant="icon">{icon!(icondata::LuCloud)}</EmptyMedia>
                <EmptyTitle>"Cloud Storage Empty"</EmptyTitle>
                <EmptyDescription>
                    "Upload files to your cloud storage to access them anywhere."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant="outline" size="sm">
                    "Upload Files"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
