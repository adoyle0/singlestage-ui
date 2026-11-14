#[cfg(feature = "tooltip")]
use crate::{Tooltip, TooltipContent, TooltipTrigger};

use crate::SidebarContext;
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

    #[cfg(feature = "tooltip")]
    if tooltip.get_untracked().is_some() {
        return view! {
            <Provider value=SidebarMenuButtonContext {}>
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
            </Provider>
        }
        .into_any();
    }

    return view! { <Provider value=SidebarMenuButtonContext {}>{children()}</Provider> }
        .into_any();
}
