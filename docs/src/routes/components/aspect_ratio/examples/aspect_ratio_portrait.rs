use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AspectRatioPortraitExample() -> impl IntoView {
    let theme_provider = expect_context::<ThemeProviderContext>();

    view! {
        <div class="w-full max-w-[10rem]">
            <AspectRatio ratio="9 / 16">
                <img
                    alt="Photo by Drew Beamer"
                    class=move || {
                        format!(
                            "w-full h-full rounded-lg object-cover{}",
                            match theme_provider.mode.get() {
                                Mode::Auto => " dark:brightness-[0.2] dark:grayscale",
                                Mode::Dark => " brightness-[0.2] grayscale",
                                _ => "",
                            },
                        )
                    }
                    src="https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80"
                />
            </AspectRatio>
        </div>
    }
}
