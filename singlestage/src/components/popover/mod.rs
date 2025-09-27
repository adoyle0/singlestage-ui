// TODO: use popover api like dropdown menu

use leptos::prelude::*;

#[slot]
pub struct PopoverTrigger {
    children: ChildrenFn,
}

#[derive(Clone)]
pub struct PopoverContext {
    hidden: RwSignal<bool>,
}

#[component]
pub fn Popover(
    children: Children,
    popover_trigger: PopoverTrigger,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
) -> impl IntoView {
    let hidden = RwSignal::new(false);

    let context = PopoverContext { hidden };
    provide_context(context);

    view! {
        <div on:click=move |_| hidden.set(false)>{(popover_trigger.children)().into_any()}</div>
        <div id=move || id.get() class=move || format!("singlestage-popover {}", &class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverContent(
    children: Children,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
) -> impl IntoView {
    let popover = expect_context::<PopoverContext>();

    view! {
        <div
            id=move || id.get()
            data-popover
            aria-hidden=move || popover.hidden.get().to_string()
            class=move || class.get()
        >
            {children()}
        </div>
    }
}
