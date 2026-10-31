use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertActionExample() -> impl IntoView {
    view! {
        <Alert class="max-w-md">
            <AlertTitle>"Dark mode is now available"</AlertTitle>
            <AlertDescription>
                "Enable it under your profile settings to get started."
            </AlertDescription>
            <AlertAction>
                <Button size="xs" variant="default">
                    "Enable"
                </Button>
            </AlertAction>
        </Alert>
    }
}
