use icondata::LuArrowRight;
use leptos::prelude::*;
use singlestage::{badge::*, icon};

#[component]
pub fn BadgeLinkExample() -> impl IntoView {
    view! {
        <div class="space-x-2">
            <a href="#">
                <Badge>"Link" {icon!(LuArrowRight)}</Badge>
            </a>
            <a href="#">
                <Badge variant="secondary">"Link" {icon!(LuArrowRight)}</Badge>
            </a>
            <a href="#">
                <Badge variant="destructive">"Link" {icon!(LuArrowRight)}</Badge>
            </a>
            <a href="#">
                <Badge variant="outline">"Link" {icon!(LuArrowRight)}</Badge>
            </a>
        </div>
    }
}
