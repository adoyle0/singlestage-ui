use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SidebarExample() -> impl IntoView {
    view! {
        <SidebarProvider>
            <Sidebar>
                <SidebarHeader>"Header"</SidebarHeader>
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupLabel>"Links"</SidebarGroupLabel>
                        <SidebarGroupContent>
                            <a href="#">{icon!(icondata::LuInfo)} <span>"link"</span></a>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
                <SidebarSeparator />
                <SidebarFooter>"Footer"</SidebarFooter>
            </Sidebar>

            <main>
                <header class="bg-(--background) sticky inset-x-0 top-0 isolate flex shrink-0
                items-center gap-2 border-b z-10">
                    <div class="flex h-14 w-full items-center justify-between gap-2 px-4">
                        "Main header"
                    </div>
                </header>
                <div class="mx-2 sm:mx-12 my-8">"Main content"</div>
            </main>
        </SidebarProvider>
    }
}
