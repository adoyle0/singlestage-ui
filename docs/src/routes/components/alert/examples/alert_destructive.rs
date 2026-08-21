use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDestructiveExample() -> impl IntoView {
    view! {
        <Alert class="max-w-md" variant="destructive">
            {icon!(icondata::FiAlertCircle)}
            <AlertTitle>"Payment failed"</AlertTitle>
            <AlertDescription>
                "Your payment could not be processed. Please check your payment method
                and try again."
            </AlertDescription>
        </Alert>
    }
}
