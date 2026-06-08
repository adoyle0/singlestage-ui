use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SheetNoCloseButtonExample() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>
                <Button variant="outline">"Open Sheet"</Button>
            </SheetTrigger>
            <SheetContent show_close_button=false>
                <SheetHeader>
                    <SheetTitle>"No Close Button"</SheetTitle>
                    <SheetDescription>
                        "This sheet doesn't have a close button in the top-right corner.
                        Click outside to close."
                    </SheetDescription>
                </SheetHeader>
            </SheetContent>
        </Sheet>
    }
}
