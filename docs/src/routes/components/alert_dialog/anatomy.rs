use leptos::prelude::*;
use singlestage::dialog::*;

#[component]
pub fn DialogAnatomy -> impl IntoView {
    view!{
        <Dialog alert=true>
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
