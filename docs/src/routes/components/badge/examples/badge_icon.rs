use icondata::FiAlertCircle;
use leptos::prelude::*;
use singlestage::{badge::*, icon};

#[component]
pub fn BadgeIconExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <Badge>{icon!(FiAlertCircle)} "Primary"</Badge>
            <Badge variant="secondary">{icon!(FiAlertCircle)} "Secondary"</Badge>
            <Badge variant="destructive">{icon!(FiAlertCircle)}"Destructive"</Badge>
            <Badge variant="outline">{icon!(FiAlertCircle)} "Outline"</Badge>
        </div>
    }
}
