use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupInputGroupExample() -> impl IntoView {
    let theme = expect_context::<ThemeProviderContext>();
    let voice_enabled = RwSignal::new(false);
    let class = RwSignal::new("rounded-full".to_string());
    let placeholder = RwSignal::new("Send a message...".to_string());

    Effect::new(move || {
        class.set(format!(
            "rounded-full{}",
            if voice_enabled.get() {
                match theme.mode.get() {
                    Mode::Dark => " bg-orange-800 text-orange-100",
                    Mode::Light => " bg-orange-100 text-orange-700",
                    _ => " bg-orange-100 text-orange-700 dark:bg-orange-800 dark:text-orange-100",
                }
            } else {
                ""
            }
        ));
    });

    Effect::new(move || {
        placeholder.set(
            if voice_enabled.get() {
                "Record and send audio..."
            } else {
                "Send a message..."
            }
            .to_string(),
        );
    });

    view! {
        <ButtonGroup>
            <ButtonGroup>
                <Button class="rounded-full" variant="outline" size="icon">
                    {icon!(icondata::LuPlus)}
                </Button>
            </ButtonGroup>
            <ButtonGroup>
                <InputGroup class="rounded-full">
                    <Input placeholder disabled=voice_enabled />
                    <InputGroupAddon align="inline-end">
                        <Tooltip value="Voice Mode">
                            <Button
                                on:click=move |_| voice_enabled.set(!voice_enabled.get())
                                size="icon-xs"
                                class
                            >
                                {icon!(icondata::LuAudioLines)}
                            </Button>
                        </Tooltip>
                    </InputGroupAddon>
                </InputGroup>
            </ButtonGroup>
        </ButtonGroup>
    }
}
