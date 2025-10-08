use crate::components::*;
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <h1 class="text-4xl font-semibold">"A familiar UI library for Leptos"</h1>
        <p class="text-(--muted-foreground) pt-3">
            "Made with " <Link href="https://tailwindcss.com">"Tailwind CSS"</Link> ", Based on "
            <Link href="https://basecoatui.com">"Basecoat"</Link> ", "
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

        <section class="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-8">
            <div class="flex flex-col gap-4">
                <TeamMembers />
                <CookieSettings />
                <PaymentMethod />
            </div>

            <div class="flex flex-col gap-4">
                <ChatDemo />
                <CreateAccount />
                <ReportIssue />
            </div>
        </section>
    }
}
