use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ChatDemo() -> impl IntoView {
    view! {
        <Card>
            <CardHeader class="flex items-center gap-4">
                <Avatar class="size-10">
                    <AvatarImage src="/avatar-1.png" />
                </Avatar>

                <div class="flex flex-col gap-1 mr-auto">
                    <p class="text-sm font-semibold leading-none text-(--card-foreground)">
                        "Sofia Davis"
                    </p>
                    <p class="text-sm">"m@example.com"</p>
                </div>

                <Tooltip value="New message">
                    <Button class="rounded-full" size="icon" variant="outline">
                        {icon!(icondata::LuPlus)}
                    </Button>
                </Tooltip>
            </CardHeader>

            <CardContent>

                <section class="space-y-4">
                    <div class="flex w-max max-w-[75%] flex-col gap-2 rounded-lg px-3 py-2 text-sm bg-(--muted)">
                        "Hi, how can I help you today?"
                    </div>
                    <div class="flex w-max max-w-[75%] flex-col gap-2 rounded-lg px-3 py-2 text-sm ml-auto bg-(--primary) text-(--primary-foreground)">
                        "Hey, I'm having trouble with my account."
                    </div>
                    <div class="flex w-max max-w-[75%] flex-col gap-2 rounded-lg px-3 py-2 text-sm bg-(--muted)">
                        "What seems to be the problem?"
                    </div>
                    <div class="flex w-max max-w-[75%] flex-col gap-2 rounded-lg px-3 py-2 text-sm ml-auto bg-(--primary) text-(--primary-foreground)">
                        "I can't log in."
                    </div>
                </section>
            </CardContent>
            <CardFooter class="gap-2">
                <Input placeholder="Type your message here..." />
                <Button size="icon">{icon!(icondata::FiSend)}</Button>
            </CardFooter>
        </Card>
    }
}
