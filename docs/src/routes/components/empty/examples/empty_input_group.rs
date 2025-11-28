use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyInputGroupExample() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyTitle>"404 - Not Found"</EmptyTitle>
                <EmptyDescription>
                    "The page you're looking for doesn't exist. Try searching for
                    what you need below."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <InputGroup class="sm:w-3/4">
                    <Input placeholder="Try searching for pages..." />
                    <InputGroupAddon>{icon!(icondata::LuSearch)}</InputGroupAddon>
                    <InputGroupAddon align="inline-end">
                        <Kbd>"/"</Kbd>
                    </InputGroupAddon>
                </InputGroup>
                <EmptyDescription>"Need help? " <a href="#">"Contact support"</a></EmptyDescription>
            </EmptyContent>
        </Empty>
    }
}
