use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogCustomCloseButtonExample() -> impl IntoView {
    view! {
        <Dialog>
            <Trigger>
                <Button variant="outline">"Share"</Button>
            </Trigger>
            <DialogContent class="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle>"Share link"</DialogTitle>
                    <DialogDescription>
                        "Anyone who has this link will be able to view this."
                    </DialogDescription>
                </DialogHeader>
                <div class="flex items-center gap-2">
                    <div class="grid flex-1 gap-2">
                        <Label label_for="link" class="sr-only">
                            "Link"
                        </Label>
                        <Input
                            id="link"
                            value="https://singlestage.doordesk.net/install"
                            readonly=true
                        />
                    </div>
                </div>
                <DialogFooter class="sm:justify-start">
                    <DialogCancel>
                        <Button button_type="button">"Close"</Button>
                    </DialogCancel>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
