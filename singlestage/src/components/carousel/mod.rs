mod carousel;
mod content;
mod item;

pub use carousel::*;
pub use content::*;
pub use item::*;

use crate::Button;
use leptos::prelude::*;

#[derive(Clone)]
pub struct CarouselContext {
    pub current_item: RwSignal<i32>,
    pub num_items: RwSignal<i32>,
    pub ul_ref: NodeRef<leptos::html::Ul>,
}

#[component]
pub fn CarouselPrevious() -> impl IntoView {
    let carousel = expect_context::<CarouselContext>();

    let on_click = move |_| {
        let current_item = carousel.current_item.get_untracked();
        let num_items = carousel.num_items.get_untracked();

        // Wrap
        if current_item == 1 {
            carousel.current_item.set(num_items)
        } else {
            carousel.current_item.update(|cur| *cur -= 1)
        }
    };

    view! {
        <button class="singlestage-carousel-previous" on:click=on_click>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m12 19-7-7 7-7" />
                <path d="M19 12H5" />
            </svg>
        </button>
    }
}

#[component]
pub fn CarouselNext() -> impl IntoView {
    let carousel = expect_context::<CarouselContext>();

    let on_click = move |_| {
        let current_item = carousel.current_item.get_untracked();
        let num_items = carousel.num_items.get_untracked();

        // Wrap
        if current_item == num_items {
            carousel.current_item.set(1)
        } else {
            carousel.current_item.update(|cur| *cur += 1)
        }
    };

    view! {
        <button class="singlestage-carousel-next" on:click=on_click>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M5 12h14" />
                <path d="m12 5 7 7-7 7" />
            </svg>
        </button>
    }
}
