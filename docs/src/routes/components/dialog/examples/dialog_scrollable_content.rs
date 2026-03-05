use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogScrollableContentExample() -> impl IntoView {
    view! {
        <Dialog>
            <Trigger>
                <Button variant="outline">"Scrollable Content"</Button>
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Scrollable Content"</DialogTitle>
                    <DialogDescription>
                        "This is a dialog with scrollable content."
                    </DialogDescription>
                </DialogHeader>
                <div class="no-scrollbar -mx-4 max-h-[50vh] overflow-y-auto px-4">
                    <For each=move || 0..10 key=|i| i.to_string() let(_)>
                        <p class="mb-4 leading-normal">
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
                            eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
                            enim ad minim veniam, quis nostrud exercitation ullamco laboris
                            nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
                            reprehenderit in voluptate velit esse cillum dolore eu fugiat
                            nulla pariatur. Excepteur sint occaecat cupidatat non proident,
                            sunt in culpa qui officia deserunt mollit anim id est laborum."
                        </p>
                    </For>
                </div>
            </DialogContent>
        </Dialog>
    }
}
