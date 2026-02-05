use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <Dialog>
            <Trigger>
                <Button variant="outline">"Open Dialog"</Button>
            </Trigger>
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
                            <Label label_for="name-1">Name</Label>
                            <Input id="name-1" name="name" value="Pedro Duarte" />
                        </Field>
                        <Field>
                            <Label label_for="username-1">Username</Label>
                            <Input id="username-1" name="username" value="@peduarte" />
                        </Field>
                    </FieldGroup>
                </form>
                <DialogFooter>
                    <DialogClose>
                        <Button variant="outline">"Cancel"</Button>
                    </DialogClose>
                    <Button button_type="submit">"Save changes"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
