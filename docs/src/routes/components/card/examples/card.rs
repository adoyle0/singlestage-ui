use leptos::prelude::*;
use singlestage::{card::*, Button, Input, Label};

#[component]
pub fn CardExample() -> impl IntoView {
    view! {
        <Card class="w-full sm:w-sm">
            <CardHeader>
                <CardTitle>
                    <p>"Log in to your account"</p>
                </CardTitle>
                <CardDescription>
                    <p>"Enter your details below to log in to your account"</p>
                </CardDescription>
            </CardHeader>
            <CardContent>
                <form class="form grid gap-6">
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-email">"Email"</Label>
                        <Input input_type="email" id="demo-card-form-email" />
                    </div>
                    <div class="grid gap-2">
                        <div class="flex items-center gap-2">
                            <Label label_for="demo-card-form-password">"Password"</Label>
                            <a
                                href="#"
                                class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                            >
                                "Forgot your password?"
                            </a>
                        </div>
                        <Input input_type="password" id="demo-card-form-password" />
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex flex-col items-center gap-2">
                <Button button_type="button" class="w-full">
                    "Log in"
                </Button>
                <Button button_type="button" variant="outline" class="w-full">
                    "Log in with Google"
                </Button>
                <p class="mt-4 text-center text-sm">
                    "Don't have an account? "<a href="#" class="underline-offset-4 hover:underline">
                        "Sign up"
                    </a>
                </p>
            </CardFooter>
        </Card>
    }
}
