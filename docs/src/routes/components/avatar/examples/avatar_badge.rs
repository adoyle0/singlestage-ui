use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AvatarBadgeExample() -> impl IntoView {
    let theme_provider = expect_context::<ThemeProviderContext>();

    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>"CN"</AvatarFallback>
            {move || match theme_provider.mode.get() {
                Mode::Auto => {
                    view! { <AvatarBadge class="bg-green-600 dark:bg-green-800" /> }.into_any()
                }
                Mode::Dark => view! { <AvatarBadge class="bg-green-800" /> }.into_any(),
                _ => view! { <AvatarBadge class="bg-green-600" /> }.into_any(),
            }}
        </Avatar>
    }
}
