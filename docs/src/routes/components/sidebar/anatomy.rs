use leptos::prelude::*;
use singlestage::Sidebar::*;

#[component]
pub fn SidebarAnatomy() -> impl IntoView {
    view! {
        <SidebarProvider>
            <Sidebar>
                <SidebarHeader />
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupLabel />
                        <SidebarGroupContent>
                            <SidebarMenu>
                                <SidebarMenuItem>
                                    <SidebarMenuButton />
                                </SidebarMenuItem>
                                <SidebarMenuItem>
                                    <Collapsible>
                                        <CollapsibleTrigger slot>
                                            <SidebarMenuButton />
                                        </CollapsibleTrigger>
                                        <CollapsibleContent>
                                            <SidebarMenuSub>
                                                <SidebarMenuSubItem />
                                            </SidebarMenuSub>
                                        </CollapsibleContent>
                                    </Collapsible>
                                </SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
                <SidebarFooter />
            </Sidebar>
            <main>
                <SidebarTrigger />
            </main>
        </SidebarProvider>
    }
}
