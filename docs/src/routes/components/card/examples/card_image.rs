use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardImageExample() -> impl IntoView {
    view! {
        <Card class="relative mx-auto w-full max-w-sm">
            <img
                src="https://avatar.vercel.sh/shadcn1"
                alt="Event cover"
                class="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40"
            />
            <div class="absolute inset-0 z-30 aspect-video bg-black/35" />
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
