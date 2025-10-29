use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ItemAnatomy() -> impl IntoView {
    view! {
        <ItemGroup>
            <Item>
                <ItemMedia />
                <ItemHeader />
                <ItemContent>
                    <ItemTitle />
                    <ItemDescription />
                </ItemContent>
                <ItemActions>
                    <Button />
                </ItemActions>
                <ItemFooter />
            </Item>
            <ItemSeparator />
        </ItemGroup>
    }
}
