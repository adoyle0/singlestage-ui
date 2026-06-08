use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemSizeExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            <Item variant="outline">
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Default Size"</ItemTitle>
                    <ItemDescription>"The standard size for most use cases."</ItemDescription>
                </ItemContent>
            </Item>
            <Item variant="outline" size="sm">
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Small Size"</ItemTitle>
                    <ItemDescription>"A compact size for dense layouts."</ItemDescription>
                </ItemContent>
            </Item>
            <Item variant="outline" size="xs">
                <ItemMedia variant="icon">{icon!(icondata::LuInbox)}</ItemMedia>
                <ItemContent>
                    <ItemTitle>"Extra Small Size"</ItemTitle>
                    <ItemDescription>"The most compact size available."</ItemDescription>
                </ItemContent>
            </Item>
        </div>
    }
}
