use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupTooltipExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <Input placeholder="Enter password" input_type="password" />
                <InputGroupAddon align="inline-end">
                    <Tooltip value="Password must be at least 8 characters">
                        <Button variant="ghost" aria_label="Info" size="icon-xs">
                            {icon!(icondata::LuInfo)}
                        </Button>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input placeholder="Your email address" />
                <InputGroupAddon align="inline-end">
                    <Tooltip value="We'll use this to send you notifications">
                        <Button variant="ghost" aria_label="Help" size="icon-xs">
                            {icon!(icondata::FiHelpCircle)}
                        </Button>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Tooltip side="left" value="Click for help with API keys">
                    <InputGroupAddon>
                        <Button variant="ghost" aria_label="Help" size="icon-xs">
                            {icon!(icondata::FiHelpCircle)}
                        </Button>
                    </InputGroupAddon>
                </Tooltip>
                <Input placeholder="Enter API key" />
            </InputGroup>
        </div>
    }
}
