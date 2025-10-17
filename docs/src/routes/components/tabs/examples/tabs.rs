use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TabsExample() -> impl IntoView {
    view! {
        <Tabs id="demo-tabs-with-panels">
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
            </TabsList>

            <TabsContent value="account">
                <Card>
                    <CardHeader>
                        <CardTitle>"Account"</CardTitle>
                        <CardDescription>
                            "Make changes to your account here. Click save when you're done."
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <form class="form grid gap-6">
                            <div class="grid gap-3">
                                <Label label_for="demo-tabs-account-name">"Name"</Label>
                                <Input
                                    input_type="text"
                                    id="demo-tabs-account-name"
                                    value="Pedro Duarte"
                                />
                            </div>
                            <div class="grid gap-3">
                                <Label label_for="demo-tabs-account-username">"Username"</Label>
                                <Input
                                    input_type="text"
                                    id="demo-tabs-account-username"
                                    value="@peduarte"
                                />
                            </div>
                        </form>
                    </CardContent>
                    <CardFooter>
                        <Button button_type="button">"Save changes"</Button>
                    </CardFooter>
                </Card>
            </TabsContent>

            <TabsContent value="password">
                <Card>
                    <CardHeader>
                        <CardTitle>"Password"</CardTitle>
                        <CardDescription>
                            "Change your password here. After saving, you'll be logged out."
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <form class="form grid gap-6">
                            <div class="grid gap-3">
                                <Label label_for="demo-tabs-password-current">
                                    "Current password"
                                </Label>
                                <Input input_type="password" id="demo-tabs-password-current" />
                            </div>
                            <div class="grid gap-3">
                                <Label label_for="demo-tabs-password-new">"New password"</Label>
                                <Input input_type="password" id="demo-tabs-password-new" />
                            </div>
                        </form>
                    </CardContent>
                    <CardFooter>
                        <Button button_type="button">"Save Password"</Button>
                    </CardFooter>
                </Card>
            </TabsContent>
        </Tabs>
    }
}
