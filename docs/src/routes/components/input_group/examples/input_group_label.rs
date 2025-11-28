use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupLabelExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-4">
            <InputGroup>
                <Input id="email" placeholder="shadcn" />
                <InputGroupAddon>
                    <Label label_for="email">"@"</Label>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <Input id="email-2" placeholder="shadcn@vercel.com" />
                <InputGroupAddon align="block-start">
                    <Label label_for="email-2" class="text-(--foreground)">
                        "Email"
                    </Label>
                    <Tooltip class="ml-auto" value="We'll use this to send you notifications">
                        <Button
                            variant="ghost"
                            aria_label="Help"
                            class="rounded-full"
                            size="icon-xs"
                        >
                            {icon!(icondata::LuInfo)}
                        </Button>
                    </Tooltip>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
