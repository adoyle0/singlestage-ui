use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertExample() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-md items-start gap-4">
            <Alert>
                {icon!(icondata::LuCircleCheck)} <AlertTitle>"Payment successful"</AlertTitle>
                <AlertDescription>
                    "Your payment of $29.99 has been processed. A receipt has been sent to
                    your email address."
                </AlertDescription>
            </Alert>
            <Alert>
                {icon!(icondata::LuInfo)} <AlertTitle>"New feature available"</AlertTitle>
                <AlertDescription>
                    "We've added dark mode support. You can enable it in your account
                    settings."
                </AlertDescription>
            </Alert>
        </div>
    }
}
