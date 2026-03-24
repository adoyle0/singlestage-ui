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
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant="outline">"Select" {icon!(icondata::LuChevronDown)}</Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-48" align="center">
                <For each=move || people.get() key=|person| person.username.clone() let(person)>
                    {
                        let username = person.username.clone();
                        view! {
                            <DropdownMenuItem class="p-0">
                                <Item size="xs" class="w-full">
                                    <ItemMedia>
                                        <Avatar class="size-[--spacing(6.5)]">
                                            <AvatarImage src=person.avatar class="grayscale" />
                                            <AvatarFallback>
                                                {username[..2].to_uppercase()}
                                            </AvatarFallback>
                                        </Avatar>
                                    </ItemMedia>
                                    <ItemContent class="gap-0">
                                        <ItemTitle>{person.username}</ItemTitle>
                                        <ItemDescription class="leading-none">
                                            {person.email}
                                        </ItemDescription>
                                    </ItemContent>
                                </Item>
                            </DropdownMenuItem>
                        }
                    }
                </For>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
