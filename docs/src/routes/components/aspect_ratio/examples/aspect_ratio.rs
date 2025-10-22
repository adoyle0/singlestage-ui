use leptos::prelude::*;
use singlestage::aspect_ratio::*;

#[component]
pub fn AspectRatioExample() -> impl IntoView {
    view! {
        <div class="w-[300px]">
            <AspectRatio ratio=16. / 9.>
                <img
                    src="https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80"
                    alt="Photo by Drew Beamer"
                    class="w-full h-full rounded-lg object-cover dark:brightness-[0.2] dark:grayscale"
                />
            </AspectRatio>
        </div>
    }
}
