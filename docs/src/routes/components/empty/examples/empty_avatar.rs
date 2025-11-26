use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyAvatarExample() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant="default">
                    <Avatar class="size-12">
                        <AvatarImage src="https://github.com/shadcn.png" class="grayscale" />
                    </Avatar>
                </EmptyMedia>
                <EmptyTitle>"User Offline"</EmptyTitle>
                <EmptyDescription>
                    "This user is currently offline. You can leave a message to notify them
                    or try again later."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button size="sm">"Leave Message"</Button>
            </EmptyContent>
        </Empty>
    }
}
