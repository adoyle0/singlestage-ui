use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TestMenu() -> impl IntoView {
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"Sidebar Test"</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <Collapsible>
                            <CollapsibleTrigger>
                                <SidebarMenuButton>
                                    <Button>{icon!(icondata::LuMenu)} "Test Menu"</Button>
                                </SidebarMenuButton>
                            </CollapsibleTrigger>
                            <CollapsibleContent>
                                <SidebarMenuSub>
                                    <SidebarMenuSubItem>
                                        <SidebarMenuButton>
                                            <Link href="/1">
                                                <span>"One"</span>
                                            </Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuSubItem>
                                </SidebarMenuSub>
                                <SidebarMenuSub>
                                    <SidebarMenuSubItem>
                                        <SidebarMenuButton>
                                            <Link href="/2">
                                                <span>"Two"</span>
                                            </Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuSubItem>
                                </SidebarMenuSub>
                                <SidebarMenuSub>
                                    <SidebarMenuSubItem>
                                        <SidebarMenuButton>
                                            <Link href="/3">
                                                <span>"Three"</span>
                                            </Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuSubItem>
                                </SidebarMenuSub>
                                <SidebarMenuSub>
                                    <SidebarMenuSubItem>
                                        <SidebarMenuButton>
                                            <Link href="/4">
                                                <span>"Four"</span>
                                            </Link>
                                        </SidebarMenuButton>
                                    </SidebarMenuSubItem>
                                </SidebarMenuSub>
                            </CollapsibleContent>
                        </Collapsible>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    }
}
