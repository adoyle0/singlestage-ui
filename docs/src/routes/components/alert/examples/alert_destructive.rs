use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDestructiveExample() -> impl IntoView {
    view! {
        <Alert variant="destructive" class="max-w-md">
            {icon!(icondata::FiAlertCircle)}
            <AlertTitle>"Payment failed"</AlertTitle>
            <AlertDescription>
                "Your payment could not be processed. Please check your payment method
                and try again."
            </AlertDescription>
        </Alert>
    }
}
