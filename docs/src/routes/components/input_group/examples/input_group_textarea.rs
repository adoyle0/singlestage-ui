use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupTextareaExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-md gap-4">
            <InputGroup>
                <Textarea
                    id="textarea-code-32"
                    placeholder="console.log('Hello, world!');"
                    class="min-h-[200px]"
                />
                <InputGroupAddon align="block-end" class="border-t">
                    <InputGroupText>"Line 1, Column 1"</InputGroupText>
                    <Button size="sm" class="ml-auto" variant="default">
                        "Run"
                        {icon!(icondata::FiCornerDownLeft)}
                    </Button>
                </InputGroupAddon>
                <InputGroupAddon align="block-start" class="border-b">
                    <InputGroupText class="font-mono font-medium">
                        {icon!(icondata::BiJavascript)} "script.js"
                    </InputGroupText>
                    <Button class="ml-auto" size="icon-xs">
                        {icon!(icondata::LuRefreshCw)}
                    </Button>
                    <Button variant="ghost" size="icon-xs">
                        {icon!(icondata::LuCopy)}
                    </Button>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
