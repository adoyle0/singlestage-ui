use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarExample() -> impl IntoView {
    let theme_provider = expect_context::<ThemeProviderContext>();

    view! {
        <div class="flex flex-row flex-wrap items-center gap-6 md:gap-12">
            <Avatar>
                <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" class="grayscale" />
                <AvatarFallback>"CN"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage src="https://github.com/evilrabbit.png" alt="@evilrabbit" />
                <AvatarFallback>"ER"</AvatarFallback>
                {move || match theme_provider.mode.get() {
                    Mode::Auto => {
                        view! { <AvatarBadge class="bg-green-600 dark:bg-green-800" /> }.into_any()
                    }
                    Mode::Dark => view! { <AvatarBadge class="bg-green-800" /> }.into_any(),
                    _ => view! { <AvatarBadge class="bg-green-600" /> }.into_any(),
                }}
            </Avatar>
            <AvatarGroup class="grayscale">
                <Avatar>
                    <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
                    <AvatarFallback>"CN"</AvatarFallback>
                </Avatar>
                <Avatar>
                    <AvatarImage src="https://github.com/maxleiter.png" alt="@maxleiter" />
                    <AvatarFallback>"LR"</AvatarFallback>
                </Avatar>
                <Avatar>
                    <AvatarImage src="https://github.com/evilrabbit.png" alt="@evilrabbit" />
                    <AvatarFallback>"ER"</AvatarFallback>
                </Avatar>
                <AvatarGroupCount>"+3"</AvatarGroupCount>
            </AvatarGroup>
        </div>
    }
}
