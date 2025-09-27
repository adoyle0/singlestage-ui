use leptos::prelude::*;
use singlestage::{alert::*, icon};

#[component]
pub fn AlertExample() -> impl IntoView {
    view! {
        <Alert>
            {icon!(icondata::LuCircleCheck)}
            <AlertTitle>"Success! Your changes have been saved"</AlertTitle>
            <AlertDescription>
                "This is an alert with icon, title and description."
            </AlertDescription>
        </Alert>
    }
}
