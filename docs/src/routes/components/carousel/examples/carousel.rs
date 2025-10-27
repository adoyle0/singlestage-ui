use leptos::prelude::*;
use singlestage::{card::*, carousel::*};

#[component]
pub fn CarouselExample() -> impl IntoView {
    let carousel_items = RwSignal::new((1..6).collect::<Vec<usize>>());

    view! {
        <Carousel class="w-full max-w-xs">
            <CarouselContent>
                <For each=move || carousel_items.get() key=|item| item.clone() let(item)>
                    <CarouselItem>
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center">
                                <span class="text-4xl font-semibold">{item.to_string()}</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                </For>
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
        </Carousel>
    }
}
