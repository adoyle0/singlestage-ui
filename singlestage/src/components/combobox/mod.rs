// use crate::ComboBoxContext;
use leptos::prelude::*;

#[derive(Clone)]
pub struct ComboBoxContext {
    combo_box_id: RwSignal<String>,
    hidden: RwSignal<bool>,
}

#[component]
pub fn ComboBox(children: Children) -> impl IntoView {
    let combo_box_id = RwSignal::new(String::new());
    let hidden = RwSignal::new(true);

    let context = ComboBoxContext {
        combo_box_id,
        hidden,
    };

    provide_context(context);

    view! {
        <div id=move || combo_box_id.get() class="singlestage-select">
            {children()}
        </div>
    }
}
