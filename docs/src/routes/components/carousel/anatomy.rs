use leptos::prelude::*;
use singlestage::carousel::*;

#[component]
pub fn CarouselAnatomy() -> impl IntoView {
    view! {
        <Carousel>
            <CarouselContent>
                <CarouselItem />
                <CarouselPrevious />
                <CarouselNext />
            </CarouselContent>
        </Carousel>
    }
}
