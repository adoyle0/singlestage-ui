use leptos::prelude::*;
use singlestage::dialog::*;

#[component]
pub fn DialogAnatomy -> impl IntoView {
    view!{
        <Dialog>
            <Trigger />
            <DialogHeader>
                <DialogTitle />
                <DialogDescription />
            </DialogHeader>
            <DialogContent/>
            <DialogFooter />
            <DialogClose/>
        </Dialog>
    }
}
