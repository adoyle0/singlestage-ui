use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupSelectExample() -> impl IntoView {
    let currency = RwSignal::new("$".to_string());

    view! {
        <ButtonGroup>
            <ButtonGroup>
                <Select value=currency>
                    <SelectContent label="Currency">
                        <SelectItem value="$">"$"</SelectItem>
                        <SelectItem value="€">"€"</SelectItem>
                        <SelectItem value="£">"£"</SelectItem>
                    </SelectContent>
                </Select>
                <Input placeholder="10.00" pattern="[0-9]*" />
            </ButtonGroup>
            <ButtonGroup>
                <Button attr:aria-label="Send" size="icon" variant="outline">
                    {icon!(icondata::LuArrowRight)}
                </Button>
            </ButtonGroup>
        </ButtonGroup>
    }
}
