use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardExample() -> impl IntoView {
    view! {
        <Card class="max-w-sm">
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
                <FieldSet>
                    <Field>
                        <Input input_type="email" placeholder="m@example.com">
                            "Email"
                        </Input>
                    </Field>
                    <Field>
                        <div class="flex">
                            <FieldLabel>"Password"</FieldLabel>
                            <Link
                                href="#"
                                class="ml-auto text-sm font-normal no-underline hover:underline"
                            >
                                "Forgot your password?"
                            </Link>
                        </div>
                        <Input input_type="password" />
                    </Field>
                </FieldSet>
            </CardContent>
            <CardFooter class="flex-col gap-2 border-t">
                <Button class="w-full">"Log in"</Button>
                <Button variant="outline" class="w-full">
                    "Log in with Google"
                </Button>
            </CardFooter>
        </Card>
    }
}
