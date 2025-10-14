use leptos::prelude::*;
use singlestage::{input::*, label::*};

#[component]
pub fn InputExample() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Input input_type="color" default="#F00">
                "Color:"
            </Input>
            <Input input_type="date">"Date:"</Input>
            <Input input_type="datetime-local">"Datetime:"</Input>
            <Input input_type="email">"Email:"</Input>
            <Input input_type="file">"File:"</Input>
            <Input input_type="hidden" name="John Cena" />
            <Input input_type="month">"Month:"</Input>
            <Input input_type="number">"Number:"</Input>
            <Input input_type="password">"Password:"</Input>
            <Input input_type="search">"Search:"</Input>
            <Input input_type="tel">"Phone Number:"</Input>
            <Input input_type="text" disabled=true>
                "Disabled:"
            </Input>
            <Input input_type="text" invalid=true>
                "Invalid:"
            </Input>
            <Input input_type="time">"Time:"</Input>
            <Input input_type="url">"URL:"</Input>
            <Input input_type="week">"Week:"</Input>
        </div>
    }
}
