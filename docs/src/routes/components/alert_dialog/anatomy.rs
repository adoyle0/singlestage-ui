use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogAnatomy -> impl IntoView {
    view!{
        <AlertDialog>
            <Trigger>
                <Button />
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogMedia />
                    <DialogTitle />
                    <DialogDescription />
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel />
                    <DialogAction />
                </DialogFooter>
            </DialogContent>
        </AlertDialog>
    }
}
