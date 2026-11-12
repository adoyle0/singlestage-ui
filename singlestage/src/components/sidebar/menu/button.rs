use crate::{SidebarContext, Tooltip, TooltipContent, TooltipTrigger};
use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct SidebarMenuButtonContext {}

/// Wraps content displayed in the click area of a SidebarMenuItem.
#[component]
pub fn SidebarMenuButton(
    children: Children,
    #[prop(optional, into)] tooltip: MaybeProp<String>,
) -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Provider value=SidebarMenuButtonContext {}>
            {if tooltip.get_untracked().is_some() {

                view! {
                    <Tooltip>
                        <TooltipTrigger>{children()}</TooltipTrigger>
                        <TooltipContent
                            style:display=move || match sidebar.open.get() {
                                true => Some("none"),
                                false => None,
                            }
                            align="center"
                            side=sidebar.side
                            sidebar_button=true
                        >
                            <p>{tooltip}</p>
                        </TooltipContent>
                    </Tooltip>
                }
                    .into_any()
            } else {
                children().into_any()
            }}
        </Provider>
    }
}
