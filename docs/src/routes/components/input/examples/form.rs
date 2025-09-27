use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FormExample() -> impl IntoView {
    view! {
        <form class="form grid gap-6">
            <div class="grid gap-2">
                <Label label_for="demo-form-text">"Username"</Label>
                <Input id="demo-form-text" placeholder="adoyle0" />
                <p class="text-muted-foreground text-sm">"This is your public display name."</p>
            </div>

            <div class="grid gap-2">
                <Label label_for="demo-form-select">"Email"</Label>
                <Select id="demo-form-select">
                    <option value="bob@example.com">"m@example.com"</option>
                    <option value="alice@example.com">"m@google.com"</option>
                    <option value="john@example.com">"m@support.com"</option>
                </Select>
                <p class="text-muted-foreground text-sm">
                    "You can manage email addresses in your email settings."
                </p>
            </div>

            <div class="grid gap-2">
                <Label label_for="demo-form-text">"Bio"</Label>
                <Textarea id="demo-form-textarea" placeholder="I like to..." rows=3></Textarea>
                <p class="text-muted-foreground text-sm">
                    "You can @mention other users and organizations."
                </p>
            </div>

            <div class="grid gap-2">
                <Label label_for="demo-form-date">"Date of birth"</Label>
                <Input input_type="date" id="demo-form-date" />
                <p class="text-muted-foreground text-sm">
                    "Your date of birth is used to calculate your age."
                </p>
            </div>

            <div class="flex flex-col gap-3">
                <Label label_for="demo-form-radio">"Notify me about..."</Label>
                <RadioGroup id="demo-form-radio">
                    <Label class="font-normal">
                        <Radio name="demo-form-radio" value="1" checked=true />
                        "All new messages"
                    </Label>
                    <Label class="font-normal">
                        <Radio name="demo-form-radio" value="2" />
                        "Direct messages and mentions"
                    </Label>
                    <Label class="font-normal">
                        <Radio name="demo-form-radio" value="3" />
                        "Nothing"
                    </Label>
                </RadioGroup>
            </div>

            <section class="grid gap-4">
                <h3 class="text-lg font-medium">"Email Notifications"</h3>
                <div class="gap-2 flex flex-row items-start justify-between rounded-lg border p-4 shadow-xs">
                    <div class="flex flex-col gap-0.5">
                        <Label label_for="demo-form-switch" class="leading-normal">
                            "Marketing emails"
                        </Label>
                        <p class="text-muted-foreground text-sm">
                            "Receive emails about new products, features, and more."
                        </p>
                    </div>
                    <Switch id="demo-form-switch" />
                </div>
                <div class="gap-2 flex flex-row items-start justify-between rounded-lg border p-4 shadow-xs">
                    <div class="flex flex-col gap-0.5 opacity-60">
                        <Label label_for="demo-form-switch-disabled" class="leading-normal">
                            "Security emails"
                        </Label>
                        <p class="text-muted-foreground text-sm">
                            "Receive emails about your account security."
                        </p>
                    </div>
                    <Switch id="demo-form-switch-disabled" disabled=true />
                </div>
            </section>

            <Button button_type="submit">"Submit"</Button>
        </form>
    }
}
