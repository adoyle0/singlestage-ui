use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn InputGroupIconExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-sm gap-6">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>{icon!(icondata::LuSearch)}</InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput input_type="email" placeholder="Enter your email" />
                <InputGroupAddon>{icon!(icondata::LuMail)}</InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon>{icon!(icondata::LuCreditCard)}</InputGroupAddon>
                <InputGroupAddon align="inline-end">{icon!(icondata::LuCheck)}</InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon align="inline-end">
                    {icon!(icondata::LuStar)} {icon!(icondata::LuInfo)}
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
