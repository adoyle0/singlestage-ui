use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupSelectExample() -> impl IntoView {
    let currency = RwSignal::new("$".to_string());

    view! {
        <ButtonGroup>
            <ButtonGroup>
                <Select value=currency>
                    <SelectOptGroup label="Currency">
                        <SelectOption value="$">"$"</SelectOption>
                        <SelectOption value="€">"€"</SelectOption>
                        <SelectOption value="£">"£"</SelectOption>
                    </SelectOptGroup>
                </Select>
                <Input placeholder="10.00" pattern="[0-9]*" />
            </ButtonGroup>
            <ButtonGroup>
                <Button aria_label="Send" size="icon" variant="outline">
                    {icon!(icondata::LuArrowRight)}
                </Button>
            </ButtonGroup>
        </ButtonGroup>
    }
}
