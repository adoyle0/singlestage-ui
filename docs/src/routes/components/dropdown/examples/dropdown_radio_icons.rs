use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DropdownRadioIconsExample() -> impl IntoView {
    let payment_method = RwSignal::new("card".to_string());
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Payment Method"</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="min-w-56">
                <DropdownMenuGroup>
                    <DropdownMenuLabel>"Select Payment Method"</DropdownMenuLabel>
                    <RadioGroup value=payment_method>
                        <Radio value="card">{icon!(icondata::LuCreditCard)} "Credit Card"</Radio>
                        <Radio value="paypal">{icon!(icondata::LuWallet)} "PayPal"</Radio>
                        <Radio value="bank">{icon!(icondata::LuBuilding2)} "Bank Transfer"</Radio>
                    </RadioGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
