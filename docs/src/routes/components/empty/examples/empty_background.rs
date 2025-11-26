use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyBackgroundExample() -> impl IntoView {
    view! {
        <Empty class="from-(--muted)/50 to-(--background) h-full bg-gradient-to-b from-30%">
            <EmptyHeader>
                <EmptyMedia variant="icon">{icon!(icondata::LuBell)}</EmptyMedia>
                <EmptyTitle>"No Notifications"</EmptyTitle>
                <EmptyDescription>
                    "You're all caught up. New notifications will appear here."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant="outline" size="sm">
                    {icon!(icondata::LuRefreshCcw)}
                    "Refresh"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
