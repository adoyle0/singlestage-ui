use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogCustomCloseButtonExample() -> impl IntoView {
    view! {
        <Dialog class="sm:max-w-md">
            <DialogTrigger>
                <Button variant="outline">"Share"</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Share link"</DialogTitle>
                    <DialogDescription>
                        "Anyone who has this link will be able to view this."
                    </DialogDescription>
                </DialogHeader>
                <Label label_for="link" class="sr-only">
                    "Link"
                </Label>
                <Input id="link" value="https://singlestage.doordesk.net/install" readonly=true />
                <DialogFooter class="sm:justify-start">
                    <DialogClose>
                        <Button>"Close"</Button>
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
