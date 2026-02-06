use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogDestructiveExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <Trigger>
                <Button variant="destructive">"Delete Chat"</Button>
            </Trigger>
            <DialogContent size="sm">
                <DialogHeader>
                    <DialogMedia variant="destructive">{icon!(icondata::LuTrash2)}</DialogMedia>
                    <DialogTitle>"Delete chat?"</DialogTitle>
                    <DialogDescription>
                        "This will permanently delete this chat conversation. View "
                        <a href="#">"Settings"</a>
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogCancel>
                        <Button variant="ghost">"Cancel"</Button>
                    </DialogCancel>
                    <DialogAction>
                        <Button variant="destructive">"Delete"</Button>
                    </DialogAction>
                </DialogFooter>
            </DialogContent>
        </AlertDialog>
    }
}
