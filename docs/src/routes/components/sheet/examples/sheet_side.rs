use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SheetSideExample() -> impl IntoView {
    let sheet_sides: [&'static str; 4] = ["top", "right", "bottom", "left"];

    view! {
        <div class="flex flex-wrap gap-2">
            <For each=move || sheet_sides.to_owned() key=|side| side.to_string() let(side)>
                <Sheet>
                    <SheetTrigger>
                        <Button variant="outline" class="capitalize">
                            {side}
                        </Button>
                    </SheetTrigger>
                    <SheetContent
                        side
                        class=match side {
                            "top" | "bottom" => "max-h-[50vh]",
                            _ => "",
                        }
                    >
                        <SheetHeader>
                            <SheetTitle>"Edit profile"</SheetTitle>
                            <SheetDescription>
                                "Make changes to your profile here. Click save when you're
                                done."
                            </SheetDescription>
                        </SheetHeader>
                        <div class="no-scrollbar overflow-y-auto px-4">

                            <For each=move || 0..10 key=|i| i.to_string() let(_)>
                                <p class="mb-2 leading-relaxed">
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed
                                    do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                    Ut enim ad minim veniam, quis nostrud exercitation ullamco
                                    laboris nisi ut aliquip ex ea commodo consequat. Duis aute
                                    irure dolor in reprehenderit in voluptate velit esse cillum
                                    dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                                    cupidatat non proident, sunt in culpa qui officia deserunt
                                    mollit anim id est laborum."
                                </p>
                            </For>
                        </div>
                        <SheetFooter>
                            <Button>"Save changes"</Button>
                            <SheetClose>
                                <Button variant="outline">"Cancel"</Button>
                            </SheetClose>
                        </SheetFooter>
                    </SheetContent>
                </Sheet>
            </For>
        </div>
    }
}
