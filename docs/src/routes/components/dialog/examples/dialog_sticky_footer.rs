use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogStickyFooterExample() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>
                <Button variant="outline">"Sticky Footer"</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Sticky Footer"</DialogTitle>
                    <DialogDescription>
                        "This dialog has a sticky footer that stays visible while the content
                        scrolls."
                    </DialogDescription>
                </DialogHeader>
                <div class="no-scrollbar -mx-4 max-h-[50vh] overflow-y-auto px-4">
                    <For each=move || 0..10 key=|i| i.to_string() let(_)>
                        <p class="mb-4 leading-normal"></p>
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
                        eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
                        enim ad minim veniam, quis nostrud exercitation ullamco laboris
                        nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
                        reprehenderit in voluptate velit esse cillum dolore eu fugiat
                        nulla pariatur. Excepteur sint occaecat cupidatat non proident,
                        sunt in culpa qui officia deserunt mollit anim id est laborum."
                    </For>
                </div>
                <DialogFooter>
                    <DialogClose>
                        <Button variant="outline">"Close"</Button>
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
