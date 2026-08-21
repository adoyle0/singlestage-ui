use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn SidebarButton() -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant="ghost" size="sm-icon">
                    <Show
                        when=move || { !sidebar.open.get() }
                        fallback=move || match sidebar.side.get().as_str() {
                            "right" => view! { {icon!(icondata::LuPanelRightClose)} }.into_any(),
                            _ => view! { {icon!(icondata::LuPanelLeftClose)} }.into_any(),
                        }
                    >
                        {match sidebar.side.get().as_str() {
                            "right" => view! { {icon!(icondata::LuPanelRightOpen)} }.into_any(),
                            _ => view! { {icon!(icondata::LuPanelLeftOpen)} }.into_any(),
                        }}
                    </Show>
                </Button>
            </TooltipTrigger>
            <TooltipContent side="bottom" align="start">
                <p>"Toggle sidebar"</p>
            </TooltipContent>
        </Tooltip>
    }
}
