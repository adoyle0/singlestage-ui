use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ButtonGroupPopoverExample() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant="outline">{icon!(icondata::LuBot)} "Copilot"</Button>
            <Popover>
                <PopoverTrigger>
                    <Button variant="outline" size="icon" aria_label="Open Popover">
                        {icon!(icondata::LuChevronDown)}
                    </Button>
                </PopoverTrigger>
                <PopoverContent align="end">
                    <PopoverHeader>
                        <PopoverTitle>"Start a new task with Copilot"</PopoverTitle>
                        <PopoverDescription>
                            "Describe your task in natural language."
                        </PopoverDescription>
                    </PopoverHeader>
                    <Field>
                        <Label label_for="task" class="sr-only">
                            "Task Description"
                        </Label>
                        <Textarea id="task" placeholder="I need to..." class="resize-none" />
                        <FieldDescription>
                            "Copilot will open a pull request for review."
                        </FieldDescription>
                    </Field>
                </PopoverContent>
            </Popover>
        </ButtonGroup>
    }
}
