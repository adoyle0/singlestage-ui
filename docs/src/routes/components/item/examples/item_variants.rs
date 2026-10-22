use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemVariantsExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            <Item>
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Default Variant"</ItemTitle>
                    <ItemDescription>"Transparent background with no border."</ItemDescription>
                </ItemContent>
            </Item>
            <Item variant="outline">
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Outline Variant"</ItemTitle>
                    <ItemDescription>"Outlined style with a visible border."</ItemDescription>
                </ItemContent>
            </Item>
            <Item variant="muted">
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Muted Variant"</ItemTitle>
                    <ItemDescription>"Muted background for secondary content."</ItemDescription>
                </ItemContent>
            </Item>
        </div>
    }
}
