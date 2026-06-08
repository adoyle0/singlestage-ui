use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertBasicExample() -> impl IntoView {
    view! {
        <Alert class="max-w-md">
            {icon!(icondata::BsCheckCircle)} <AlertTitle>"Account updated successfully"</AlertTitle>
            <AlertDescription>
                "Your profile information has been saved. Changes will be reflected
                immediately."
            </AlertDescription>
        </Alert>
    }
}
