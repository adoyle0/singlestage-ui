use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SidebarExampleTest() -> impl IntoView {
    view! {
        <SidebarProvider>
            <Sidebar collapsible="icon">
                <SidebarHeader>"header"</SidebarHeader>
                <SidebarContent>"Content"</SidebarContent>
                <SidebarFooter>"Footer"</SidebarFooter>
                <SidebarRail />
            </Sidebar>
            <SidebarInset>
                <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
                    <SidebarTrigger class="-ml-1">
                        <Button>"Trigger"</Button>
                    </SidebarTrigger>
                    <div class="flex items-center gap-2 px-4">"Content and stuff"</div>
                </header>
            </SidebarInset>
        </SidebarProvider>
    }
}
