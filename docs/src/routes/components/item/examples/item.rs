use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            <Item variant="outline">
                <ItemContent>
                    <ItemTitle>"Basic Item"</ItemTitle>
                    <ItemDescription>"A simple item with title and description."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant="outline" size="sm">
                        "Action"
                    </Button>
                </ItemActions>
            </Item>
            <a href="#">
                <Item variant="outline" size="sm">
                    <ItemMedia>{icon!(icondata::LuBadgeCheck, class="size-5")}</ItemMedia>
                    <ItemContent class="block">
                        <ItemTitle>"Your profile has been verified."</ItemTitle>
                    </ItemContent>
                    <ItemActions>{icon!(icondata::LuChevronRight, class="size-4")}</ItemActions>
                </Item>
            </a>
        </div>
    }
}
