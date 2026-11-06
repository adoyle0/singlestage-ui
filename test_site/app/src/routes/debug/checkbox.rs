use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugCheckbox() -> impl IntoView {
    let checked = RwSignal::new(true);
    let disabled = RwSignal::new(true);
    let invalid = RwSignal::new(true);
    let value = RwSignal::new(vec!["two".to_string()]);

    view! {
        <h1 class="text-4xl font-semibold">"Checkbox"</h1>
        <ul class="singlestage-ulist text-(--muted-foreground)">
            <li>"Checked state should not flicker on load or reload"</li>
            <li>
                "Checked state should not lose sync with the signal when alternating clicking the
                button and the checkbox"
            </li>
            <li>"Disabled state should not flicker on load or reload"</li>
        </ul>

        <ul class="text-(--muted-foreground)">
            <li>"Checked: "{move || checked.get().to_string()}</li>
            <li>"Disabled: "{move || disabled.get().to_string()}</li>
        </ul>

        <Label>
            <Checkbox disabled checked />
            "Accept terms and conditions"
        </Label>

        <Label>
            <Switch disabled checked />
            "Accept terms and conditions"
        </Label>

        <Button variant="outline" on:click=move |_| { checked.set(!checked.get_untracked()) }>
            "Toggle Checked"
        </Button>

        <p>{move || value.get()}</p>
        <CheckboxGroup disabled invalid value>
            <legend>"Group"</legend>
            <Checkbox value="one">"One"</Checkbox>
            <Checkbox value="two">"Two"</Checkbox>
            <Checkbox value="three">"Three"</Checkbox>
        </CheckboxGroup>
        <CheckboxGroup disabled invalid value>
            <legend>"Group"</legend>
            <Switch value="one">"One"</Switch>
            <Switch value="two">"Two"</Switch>
            <Switch value="three">"Three"</Switch>
        </CheckboxGroup>

        <Switch disabled invalid>
            "Solo"
        </Switch>

        <Field disabled invalid>
            <Checkbox>"in Field"</Checkbox>
        </Field>

        <Field disabled invalid>
            <Switch>"in Field"</Switch>
        </Field>

        <span class="space-x-2">
            <Button variant="outline" on:click=move |_| { disabled.set(!disabled.get_untracked()) }>
                "Toggle Disabled"
            </Button>
            <Button variant="outline" on:click=move |_| { invalid.set(!invalid.get_untracked()) }>
                "Toggle Invalid"
            </Button>
            <Button
                variant="outline"
                on:click=move |_| {
                    value
                        .update(move |v| {
                            let idx = v.into_iter().position(|el| *el == "one".to_string());
                            if let Some(idx) = idx {
                                v.remove(idx);
                            } else {
                                v.push("one".to_string());
                            }
                        })
                }
            >
                "One"
            </Button>
            <Button
                variant="outline"
                on:click=move |_| {
                    value
                        .update(move |v| {
                            let idx = v.into_iter().position(|el| *el == "two".to_string());
                            if let Some(idx) = idx {
                                v.remove(idx);
                            } else {
                                v.push("two".to_string());
                            }
                        })
                }
            >
                "Two"
            </Button>
            <Button
                variant="outline"
                on:click=move |_| {
                    value
                        .update(move |v| {
                            let idx = v.into_iter().position(|el| *el == "three".to_string());
                            if let Some(idx) = idx {
                                v.remove(idx);
                            } else {
                                v.push("three".to_string());
                            }
                        })
                }
            >
                "Three"
            </Button>
        </span>
    }
}
