use super::*;
use crate::Reactive;
use leptos::prelude::*;

/// Sidebar context provider
#[component]
pub fn SidebarProvider(
    children: Children,

    /// Reactive signal coupled to the sidebar's hidden state.
    #[prop(optional, into)]
    hidden: Reactive<bool>,
    /// Reactive signal coupled to which side of its container the sidebar renders on.
    #[prop(optional, into, default = Reactive::new("left".to_string()))]
    side: Reactive<String>,
) -> impl IntoView {
    // Sent from SidebarGroupContent and SidebarMenuContent
    let close_if_small_screen = move || {
        if screen_is_small() {
            hidden.set(true)
        }
    };

    let context = SidebarContext {
        close_if_small_screen: Arc::new(close_if_small_screen),
        hidden,
        side,
    };
    provide_context(context);

    view! { {children()} }
}
