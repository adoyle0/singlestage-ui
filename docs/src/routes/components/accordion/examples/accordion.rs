use leptos::prelude::*;
use singlestage::accordion::*;

#[component]
pub fn AccordionExample() -> impl IntoView {
    view! {
        <Accordion class="w-full sm:w-sm md:w-md">
            <AccordionItem name="demo" open=false>
                <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                <AccordionContent>
                    <p>"Yes. It adheres to the WAI-ARIA design pattern."</p>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem name="demo" open=true>
                <AccordionTrigger>"Is it styled?"</AccordionTrigger>
                <AccordionContent>
                    <p>
                        "Yes. It comes with default styles
                        that match the other components' aesthetic."
                    </p>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem name="demo">
                <AccordionTrigger>"Is it animated?"</AccordionTrigger>
                <AccordionContent>
                    <p>"Yes. It's animated by default."</p>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
