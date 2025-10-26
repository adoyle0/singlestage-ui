use leptos::prelude::*;
use singlestage::{card::*, carousel::*};

#[component]
pub fn CarouselExample() -> impl IntoView {
    let carousel_items = RwSignal::new((1..6).collect::<Vec<usize>>());

    view! {
        <Carousel class="w-full max-w-xs">
            <CarouselContent>
                <For
                    each=move || carousel_items.get()
                    key=|item| item.clone()
                    children=move |item| {
                        view! {
                            <CarouselItem>
                                <div class="p-1">
                                    <Card>
                                        <CardContent class="flex aspect-square items-center justify-center p-6">
                                            <span class="text-4xl font-semibold">
                                                {item.to_string()}
                                            </span>
                                        </CardContent>
                                    </Card>
                                </div>
                            </CarouselItem>
                        }
                    }
                />
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
        </Carousel>
    }
}
