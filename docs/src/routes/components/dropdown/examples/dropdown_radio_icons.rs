use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownRadioIconsExample() -> impl IntoView {
    let payment_method = RwSignal::new("card".to_string());
    view! {
        <DropdownMenu>
            <Trigger>
                <Button variant="outline">"Payment Method"</Button>
            </Trigger>
            <MenuContent class="min-w-56">
                <MenuGroup>
                    <Label>"Select Payment Method"</Label>
                    <RadioGroup value=payment_method>
                        <RadioItem value="card">
                            {icon!(icondata::LuCreditCard)} "Credit Card"
                        </RadioItem>
                        <RadioItem value="paypal">{icon!(icondata::LuWallet)} "PayPal"</RadioItem>
                        <RadioItem value="bank">
                            {icon!(icondata::LuBuilding2)} "Bank Transfer"
                        </RadioItem>
                    </RadioGroup>
                </MenuGroup>
            </MenuContent>
        </DropdownMenu>
    }
}
