use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ReportIssue() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <h2>"Report an issue"</h2>
                <p>"What area are you having problems with?"</p>
            </CardHeader>
            <CardContent>
                <form class="form grid gap-6">
                    <div class="flex gap-4">
                        <div class="grid gap-2 flex-1">
                            <Label label_for="report-issue-area">"Area"</Label>
                            <Select id="report-issue-area" class="w-full">
                                <SelectOption value="team">"Team"</SelectOption>
                                <SelectOption value="billing">"Billing"</SelectOption>
                                <SelectOption value="account">"Account"</SelectOption>
                                <SelectOption value="deployments">"Deployments"</SelectOption>
                                <SelectOption value="support">"Support"</SelectOption>
                            </Select>
                        </div>
                        <div class="grid gap-2 flex-1">
                            <Label label_for="report-issue-security-level">"Security Level"</Label>
                            <Select id="report-issue-security-level" class="w-full">
                                <SelectOption value="1">"Severity 1 (Highest)"</SelectOption>
                                <SelectOption value="2">"Severity 2"</SelectOption>
                                <SelectOption value="3">"Severity 3"</SelectOption>
                                <SelectOption value="4">"Severity 4 (Lowest)"</SelectOption>
                            </Select>
                        </div>
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="report-issue-subject">"Subject"</Label>
                        <Input
                            id="report-issue-subject"
                            placeholder="I need help with..."
                            class="w-full"
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="report-issue-description">"Description"</Label>
                        <Textarea
                            id="report-issue-description"
                            placeholder="Please include all information relevant to your issue."
                            class="w-full"
                        ></Textarea>
                    </div>
                    <CardFooter class="flex items-center gap-4 justify-between">
                        <Button variant="ghost">
                            "Cancel"
                        </Button>
                        <Button>"Continue"</Button>
                    </CardFooter>
                </form>
            </CardContent>
        </Card>
    }
}
