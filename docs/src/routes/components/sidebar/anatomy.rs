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
                                        <CollapsibleTrigger>
                                            <SidebarMenuButton />
                                        </CollapsibleTrigger>
                                        <CollapsibleContent>
                                            <SidebarMenuSub>
                                                <SidebarMenuSubItem>
                                                    <SidebarMenuSubButton />
                                                </SidebarMenuSubItem>
                                            </SidebarMenuSub>
                                        </CollapsibleContent>
                                    </Collapsible>
                                </SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
                <SidebarFooter />
                <SidebarRail />
            </Sidebar>
            <main>
                <SidebarTrigger />
            </main>
        </SidebarProvider>
    }
}
