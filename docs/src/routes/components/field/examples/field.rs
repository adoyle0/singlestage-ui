use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn FieldExample() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <form>
                <FieldGroup>
                    <FieldSet>
                        <FieldLegend>"Payment Method"</FieldLegend>
                        <FieldDescription>
                            "All transactions are secure and encrypted"
                        </FieldDescription>
                        <FieldGroup>
                            <Field>
                                <Input placeholder="Evil Rabbit" required=true>
                                    "Name on Card"
                                </Input>
                            </Field>
                            <Field>
                                <Input
                                    placeholder="1234 5678 9012 3456"
                                    inputmode="numeric"
                                    required=true
                                >
                                    "Card Number"
                                </Input>
                                <FieldDescription>
                                    "Enter your 16-digit card number"
                                </FieldDescription>
                            </Field>
                            <FieldGroup class="grid grid-cols-3">
                                <Field>
                                    <FieldLabel>"Month"</FieldLabel>
                                    <Select placeholder="MM">
                                        <SelectOptGroup>
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
                                        </SelectOptGroup>
                                    </Select>
                                </Field>
                                <Field>
                                    <FieldLabel>"Year"</FieldLabel>
                                    <Select placeholder="YYYY">
                                        <SelectOptGroup>
                                            <SelectOption value="2024">"2024"</SelectOption>
                                            <SelectOption value="2025">"2025"</SelectOption>
                                            <SelectOption value="2026">"2026"</SelectOption>
                                            <SelectOption value="2027">"2027"</SelectOption>
                                            <SelectOption value="2028">"2028"</SelectOption>
                                            <SelectOption value="2029">"2029"</SelectOption>
                                        </SelectOptGroup>
                                    </Select>
                                </Field>
                                <Field>
                                    <Input inputmode="numeric" placeholder="123" required=true>
                                        "CVV"
                                    </Input>
                                </Field>
                            </FieldGroup>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSeparator />
                    <FieldSet>
                        <FieldLegend>"Billing Address"</FieldLegend>
                        <FieldDescription>
                            "The billing address associated with your payment method"
                        </FieldDescription>
                        <Field orientation="horizontal">
                            <Checkbox class="font-normal" checked=true>
                                "Same as shipping address"
                            </Checkbox>
                        </Field>
                    </FieldSet>
                    <FieldSet>
                        <Field>
                            <Textarea placeholder="Add any additional comments" class="resize-none">
                                "Comments"
                            </Textarea>
                        </Field>
                    </FieldSet>
                    <Field orientation="horizontal">
                        <Button button_type="submit">"Submit"</Button>
                        <Button variant="outline" button_type="button">
                            "Cancel"
                        </Button>
                    </Field>
                </FieldGroup>
            </form>
        </div>
    }
}
