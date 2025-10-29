use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemLinkExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            <a href="#">
                <Item>
                    <ItemContent>
                        <ItemTitle>"Visit our documentation"</ItemTitle>
                        <ItemDescription>
                            "Learn how to get started with our components."
                        </ItemDescription>
                    </ItemContent>
                    <ItemActions>{icon!(icondata::LuChevronRight, class="size-4")}</ItemActions>
                </Item>
            </a>
            <a href="#" target="_blank" rel="noopener noreferrer">
                <Item variant="outline">
                    <ItemContent>
                        <ItemTitle>"External resource"</ItemTitle>
                        <ItemDescription>
                            "Opens in a new tab with security attributes."
                        </ItemDescription>
                    </ItemContent>
                    <ItemActions>{icon!(icondata::LuExternalLink, class="size-4")}</ItemActions>
                </Item>
            </a>
        </div>
    }
}
