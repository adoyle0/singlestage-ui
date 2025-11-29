use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyAvatarGroupExample() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia>
                    <AvatarGroup>
                        <Avatar class="size-12 grayscale">
                            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
                            <AvatarFallback>"CN"</AvatarFallback>
                        </Avatar>
                        <Avatar class="size-12 grayscale">
                            <AvatarImage src="https://github.com/maxleiter.png" alt="@maxleiter" />
                            <AvatarFallback>"LR"</AvatarFallback>
                        </Avatar>
                        <Avatar class="size-12 grayscale">
                            <AvatarImage
                                src="https://github.com/evilrabbit.png"
                                alt="@evilrabbit"
                            />
                            <AvatarFallback>"ER"</AvatarFallback>
                        </Avatar>
                    </AvatarGroup>
                </EmptyMedia>
                <EmptyTitle>"No Team Members"</EmptyTitle>
                <EmptyDescription>
                    "Invite your team to collaborate on this project."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button size="sm">{icon!(icondata::LuPlus)} "Invite Members"</Button>
            </EmptyContent>
        </Empty>
    }
}
