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
                    <DropdownMenuRadioGroup value=payment_method>
                        <DropdownMenuRadioItem value="card">
                            {icon!(icondata::LuCreditCard)} "Credit Card"
                        </DropdownMenuRadioItem>
                        <DropdownMenuRadioItem value="paypal">
                            {icon!(icondata::LuWallet)} "PayPal"
                        </DropdownMenuRadioItem>
                        <DropdownMenuRadioItem value="bank">
                            {icon!(icondata::LuBuilding2)} "Bank Transfer"
                        </DropdownMenuRadioItem>
                    </DropdownMenuRadioGroup>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
