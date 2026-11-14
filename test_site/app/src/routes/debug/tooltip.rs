use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn DebugTooltip() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"Tooltip"</h1>

        <div class="my-4">
            <div class="flex">
                <For
                    each=move || ["start", "center", "end"]
                    key=|align| align.to_string()
                    let(align)
                >
                    <Tooltip>
                        <TooltipTrigger>
                            <Button class="w-[4rem]" variant="outline">
                                "-"
                            </Button>
                        </TooltipTrigger>
                        <TooltipContent align={align}>
                            <p>"Add to library"</p>
                        </TooltipContent>
                    </Tooltip>
                </For>
            </div>

            <div class="flex gap-16">
                <For each=move || ["left", "right"] key=|align| align.to_string() let(side)>
                    <div class="flex-col gap-2">
                        <For
                            each=move || ["start", "center", "end"]
                            key=|align| align.to_string()
                            let(align)
                        >
                            <Tooltip>
                                <TooltipTrigger>
                                    <Button class="w-[4rem]" variant="outline">
                                        "-"
                                    </Button>
                                </TooltipTrigger>
                                <TooltipContent align={align} side={side}>
                                    <p>"Add to library"</p>
                                </TooltipContent>
                            </Tooltip>
                        </For>
                    </div>
                </For>
            </div>

            <div class="flex">
                <For
                    each=move || ["start", "center", "end"]
                    key=|align| align.to_string()
                    let(align)
                >
                    <Tooltip>
                        <TooltipTrigger>
                            <Button class="w-[4rem]" variant="outline">
                                "-"
                            </Button>
                        </TooltipTrigger>
                        <TooltipContent align={align} side="bottom">
                            <p>"Add to library"</p>
                        </TooltipContent>
                    </Tooltip>
                </For>
            </div>
        </div>
    }
}
