use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SidebarAnatomy() -> impl IntoView {
    view! {
        // Left side
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
            <SidebarInset>
                <SidebarTrigger />
            </SidebarInset>
        </SidebarProvider>

        // Right side
        <SidebarProvider side="right">
            <SidebarInset>
                <SidebarTrigger />
            </SidebarInset>
            <Sidebar>
                <SidebarRail />
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
            </Sidebar>
        </SidebarProvider>
    }
}
