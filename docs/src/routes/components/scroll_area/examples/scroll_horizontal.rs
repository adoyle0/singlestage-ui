use leptos::prelude::*;
use singlestage::scroll_area::*;

#[derive(Clone)]
struct Artwork {
    art: String,
    artist: String,
}

#[component]
pub fn ScrollHorizontalExample() -> impl IntoView {
    let works = RwSignal::new(vec![
        Artwork {
            artist: "Ornella Binni".to_string(),
            art: "https://images.unsplash.com/photo-1465869185982-5a1a7522cbcb?auto=format&fit=crop&w=300&q=80".to_string(),
        },
        Artwork {
            artist: "Tom Byrom".to_string(),
            art: "https://images.unsplash.com/photo-1548516173-3cabfa4607e9?auto=format&fit=crop&w=300&q=80".to_string(),
        },
        Artwork {
            artist: "Vladimir Malyavko".to_string(),
            art: "https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80".to_string(),
        },
    ]);

    view! {
        <ScrollArea class="w-96 rounded-md border whitespace-nowrap" orientation="horizontal">
            <div class="flex w-max space-x-4 p-4">
                <For each=move || works.get() key=|artwork| artwork.artist.clone() let(artwork)>
                    <figure class="shrink-0">
                        <div class="overflow-hidden rounded-md">
                            <img
                                src=artwork.art
                                alt=format!("Photo by {}", artwork.artist)
                                class="aspect-[3/4] h-fit w-fit object-cover"
                                width=300
                                height=400
                            />
                        </div>
                        <figcaption class="text-muted-foreground pt-2 text-xs">
                            "Photo by "
                            <span class="text-foreground font-semibold">{artwork.artist}</span>
                        </figcaption>
                    </figure>
                </For>
            </div>
        </ScrollArea>
    }
}
