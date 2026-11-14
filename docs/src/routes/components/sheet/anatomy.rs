use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SheetAnatomy() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>
                <Button />
            </SheetTrigger>
            <SheetContent>
                <SheetHeader>
                    <SheetTitle />
                    <SheetDescription />
                </SheetHeader>
                <SheetFooter>
                    <SheetClose>
                        <Button />
                    </SheetClose>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
