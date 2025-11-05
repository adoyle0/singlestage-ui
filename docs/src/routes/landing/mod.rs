use leptos::prelude::*;
use singlestage::*;

mod demos;
use demos::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <header class="w-fit mx-auto lg:ml-0 lg:mr-auto">
            <h1 class="text-4xl font-semibold">"A familiar UI library for Leptos"</h1>
            <p class="text-(--muted-foreground) pt-3">
                "Made with " <Link href="https://tailwindcss.com">"Tailwind CSS"</Link>
                ", Based on " <Link href="https://basecoatui.com">"Basecoat"</Link> ", "
                <Link href="https://ui.shadcn.com">"shadcn/ui"</Link> ", and "
                <Link href="https://radix-ui.com">"Radix"</Link> "."
            </p>

            <div class="my-6 space-x-2">
                <a href="install">
                    <Button>"Get Started"</Button>
                </a>
                <a href="introduction">
                    <Button variant="outline">"Learn More"</Button>
                </a>
            </div>
        </header>

        <section class="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-8 max-w-fit mx-auto lg:ml-0 lg:mr-auto">
            <div class="flex flex-col gap-4 max-w-md">
                <TeamMembers />
                <CookieSettings />
                <PaymentMethod />
            </div>

            <div class="flex flex-col gap-4 max-w-md">
                <ChatDemo />
                <CreateAccount />
                <ReportIssue />
            </div>
        </section>
    }
}
