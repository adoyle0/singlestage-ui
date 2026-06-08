use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugMenu() -> impl IntoView {
    {
        if cfg!(debug_assertions) {
            view! {
                <SidebarGroup>
                    <SidebarGroupLabel>"Debug"</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuItem>
                                <Collapsible>
                                    <CollapsibleTrigger>
                                        <SidebarMenuButton>
                                            <Button>{icon!(icondata::LuSettings)} "Test Menu"</Button>
                                        </SidebarMenuButton>
                                    </CollapsibleTrigger>
                                    <CollapsibleContent>
                                        <SidebarMenuSub>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuButton>
                                                    <Link href="#">
                                                        <span>"One"</span>
                                                    </Link>
                                                </SidebarMenuButton>
                                            </SidebarMenuSubItem>
                                        </SidebarMenuSub>
                                        <SidebarMenuSub>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuButton>
                                                    <Link href="#">
                                                        <span>"Two"</span>
                                                    </Link>
                                                </SidebarMenuButton>
                                            </SidebarMenuSubItem>
                                        </SidebarMenuSub>
                                        <SidebarMenuSub>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuButton>
                                                    <Link href="#">
                                                        <span>"Three"</span>
                                                    </Link>
                                                </SidebarMenuButton>
                                            </SidebarMenuSubItem>
                                        </SidebarMenuSub>
                                        <SidebarMenuSub>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuButton>
                                                    <Link href="#">
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
            .into_any()
        } else {
            ().into_any()
        }
    }
}
