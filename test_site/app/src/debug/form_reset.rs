use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugFormReset() -> impl IntoView {
    let input_value = RwSignal::new(String::from("default value"));
    let input_default = RwSignal::new(String::from("default value"));
    let text_value = RwSignal::new(String::from("default value"));
    let text_default = RwSignal::new(String::from("default value"));

    let update_text = move |_| {
        input_value.set(String::from("updated value"));
        text_value.set(String::from("updated value"));
    };

    let reset_text = move |_| {
        input_default.set(String::from("reset value"));
        text_default.set(String::from("reset value"));
    };

    view! {
        <h1 class="text-4xl font-semibold">"Form Reset"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>
                "Controlled form elements should reset to their initial value set at page load.
                Unless the default was reset, in which case they reset to the new value."
            </li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Input Value: "{move || input_value.get().to_string()}</li>
            <li>"Input Default Value: "{move || input_default.get().to_string()}</li>
            <li>"Textarea Value: "{move || text_value.get().to_string()}</li>
            <li>"Textarea Default Value: "{move || text_default.get().to_string()}</li>
        </ul>

        <form class="space-y-3" style="width: 540px;">

            <Input value=input_value default=input_default>
                "Controlled Input"
            </Input>

            <Label label_for="html-input">"HTML Input"</Label>
            <input
                id="html-input"
                type="text"
                class="singlestage-input"
                value=move || input_default.get_untracked()
                prop:value=move || input_value.get()
            />

            <Textarea value=text_value default=text_default>
                "Controlled Textarea"
            </Textarea>

            <Label label_for="html-textarea">"HTML Textarea"</Label>
            <textarea
                class="singlestage-textarea"
                id="html-textarea"
                prop:value=move || text_value.get()
            >
                {move || text_default.get_untracked()}
            </textarea>

            <span class="space-x-2">
                <Button variant="outline" button_type="button" on:click=update_text>
                    "Controlled Update Text (value)"
                </Button>
                <Button variant="outline" button_type="button" on:click=reset_text>
                    "Controlled Reset Text (default)"
                </Button>
                <Button variant="outline" button_type="reset">
                    "HTML Reset"
                </Button>
            </span>
        </form>
    }
}
