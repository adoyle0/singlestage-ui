use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemAvatarExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-lg flex-col gap-6">
            <Item variant="outline">
                <ItemMedia>
                    <Avatar class="size-10">
                        <AvatarImage src="https://github.com/evilrabbit.png" />
                        <AvatarFallback>"ER"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Evil Rabbit"</ItemTitle>
                    <ItemDescription>"Last seen 5 months ago"</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button
                        size="icon-sm"
                        variant="outline"
                        class="rounded-full"
                        aria_label="Invite"
                    >
                        {icon!(icondata::LuPlus)}
                    </Button>
                </ItemActions>
            </Item>
            <Item variant="outline">
                <ItemMedia>
                    <div class="*:ring-(--background) flex -space-x-2 *:ring-2 *:grayscale">
                        <Avatar>
                            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
                            <AvatarFallback>"CN"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="https://github.com/maxleiter.png" alt="@maxleiter" />
                            <AvatarFallback>"LR"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage
                                src="https://github.com/evilrabbit.png"
                                alt="@evilrabbit"
                            />
                            <AvatarFallback>"ER"</AvatarFallback>
                        </Avatar>
                    </div>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"No Team Members"</ItemTitle>
                    <ItemDescription>
                        "Invite your team to collaborate on this project."
                    </ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button size="sm" variant="outline">
                        "Invite"
                    </Button>
                </ItemActions>
            </Item>
        </div>
    }
}
