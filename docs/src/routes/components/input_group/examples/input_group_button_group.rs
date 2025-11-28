use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupButtonGroupExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-6">
            <ButtonGroup>
                <ButtonGroupText>
                    <Label label_for="url">"https://"</Label>
                </ButtonGroupText>
                <InputGroup>
                    <Input id="url" />
                    <InputGroupAddon align="inline-end">{icon!(icondata::LuLink2)}</InputGroupAddon>
                </InputGroup>
                <ButtonGroupText>".com"</ButtonGroupText>
            </ButtonGroup>
        </div>
    }
}
