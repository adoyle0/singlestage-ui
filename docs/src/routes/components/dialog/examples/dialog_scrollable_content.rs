use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogScrollableContentExample() -> impl IntoView {
    let data: [&'static str; 10] = core::array::from_fn(|_| {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
        enim ad minim veniam, quis nostrud exercitation ullamco laboris
        nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
        reprehenderit in voluptate velit esse cillum dolore eu fugiat
        nulla pariatur. Excepteur sint occaecat cupidatat non proident,
        sunt in culpa qui officia deserunt mollit anim id est laborum."
    });

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
                    <For each=move || data key=|_| "42" let(data)>
                        <p class="mb-4 leading-normal">{data}</p>
                    </For>
                </div>
            </DialogContent>
        </Dialog>
    }
}
