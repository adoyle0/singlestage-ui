use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogAnatomy -> impl IntoView {
    view!{
        <Dialog>
            <DialogTrigger>
                <Button />
            </DialogTrigger>
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
