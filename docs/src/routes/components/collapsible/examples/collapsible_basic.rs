use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CollapsibleBasicExample() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Card class="mx-auto w-full max-w-sm">
            <CardContent>
                <Collapsible
                    open
                    attr:class=move || {
                        format!(
                            "rounded-md{}",
                            match open.get() {
                                true => " bg-(--muted)",
                                false => "",
                            },
                        )
                    }
                >
                    <CollapsibleTrigger>
                        <Button variant="ghost" class="group w-full">
                            "Product details"
                            <span class=move || {
                                format!(
                                    "ml-auto{}",
                                    match open.get() {
                                        true => " rotate-180",
                                        false => "",
                                    },
                                )
                            }>{icon!(icondata::LuChevronDown)}</span>
                        </Button>
                    </CollapsibleTrigger>
                    <CollapsibleContent class="flex flex-col items-start gap-2 p-2.5 pt-0 text-sm">
                        <div>
                            "This panel can be expanded or collapsed to reveal additional
                            content."
                        </div>
                        <Button size="xs">"Learn More"</Button>
                    </CollapsibleContent>
                </Collapsible>
            </CardContent>
        </Card>
    }
}
