use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardExample() -> impl IntoView {
    view! {
        <Card class="w-full max-w-sm">
            <CardHeader>
                <CardTitle>"Log in to your account"</CardTitle>
                <CardDescription>
                    "Enter your email below to Log in to your account"
                </CardDescription>
                <CardAction>
                    <Button variant="link">"Sign Up"</Button>
                </CardAction>
            </CardHeader>
            <CardContent>
                <form>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            <Label label_for="email">"Email"</Label>
                            <Input
                                id="email"
                                input_type="email"
                                placeholder="m@example.com"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center">
                                <Label label_for="password">"Password"</Label>
                                <a
                                    href="#"
                                    class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                                >
                                    "Forgot your password?"
                                </a>
                            </div>
                            <Input id="password" input_type="password" />
                        </div>
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex-col gap-2 border-t">
                <Button button_type="submit" class="w-full">
                    "Log in"
                </Button>
                <Button variant="outline" class="w-full">
                    "Log in with Google"
                </Button>
            </CardFooter>
        </Card>
    }
}
