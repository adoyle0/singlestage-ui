use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CreateAccount() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <h2>"Create an account"</h2>
                <p>"Enter your email below to create your account"</p>
            </CardHeader>

            <CardContent>
                <div class="flex gap-6">
                    <Button class="flex-1" variant="outline">
                        {icon!(icondata::BsGithub)}
                        " GitHub"
                    </Button>
                    <Button class="flex-1" variant="outline">
                        <svg
                            role="img"
                            fill="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Google</title>
                            <path d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"></path>
                        </svg>
                        " Google"
                    </Button>
                </div>

                <div class="relative my-6">
                    <div class="absolute inset-0 flex items-center">
                        <span class="w-full border-t"></span>
                    </div>
                    <div class="relative flex justify-center text-xs uppercase">
                        <span class="bg-(--card) px-2 text-(--muted-foreground)">
                            "Or continue with"
                        </span>
                    </div>
                </div>

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
                    <Button>"Create account"</Button>
                </form>

            </CardContent>
        </Card>
    }
}
