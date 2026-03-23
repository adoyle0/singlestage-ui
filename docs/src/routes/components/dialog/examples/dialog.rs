use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>
                <Button variant="outline">"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent class="sm:max-w-sm">
                <DialogHeader>
                    <DialogTitle>"Edit profile"</DialogTitle>
                    <DialogDescription>
                        "Make changes to your profile here. Click save when you're
                        done."
                    </DialogDescription>
                </DialogHeader>
                <form>
                    <FieldGroup>
                        <Field>
                            <FieldLabel label_for="name-1">"Name"</FieldLabel>
                            <Input id="name-1" name="name" value="Pedro Duarte" />
                        </Field>
                        <Field>
                            <FieldLabel label_for="username-1">"Username"</FieldLabel>
                            <Input id="username-1" name="username" value="@peduarte" />
                        </Field>
                    </FieldGroup>
                </form>
                <DialogFooter>
                    <DialogCancel>
                        <Button>"Cancel"</Button>
                    </DialogCancel>
                    <DialogAction>
                        <Button button_type="submit">"Save changes"</Button>
                    </DialogAction>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
