use leptos::prelude::*;
use singlestage::{button::*, dialog::*};

#[component]
pub fn DialogAlertExample() -> impl IntoView {
    view! {
        <Dialog alert=true>
            <DialogTrigger slot>
                <Button variant="destructive">"Delete account"</Button>
            </DialogTrigger>
            <DialogHeader>
                <DialogTitle>"Are you absolutely sure?"</DialogTitle>
                <DialogDescription>
                    "This action cannot be undone.
                    This will permanently delete your account 
                    and remove your data from our servers."
                </DialogDescription>
            </DialogHeader>
            <DialogFooter>
                <Button variant="outline">"Cancel"</Button>
                <Button variant="destructive">"Yes, delete account"</Button>
            </DialogFooter>
        </Dialog>
    }
}
