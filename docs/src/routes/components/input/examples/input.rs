use leptos::prelude::*;
use singlestage::{input::*, label::*};

#[component]
pub fn InputExample() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Label class="grid gap-2">"Color:"<Input input_type="color" default="#F00" /></Label>
            <Label class="grid gap-2">"Date:"<Input input_type="date" /></Label>
            <Label class="grid gap-2">"Datetime:"<Input input_type="datetime-local" /></Label>
            <Label class="grid gap-2">"Email:"<Input input_type="email" /></Label>
            <Label class="grid gap-2">"File:"<Input input_type="file" /></Label>
            <Input input_type="hidden" name="John Cena" />
            <Label class="grid gap-2">"Month:"<Input input_type="month" /></Label>
            <Label class="grid gap-2">"Number:"<Input input_type="number" /></Label>
            <Label class="grid gap-2">"Password:"<Input input_type="password" /></Label>
            <Label class="grid gap-2">"Search:"<Input input_type="search" /></Label>
            <Label class="grid gap-2">"Phone Number:"<Input input_type="tel" /></Label>
            <Label class="grid gap-2">"Disabled:"<Input input_type="text" disabled=true /></Label>
            <Label class="grid gap-2">"Invalid:"<Input input_type="text" invalid=true /></Label>
            <Label class="grid gap-2">"Time:"<Input input_type="time" /></Label>
            <Label class="grid gap-2">"URL:"<Input input_type="url" /></Label>
            <Label class="grid gap-2">"Week:"<Input input_type="week" /></Label>
        </div>
    }
}
