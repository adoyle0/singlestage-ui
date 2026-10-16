use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldExample() -> impl IntoView {
    let billing_same_as_shipping = RwSignal::new(true);

    view! {
        <FieldSet class="max-w-md">
            <FieldLegend>"Payment Method"</FieldLegend>
            <FieldDescription>"All transactions are secure and encrypted"</FieldDescription>
            <FieldGroup>
                <Field>
                    <Input placeholder="Evil Rabbit">"Name on Card"</Input>
                </Field>
                <Field>
                    <Input placeholder="1234 5678 9012 3456">"Card Number"</Input>
                    <FieldDescription>"Enter your 16-digit card number"</FieldDescription>
                </Field>
                <FieldGroup class="grid grid-cols-3">
                    <Field>
                        <FieldLabel>"Month"</FieldLabel>
                        <Select placeholder="MM">
                            <SelectOption value="01">"01"</SelectOption>
                            <SelectOption value="02">"02"</SelectOption>
                            <SelectOption value="03">"03"</SelectOption>
                            <SelectOption value="04">"04"</SelectOption>
                            <SelectOption value="05">"05"</SelectOption>
                            <SelectOption value="06">"06"</SelectOption>
                            <SelectOption value="07">"07"</SelectOption>
                            <SelectOption value="08">"08"</SelectOption>
                            <SelectOption value="09">"09"</SelectOption>
                            <SelectOption value="10">"10"</SelectOption>
                            <SelectOption value="11">"11"</SelectOption>
                            <SelectOption value="12">"12"</SelectOption>
                        </Select>
                    </Field>
                    <Field>
                        <FieldLabel>"Year"</FieldLabel>
                        <Select placeholder="YYYY">
                            <SelectOption value="2024">"2024"</SelectOption>
                            <SelectOption value="2025">"2025"</SelectOption>
                            <SelectOption value="2026">"2026"</SelectOption>
                            <SelectOption value="2027">"2027"</SelectOption>
                            <SelectOption value="2028">"2028"</SelectOption>
                            <SelectOption value="2029">"2029"</SelectOption>
                        </Select>
                    </Field>
                    <Field>
                        <Input placeholder="123">"CVV"</Input>
                    </Field>
                </FieldGroup>
            </FieldGroup>
            <FieldSeparator />
            <FieldSet>
                <FieldLegend>"Billing Address"</FieldLegend>
                <FieldDescription>
                    "The billing address associated with your payment method"
                </FieldDescription>
                <FieldGroup>
                    <Field orientation="horizontal">
                        <Checkbox checked=billing_same_as_shipping>
                            "Same as shipping address"
                        </Checkbox>
                    </Field>
                    <Field>
                        <Textarea class="resize-none" placeholder="Add any additional comments">
                            "Comments"
                        </Textarea>
                    </Field>
                    <Field orientation="horizontal">
                        <Button>"Submit"</Button>
                        <Button variant="outline">"Cancel"</Button>
                    </Field>
                </FieldGroup>
            </FieldSet>
        </FieldSet>
    }
}
