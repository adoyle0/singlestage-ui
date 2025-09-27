use leptos::prelude::*;
use singlestage::{button::*, dialog::*, input::*, label::*};

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger slot>
                <Button variant="outline">"Edit Profile"</Button>
            </DialogTrigger>

            <DialogClose />

            <DialogHeader>
                <DialogTitle>"Edit profile"</DialogTitle>
                <DialogDescription>
                    "Make changes to your profile here. Click save when you're done."
                </DialogDescription>
            </DialogHeader>

            <DialogContent>
                <form class="grid gap-4">
                    <div class="grid gap-3">
                        <Label label_for="demo-dialog-edit-profile-name">"Name"</Label>
                        <Input
                            value=RwSignal::new("Pedro Duarte".to_string())
                            id="demo-dialog-edit-profile-name"
                            autofocus=true
                        />
                    </div>
                    <div class="grid gap-3">
                        <Label label_for="demo-dialog-edit-profile-username">"Username"</Label>
                        <Input
                            value=RwSignal::new("@peduarte".to_string())
                            id="demo-dialog-edit-profile-username"
                        />
                    </div>
                </form>
            </DialogContent>

            <DialogFooter>
                <Button variant="outline">"Cancel"</Button>
                <Button>"Save changes"</Button>
            </DialogFooter>

        </Dialog>
    }
}
