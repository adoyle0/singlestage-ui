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
                <FieldSet class="px-4">
                    <Field>
                        <Input default="Pedro Duarte">"Name"</Input>
                    </Field>
                    <Field>
                        <Input default="@peduarte">"Username"</Input>
                    </Field>
                </FieldSet>
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
