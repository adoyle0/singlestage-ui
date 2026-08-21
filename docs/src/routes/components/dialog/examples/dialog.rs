use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <Dialog class="sm:max-w-sm">
            <DialogTrigger>
                <Button variant="outline">"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent>
                <form>
                    <DialogHeader>
                        <DialogTitle>"Edit profile"</DialogTitle>
                        <DialogDescription>
                            "Make changes to your profile here. Click save when you're
                            done."
                        </DialogDescription>
                    </DialogHeader>
                    <FieldGroup>
                        <Field>
                            <Input value="Pedro Duarte">"Name"</Input>
                        </Field>
                        <Field>
                            <Input value="@peduarte">"Username"</Input>
                        </Field>
                    </FieldGroup>
                    <DialogFooter>
                        <DialogClose>
                            <Button>"Cancel"</Button>
                        </DialogClose>
                        <Button>"Save changes"</Button>
                    </DialogFooter>
                </form>
            </DialogContent>
        </Dialog>
    }
}
