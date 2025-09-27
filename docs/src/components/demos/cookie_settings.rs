use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CookieSettings() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Cookie Settings"</CardTitle>
                <CardDescription>"Manage your cookie settings here."</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-6">
                <Label class="justify-between">
                    <div class="flex flex-col gap-2">
                        <p class="font-semibold">"Strictly Necessary"</p>
                        <p class="text-(--muted-foreground)">
                            "These cookies are essential in order to use the website and use its features."
                        </p>
                    </div>
                    <Switch checked=RwSignal::new(true) />
                </Label>

                <Label class="justify-between">
                    <div class="flex flex-col gap-2">
                        <p class="font-semibold">"Functional Cookies"</p>
                        <p class="text-(--muted-foreground)">
                            "These cookies allow the website to provide personalized funtionality."
                        </p>
                    </div>
                    <Switch />
                </Label>

                <Label class="justify-between">
                    <div class="flex flex-col gap-2">
                        <p class="font-semibold">"Performance Cookies"</p>
                        <p class="text-(--muted-foreground)">
                            "These cookies help to improve the performance of the website."
                        </p>
                    </div>
                    <Switch />
                </Label>

                <Button variant="outline">"Save Preferences"</Button>
            </CardContent>
        </Card>
    }
}
