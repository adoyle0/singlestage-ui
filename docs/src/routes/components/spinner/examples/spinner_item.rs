use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SpinnerItemExample() -> impl IntoView {
    view! {
        <div class="flex w-full max-w-md flex-col gap-4 [--radius:1rem]">
            <Item variant="outline">
                <ItemMedia variant="icon">
                    <Spinner />
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Downloading..."</ItemTitle>
                    <ItemDescription>"129 MB / 1000 MB"</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <Button variant="outline" size="sm">
                        "Cancel"
                    </Button>
                </ItemActions>
                <ItemFooter>
                    <Progress max=1000 value=129 />
                </ItemFooter>
            </Item>
        </div>
    }
}
