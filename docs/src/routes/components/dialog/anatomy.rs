use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogAnatomy -> impl IntoView {
    view!{
        <Dialog>
            <Trigger>
                <Button />
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle />
                    <DialogDescription />
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel />
                    <DialogAction />
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
