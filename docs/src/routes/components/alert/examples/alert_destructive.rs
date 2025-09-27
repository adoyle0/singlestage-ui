use leptos::prelude::*;
use singlestage::{alert::*, icon};

#[component]
pub fn AlertDestructiveExample() -> impl IntoView {
    view! {
        <Alert variant="destructive">
            {icon!(icondata::FiAlertCircle)} <AlertTitle>"Something went wrong!"</AlertTitle>
            <AlertDescription>"Your session has expired. Please log in again."</AlertDescription>
        </Alert>
    }
}
