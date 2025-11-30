use leptos::prelude::*;
use singlestage::badge::*;

#[component]
pub fn BadgeInvalidExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Badge invalid=true>"Primary"</Badge>
            <Badge invalid=true variant="secondary">
                "Secondary"
            </Badge>
            <Badge invalid=true variant="destructive">
                "Destructive"
            </Badge>
            <Badge invalid=true variant="outline">
                "Outline"
            </Badge>
        </div>
    }
}
