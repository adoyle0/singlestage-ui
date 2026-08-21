use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupTooltipExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <Input placeholder="Enter password" input_type="password" />
                <InputGroupAddon align="inline-end">
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant="ghost" aria_label="Info" size="icon-xs">
                                {icon!(icondata::LuInfo)}
                            </Button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <p>"Password must be at least 8 characters"</p>
                        </TooltipContent>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Your email address" />
                <InputGroupAddon align="inline-end">
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant="ghost" aria_label="Help" size="icon-xs">
                                {icon!(icondata::FiHelpCircle)}
                            </Button>
                        </TooltipTrigger>
                        <TooltipContent>
                            <p>"We'll use this to send you notifications"</p>
                        </TooltipContent>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupAddon>
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant="ghost" aria_label="Help" size="icon-xs">
                                {icon!(icondata::FiHelpCircle)}
                            </Button>
                        </TooltipTrigger>
                        <TooltipContent side="left">
                            <p>"Click for help with API keys"</p>
                        </TooltipContent>
                    </Tooltip>
                </InputGroupAddon>
                <Input placeholder="Enter API key" />
            </InputGroup>
        </div>
    }
}
