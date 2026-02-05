use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogExample() -> impl IntoView {
    view! {
        <Dialog alert=true>
            <Trigger>
                <Button variant="outline">"Show Dialog"</Button>
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Are you absolutely sure?"</DialogTitle>
                    <DialogDescription>
                        "This action cannot be undone. This will permanently delete your
                        account from our servers."
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel>"Cancel"</DialogCancel>
                    <DialogAction>"Continue"</DialogAction>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
