use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardImageExample() -> impl IntoView {
    let theme_provider = expect_context::<ThemeProviderContext>();

    view! {
        <Card class="relative max-w-sm">
            <img
                src="https://avatar.vercel.sh/shadcn1"
                alt="Event cover"
                class=move || {
                    format!(
                        "aspect-video w-full object-cover grayscale {}",
                        match theme_provider.mode.get() {
                            Mode::Auto => "brightness-60 dark:brightness-40",
                            Mode::Dark => "brightness-40",
                            _ => "brightness-60",
                        },
                    )
                }
            />
            <div class="absolute inset-0 aspect-video bg-black/35" />
            <CardHeader>
                <CardAction>
                    <Badge variant="secondary">"Featured"</Badge>
                </CardAction>
                <CardTitle>"Design systems meetup"</CardTitle>
                <CardDescription>
                    "A practical talk on component APIs, accessibility, and shipping
                    faster."
                </CardDescription>
            </CardHeader>
            <CardFooter>
                <Button class="w-full">"View Event"</Button>
            </CardFooter>
        </Card>
    }
}
