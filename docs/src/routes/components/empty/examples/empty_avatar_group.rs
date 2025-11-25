use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyAvatarGroupExample() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia>
                    <div class="flex -space-x-2
                    [&_.singlestage-avatar]:ring-(--background)
                    [&_.singlestage-avatar]:size-12
                    [&_.singlestage-avatar]:ring-2
                    [&_.singlestage-avatar]:grayscale">
                        <Avatar>
                            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="https://github.com/maxleiter.png" alt="@maxleiter" />
                        </Avatar>
                        <Avatar>
                            <AvatarImage
                                src="https://github.com/evilrabbit.png"
                                alt="@evilrabbit"
                            />
                        </Avatar>
                    </div>
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
