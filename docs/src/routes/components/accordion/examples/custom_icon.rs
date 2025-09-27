use leptos::prelude::*;
use singlestage::{accordion::*, icon};

#[component]
pub fn CustomIconExample() -> impl IntoView {
    view! {
        <Accordion class="w-xs sm:w-sm md:w-md">
            <AccordionItem name="icon_demo" open=true>
                <AccordionTrigger>
                    "Custom icons" <AccordionIcon slot>{icon!(icondata::LuSmile)}</AccordionIcon>
                </AccordionTrigger>
                <AccordionContent>"You can use your own custom icons."</AccordionContent>
            </AccordionItem>
            <AccordionItem name="icon_demo">
                <AccordionTrigger>
                    "Turn that frown upside down"
                    <AccordionIcon slot>{icon!(icondata::LuFrown)}</AccordionIcon>
                </AccordionTrigger>
                <AccordionContent>"Custom icons make me happy."</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
