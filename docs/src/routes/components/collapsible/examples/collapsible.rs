use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CollapsibleExample() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Collapsible open class="flex flex-col w-full max-w-xs gap-2">
            <Item size="sm">
                <ItemContent>
                    <ItemTitle>"Order #4189"</ItemTitle>
                </ItemContent>
                <ItemActions>
                    <CollapsibleTrigger>
                        <Button variant="ghost" size="icon" class="size-8">
                            {icon!(icondata::LuChevronsUpDown)}
                            <span class="sr-only">"Toggle details"</span>
                        </Button>
                    </CollapsibleTrigger>
                </ItemActions>
            </Item>
            <Item size="sm" variant="outline">
                <ItemContent>
                    <ItemDescription>"Status"</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <span class="font-medium">"Shipped"</span>
                </ItemActions>
            </Item>
            <CollapsibleContent class="flex flex-col gap-2">
                <Item size="sm" variant="outline">
                    <ItemContent>
                        <ItemTitle>"Shipping Address"</ItemTitle>
                        <ItemDescription>"100 Market St, San Francisco"</ItemDescription>
                    </ItemContent>
                </Item>
                <Item size="sm" variant="outline">
                    <ItemContent>
                        <ItemTitle>"Items"</ItemTitle>
                        <ItemDescription>"2x Studio Headphones"</ItemDescription>
                    </ItemContent>
                </Item>
            </CollapsibleContent>
        </Collapsible>
    }
}
