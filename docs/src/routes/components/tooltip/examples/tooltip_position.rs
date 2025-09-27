use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TooltipPositionExample() -> impl IntoView {
    view! {
        <div>
            <div class="flex">
                <Tooltip align="start" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "Start"
                    </Button>
                </Tooltip>
                <Tooltip align="center" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "Center"
                    </Button>
                </Tooltip>
                <Tooltip align="end" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "End"
                    </Button>
                </Tooltip>
            </div>

            <div class="flex">
                <div>
                    <Tooltip side="left" align="start" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Left-Start"
                        </Button>
                    </Tooltip>
                    <Tooltip side="left" align="center" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Left-Center"
                        </Button>
                    </Tooltip>
                    <Tooltip side="left" align="end" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Left-End"
                        </Button>
                    </Tooltip>
                </div>
                <div>
                    <Tooltip side="right" align="start" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Right-Start"
                        </Button>
                    </Tooltip>
                    <Tooltip side="right" align="center" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Left-Center"
                        </Button>
                    </Tooltip>
                    <Tooltip side="right" align="end" value="Add to library">
                        <Button class="w-[6rem] h-[4rem]" variant="outline">
                            "Right-End"
                        </Button>
                    </Tooltip>
                </div>
            </div>

            <div class="flex">
                <Tooltip side="bottom" align="start" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "Start"
                    </Button>
                </Tooltip>
                <Tooltip side="bottom" align="center" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "Center"
                    </Button>
                </Tooltip>
                <Tooltip side="bottom" align="end" value="Add to library">
                    <Button class="w-[4rem]" variant="outline">
                        "End"
                    </Button>
                </Tooltip>
            </div>
        </div>
    }
}
