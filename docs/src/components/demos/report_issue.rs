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
                                <option value="team">"Team"</option>
                                <option value="billing">"Billing"</option>
                                <option value="account">"Account"</option>
                                <option value="deployments">"Deployments"</option>
                                <option value="support">"Support"</option>
                            </Select>
                        </div>
                        <div class="grid gap-2 flex-1">
                            <Label label_for="report-issue-security-level">"Security Level"</Label>
                            <Select id="report-issue-security-level" class="w-full">
                                <option value="1">"Severity 1 (Highest)"</option>
                                <option value="2">"Severity 2"</option>
                                <option value="3">"Severity 3"</option>
                                <option value="4">"Severity 4 (Lowest)"</option>
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
                        <Button variant="ghost" size="sm">
                            "Cancel"
                        </Button>
                        <Button size="sm">"Continue"</Button>
                    </CardFooter>
                </form>
            </CardContent>
        </Card>
    }
}
