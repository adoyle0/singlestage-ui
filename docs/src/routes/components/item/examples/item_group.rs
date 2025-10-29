use leptos::prelude::*;
use singlestage::*;

#[derive(Clone)]
struct Person {
    username: String,
    avatar: String,
    email: String,
}

#[component]
pub fn ItemGroupExample() -> impl IntoView {
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
        <div class="flex w-full max-w-md flex-col gap-6">
            <ItemGroup>
                <For each=move || people.get() key=|person| person.username.clone() let(person)>
                    {
                        let username = person.username.clone();

                        view! {
                            <Item>
                                <ItemMedia>
                                    <Avatar>
                                        <AvatarImage src=person.avatar class="grayscale" />
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-1">
                                    <ItemTitle>{person.username}</ItemTitle>
                                    <ItemDescription>{person.email}</ItemDescription>
                                </ItemContent>
                                <ItemActions>
                                    <Button variant="ghost" size="icon" class="rounded-full">
                                        {icon!(icondata::LuPlus)}
                                    </Button>
                                </ItemActions>
                            </Item>

                            {if username.as_str() != "evilrabbit" {

                                view! { <ItemSeparator /> }
                                    .into_any()
                            } else {
                                "".into_any()
                            }}
                        }
                    }
                </For>
            </ItemGroup>
        </div>
    }
}
