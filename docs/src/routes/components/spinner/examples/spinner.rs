use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerExample() -> impl IntoView {
    view! {
        <Item class="max-w-xs" variant="muted">
            <ItemMedia>
                <Spinner />
            </ItemMedia>
            <ItemContent>
                <ItemTitle>"Processing payment..."</ItemTitle>
            </ItemContent>
            <ItemContent class="justify-end">
                <span class="tabular-nums">"$100.00"</span>
            </ItemContent>
        </Item>
    }
}
