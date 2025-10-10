use leptos::prelude::*;
use singlestage::{card::*, carousel::*};

#[component]
pub fn CarouselExample() -> impl IntoView {
    view! {
        <Carousel>
            <CarouselContent class="text-4xl font-bold">
                <CarouselItem>
                    <Card class="flex size-32">
                        <CardContent class="h-full flex items-center justify-center">
                            <p>"1"</p>
                        </CardContent>
                    </Card>
                </CarouselItem>
                <CarouselItem>
                    <Card class="flex size-32">
                        <CardContent class="h-full flex items-center justify-center">
                            <p>"2"</p>
                        </CardContent>
                    </Card>
                </CarouselItem>
                <CarouselItem>
                    <Card class="flex size-32">
                        <CardContent class="h-full flex items-center justify-center">
                            <p>"3"</p>
                        </CardContent>
                    </Card>
                </CarouselItem>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
        </Carousel>
    }
}
