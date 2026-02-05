use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DialogStickyFooterExample() -> impl IntoView {
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
                <Button variant="outline">"Sticky Footer"</Button>
            </Trigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Sticky Footer"</DialogTitle>
                    <DialogDescription>
                        "This dialog has a sticky footer that stays visible while the content
                        scrolls."
                    </DialogDescription>
                </DialogHeader>
                <div class="no-scrollbar -mx-4 max-h-[50vh] overflow-y-auto px-4">
                    <For each=move || data key=|_| "42" let(data)>
                        <p class="mb-4 leading-normal"></p>
                        {data}
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
