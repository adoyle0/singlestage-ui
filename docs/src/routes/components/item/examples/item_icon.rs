use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemIconExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-lg flex-col gap-6">
            <Item variant="outline">
                <ItemMedia variant="icon">{icon!(icondata::LuShieldAlert)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Security Alert"</ItemTitle>
                    <ItemDescription>"New login detected from unknown device."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button size="sm" variant="outline">
                        "Review"
                    </Button>
                </ItemActions>
            </Item>
        </div>
    }
}
