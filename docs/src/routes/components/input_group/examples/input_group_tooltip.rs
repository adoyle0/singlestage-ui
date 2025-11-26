use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupTooltipExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <InputGroupInput placeholder="Enter password" input_type="password" />
                <InputGroupAddon align="inline-end">
                    <Tooltip value="Password must be at least 8 characters">
                        <InputGroupButton variant="ghost" attr:aria-label="Info" size="icon-xs">
                            {icon!(icondata::LuInfo)}
                        </InputGroupButton>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="Your email address" />
                <InputGroupAddon align="inline-end">
                    <Tooltip value="We'll use this to send you notifications">
                        <InputGroupButton variant="ghost" attr:aria-label="Help" size="icon-xs">
                            {icon!(icondata::FiHelpCircle)}
                        </InputGroupButton>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="Enter API key" />
                <Tooltip value="Click for help with API keys">
                    <InputGroupAddon>
                        <InputGroupButton variant="ghost" attr:aria-label="Help" size="icon-xs">
                            {icon!(icondata::FiHelpCircle)}
                        </InputGroupButton>
                    </InputGroupAddon>
                </Tooltip>
            </InputGroup>
        </div>
    }
}
