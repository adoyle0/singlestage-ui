use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SheetExample() -> impl IntoView {
    view! {
        <Sheet>
            <SheetTrigger>
                <Button variant="outline">"Open"</Button>
            </SheetTrigger>
            <SheetContent>
                <SheetHeader>
                    <SheetTitle>"Edit profile"</SheetTitle>
                    <SheetDescription>
                        "Make changes to your profile here. Click save when you're done."
                    </SheetDescription>
                </SheetHeader>
                <div class="grid flex-1 auto-rows-min gap-6 px-4">
                    <div class="grid gap-3">
                        <Label label_for="sheet-demo-name">"Name"</Label>
                        <Input id="sheet-demo-name" value="Pedro Duarte" />
                    </div>
                    <div class="grid gap-3">
                        <Label label_for="sheet-demo-username">"Username"</Label>
                        <Input id="sheet-demo-username" value="@peduarte" />
                    </div>
                </div>
                <SheetFooter>
                    <Button>"Save changes"</Button>
                    <SheetClose>
                        <Button variant="outline">"Close"</Button>
                    </SheetClose>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
}
