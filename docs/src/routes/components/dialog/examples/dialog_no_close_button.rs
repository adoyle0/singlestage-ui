use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogNoCloseButtonExample() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>
                <Button variant="outline">"No Close Button"</Button>
            </DialogTrigger>
            <DialogContent close_button=false>
                <DialogHeader>
                    <DialogTitle>"No Close Button"</DialogTitle>
                    <DialogDescription>
                        "This dialog doesn't have a close button in the top-right
                        corner."
                    </DialogDescription>
                </DialogHeader>
            </DialogContent>
        </Dialog>
    }
}
