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
                                <FieldLabel label_for="checkout-7j9-card-name-43j">
                                    "Name on Card"
                                </FieldLabel>
                                <Input id="checkout-7j9-card-name-43j" placeholder="Evil Rabbit" />

                            </Field>
                            <Field>
                                <FieldLabel label_for="checkout-7j9-card-number-uw1">
                                    "Card Number"
                                </FieldLabel>
                                <Input
                                    id="checkout-7j9-card-number-uw1"
                                    placeholder="1234 5678 9012 3456"
                                />

                                <FieldDescription>
                                    "Enter your 16-digit card number"
                                </FieldDescription>
                            </Field>
                            <div class="grid grid-cols-3 gap-4">
                                <Field>
                                    <FieldLabel label_for="checkout-exp-month-ts6">
                                        "Month"
                                    </FieldLabel>
                                    <Select placeholder="MM" id="checkout-exp-month-ts6">
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
                                    <FieldLabel label_for="checkout-7j9-exp-year-f59">
                                        "Year"
                                    </FieldLabel>
                                    <Select id="checkout-7j9-exp-year-f59" placeholder="YYYY">
                                        <SelectOption value="2024">"2024"</SelectOption>
                                        <SelectOption value="2025">"2025"</SelectOption>
                                        <SelectOption value="2026">"2026"</SelectOption>
                                        <SelectOption value="2027">"2027"</SelectOption>
                                        <SelectOption value="2028">"2028"</SelectOption>
                                        <SelectOption value="2029">"2029"</SelectOption>
                                    </Select>
                                </Field>
                                <Field>
                                    <FieldLabel label_for="checkout-7j9-cvv">"CVV"</FieldLabel>
                                    <Input id="checkout-7j9-cvv" placeholder="123" />
                                </Field>
                            </div>
                        </FieldGroup>
                    </FieldSet>
                    <Separator />
                    <FieldSet>
                        <FieldLegend>"Billing Address"</FieldLegend>
                        <FieldDescription>
                            "The billing address associated with your payment method"
                        </FieldDescription>
                        <FieldGroup>
                            <Field orientation="horizontal">
                                <Checkbox id="checkout-7j9-same-as-shipping-wgm" checked=true />
                                <FieldLabel
                                    label_for="checkout-7j9-same-as-shipping-wgm"
                                    class="font-normal"
                                >
                                    "Same as shipping address"
                                </FieldLabel>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSet>
                        <FieldGroup>
                            <Field>
                                <FieldLabel label_for="checkout-7j9-optional-comments">
                                    "Comments"
                                </FieldLabel>
                                <Textarea
                                    id="checkout-7j9-optional-comments"
                                    placeholder="Add any additional comments"
                                    class="resize-none"
                                />
                            </Field>
                        </FieldGroup>
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
