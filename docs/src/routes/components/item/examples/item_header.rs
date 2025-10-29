use leptos::prelude::*;
use singlestage::*;

#[derive(Clone)]
struct Model {
    name: String,
    description: String,
    image: String,
    credit: String,
}

#[component]
pub fn ItemHeaderExample() -> impl IntoView {
    let models = RwSignal::new(vec![
        Model {
            name: "v0-1.5-sm".to_string(),
            description: "Everyday tasks and UI generation.".to_string(),
            image: "https://images.unsplash.com/photo-1650804068570-7fb2e3dbf888?q=80&w=640&auto=format&fit=crop".to_string(),
            credit: "Valeria Reverdo on Unsplash".to_string(),
        },
        Model {
            name: "v0-1.5-lg".to_string(),
            description: "Advanced thinking or reasoning.".to_string(),
            image: "https://images.unsplash.com/photo-1610280777472-54133d004c8c?q=80&w=640&auto=format&fit=crop".to_string(),
            credit: "Michael Oeser on Unsplash".to_string(),
        },
        Model {
            name: "v0-2.0-mini".to_string(),
            description: "Open Source model for everyone.".to_string(),
            image: "https://images.unsplash.com/photo-1602146057681-08560aee8cde?q=80&w=640&auto=format&fit=crop".to_string(),
            credit: "Cherry Laithang on Unsplash".to_string(),
        },
    ]);

    view! {
        <div class="flex w-full max-w-xl flex-col gap-6">
            <ItemGroup class="grid grid-cols-3 gap-4">
                <For each=move || models.get() key=|model| model.name.clone() let(model)>
                    {
                        let model_name = model.name.clone();

                        view! {
                            <Item variant="outline">
                                <ItemHeader>
                                    <Tooltip value=model.credit>
                                        <img
                                            src=model.image
                                            alt=model.name
                                            width=128
                                            height=128
                                            class="aspect-square w-full rounded-sm object-cover"
                                        />
                                    </Tooltip>
                                </ItemHeader>
                                <ItemContent>
                                    <ItemTitle>{model_name}</ItemTitle>
                                    <ItemDescription>{model.description}</ItemDescription>
                                </ItemContent>
                            </Item>
                        }
                    }
                </For>
            </ItemGroup>
        </div>
    }
}
