use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogMediaExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <Trigger>
                <Button variant="outline">"Share Project"</Button>
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogMedia>{icon!(icondata::LuCircleFadingPlus)}</DialogMedia>
                    <DialogTitle>"Share this project?"</DialogTitle>
                    <DialogDescription>
                        "Anyone with the link will be able to view and edit this project."
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel>
                        <Button>"Cancel"</Button>
                    </DialogCancel>
                    <DialogAction>
                        <Button>"Share"</Button>
                    </DialogAction>
                </DialogFooter>
            </DialogContent>
        </AlertDialog>
    }
}
