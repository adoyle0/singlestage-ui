use crate::{SidebarContext, Tooltip, TooltipContent, TooltipTrigger};
use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct SidebarMenuButtonContext {}

#[component]
pub fn SidebarMenuButton(
    children: Children,
    #[prop(optional, into)] tooltip: MaybeProp<String>,
) -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Provider value=SidebarMenuButtonContext {}>
            <Tooltip>
                <TooltipTrigger>{children()}</TooltipTrigger>
                <Show when=move || !sidebar.open.get() && tooltip.get().is_some()>
                    <TooltipContent side=match sidebar.side.get().as_str() {
                        "right" => "left",
                        _ => "right",
                    }>
                        <p>{move || tooltip.get().unwrap_or_default()}</p>
                    </TooltipContent>
                </Show>
            </Tooltip>
        </Provider>
    }
}
