use leptos::prelude::*;
use singlestage::*;

#[derive(Clone)]
struct Person {
    username: String,
    avatar: String,
    email: String,
}

#[component]
pub fn ItemDropdownExample() -> impl IntoView {
    let people = RwSignal::new(vec![
        Person {
            username: "shadcn".to_string(),
            avatar: "https://github.com/shadcn.png".to_string(),
            email: "shadcn@vercel.com".to_string(),
        },
        Person {
            username: "maxleiter".to_string(),
            avatar: "https://github.com/maxleiter.png".to_string(),
            email: "maxleiter@vercel.com".to_string(),
        },
        Person {
            username: "evilrabbit".to_string(),
            avatar: "https://github.com/evilrabbit.png".to_string(),
            email: "evilrabbit@vercel.com".to_string(),
        },
    ]);

    view! {
        <div class="flex min-h-64 w-full max-w-md flex-col items-center gap-6">
            <DropdownMenu>
                <DropdownMenuTrigger variant="outline" size="sm" class="w-fit">
                    "Select"
                    {icon!(icondata::LuChevronDown)}
                </DropdownMenuTrigger>
                <DropdownMenuContent class="w-72 [--radius:0.65rem]">
                    <For each=move || people.get() key=|person| person.username.clone() let(person)>
                        <DropdownMenuItem class="p-0">
                            <Item size="sm" class="w-full p-2">
                                <ItemMedia>
                                    <Avatar class="size-8">
                                        <AvatarImage src=person.avatar class="grayscale" />
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-0.5">
                                    <ItemTitle>{person.username}</ItemTitle>
                                    <ItemDescription>{person.email}</ItemDescription>
                                </ItemContent>
                            </Item>
                        </DropdownMenuItem>
                    </For>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    }
}
