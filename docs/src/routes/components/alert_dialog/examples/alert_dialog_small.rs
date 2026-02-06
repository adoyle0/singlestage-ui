use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogSmallExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <Trigger>
                <Button variant="outline">"Show Dialog"</Button>
            </Trigger>
            <DialogContent size="sm">
                <DialogHeader>
                    <DialogTitle>"Allow accessory to connect?"</DialogTitle>
                    <DialogDescription>
                        "Do you want to allow the USB accessory to connect to this device?"
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel>
                        <Button>"Don't allow"</Button>
                    </DialogCancel>
                    <DialogAction>
                        <Button>"Allow"</Button>
                    </DialogAction>
                </DialogFooter>
            </DialogContent>
        </AlertDialog>
    }
}
