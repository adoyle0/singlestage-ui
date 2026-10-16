use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipWithKeyboardShortcutExample() -> impl IntoView {
    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant="outline" size="icon-sm">
                    {icon!(icondata::LuSave)}
                </Button>
            </TooltipTrigger>
            <TooltipContent>"Save Changes" <Kbd>"S"</Kbd></TooltipContent>
        </Tooltip>
    }
}
