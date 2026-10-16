use leptos::prelude::*;
use singlestage::*;

#[derive(Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    duration: String,
}

#[component]
pub fn ItemImageExample() -> impl IntoView {
    let music = RwSignal::new(vec![
        Song {
            title: "Midnight City Lights".to_string(),
            artist: "Neon Dreams".to_string(),
            album: "Electric Nights".to_string(),
            duration: "3:45".to_string(),
        },
        Song {
            title: "Coffee Shop Conversations".to_string(),
            artist: "The Morning Brew".to_string(),
            album: "Urban Stories".to_string(),
            duration: "4:05".to_string(),
        },
        Song {
            title: "Digital Rain".to_string(),
            artist: "Cyber Symphony".to_string(),
            album: "Binary Beats".to_string(),
            duration: "3:30".to_string(),
        },
    ]);

    view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            <ItemGroup class="gap-4">
                <For each=move || music.get() key=|song| song.title.clone() let(song)>
                    {
                        let song_title = song.title.clone();
                        view! {
                            <a href="#" role="listitem">
                                <Item variant="outline">
                                    <ItemMedia variant="image">
                                        <img
                                            src=format!("https://avatar.vercel.sh/{}", song.title)
                                            alt=song.title
                                            width=32
                                            height=32
                                            class="object-cover grayscale"
                                        />
                                    </ItemMedia>
                                    <ItemContent>
                                        <ItemTitle class="line-clamp-1">
                                            {song_title} " - "
                                            <span class="text-(--muted-foreground)">{song.album}</span>
                                        </ItemTitle>
                                        <ItemDescription>{song.artist}</ItemDescription>
                                    </ItemContent>
                                    <ItemContent class="flex-none text-center">
                                        <ItemDescription>{song.duration}</ItemDescription>
                                    </ItemContent>
                                </Item>
                            </a>
                        }
                    }
                </For>
            </ItemGroup>
        </div>
    }
}
