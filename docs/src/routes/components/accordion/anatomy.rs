use leptos::prelude::*;
use singlestage::accordion::*;

#[component]
pub fn AccordionAnatomy() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionItem>
                <AccordionTrigger
                    <AccordionIcon slot />
                <AccordionTrigger/>
                <AccordionContent />
            </AccordionItem>
        </Accordion>
    }
}
