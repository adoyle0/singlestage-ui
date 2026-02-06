use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogBasicExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <Trigger>
                <Button variant="outline">"Show Dialog"</Button>
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Are you absolutely sure?"</DialogTitle>
                    <DialogDescription>
                        "This action cannot be undone. This will permanently delete your
                        account and remove your data from our servers."
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel>
                        <Button>"Cancel"</Button>
                    </DialogCancel>
                    <DialogAction>
                        <Button>"Continue"</Button>
                    </DialogAction>
                </DialogFooter>
            </DialogContent>
        </AlertDialog>
    }
}
