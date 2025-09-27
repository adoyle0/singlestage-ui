use leptos::prelude::*;
use singlestage::dialog::*;

#[component]
pub fn DialogAnatomy -> impl IntoView {
    view!{
        <Dialog>
            <DialogTrigger slot />
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
