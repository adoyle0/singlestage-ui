use leptos::prelude::*;
use singlestage::accordion::*;

#[component]
pub fn AccordionExample() -> impl IntoView {
    view! {
        <Accordion class="w-full sm:w-sm md:w-md">
            <AccordionItem name="demo" open=false>
                <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It adheres to the WAI-ARIA design pattern."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem name="demo" open=true>
                <AccordionTrigger>"Is it styled?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It comes with default styles
                    that match the other components' aesthetic."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem name="demo">
                <AccordionTrigger>"Is it animated?"</AccordionTrigger>
                <AccordionContent>"Yes. It's animated by default."</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
