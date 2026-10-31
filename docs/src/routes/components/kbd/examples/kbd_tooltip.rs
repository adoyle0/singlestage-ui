use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn KbdTooltipExample() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4">
            <ButtonGroup>
                <Tooltip>
                    <TooltipTrigger>
                        <Button variant="outline">"Save"</Button>
                    </TooltipTrigger>
                    <TooltipContent>"Save Changes "<Kbd>"S"</Kbd></TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger>
                        <Button variant="outline">"Print"</Button>
                    </TooltipTrigger>
                    <TooltipContent>
                        "Print Document "<KbdGroup>
                            <Kbd>"Ctrl"</Kbd>
                            <Kbd>"P"</Kbd>
                        </KbdGroup>
                    </TooltipContent>
                </Tooltip>
            </ButtonGroup>
        </div>
    }
}
