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
                                <Input
                                    id="checkout-7j9-card-name-43j"
                                    placeholder="Evil Rabbit"
                                    required=true
                                />
                            </Field>
                            <Field>
                                <FieldLabel label_for="checkout-7j9-card-number-uw1">
                                    "Card Number"
                                </FieldLabel>
                                <Input
                                    id="checkout-7j9-card-number-uw1"
                                    placeholder="1234 5678 9012 3456"
                                    required=true
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
                                    <Select default="">
                                        // <SelectTrigger id="checkout-exp-month-ts6">
                                        // <SelectValue placeholder="MM" />
                                        // </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="01">"01"</SelectItem>
                                            <SelectItem value="02">"02"</SelectItem>
                                            <SelectItem value="03">"03"</SelectItem>
                                            <SelectItem value="04">"04"</SelectItem>
                                            <SelectItem value="05">"05"</SelectItem>
                                            <SelectItem value="06">"06"</SelectItem>
                                            <SelectItem value="07">"07"</SelectItem>
                                            <SelectItem value="08">"08"</SelectItem>
                                            <SelectItem value="09">"09"</SelectItem>
                                            <SelectItem value="10">"10"</SelectItem>
                                            <SelectItem value="11">"11"</SelectItem>
                                            <SelectItem value="12">"12"</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </Field>
                                <Field>
                                    <FieldLabel label_for="checkout-7j9-exp-year-f59">
                                        "Year"
                                    </FieldLabel>
                                    <Select default="">
                                        // <SelectTrigger id="checkout-7j9-exp-year-f59">
                                        // <SelectValue placeholder="YYYY" />
                                        // </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="2024">"2024"</SelectItem>
                                            <SelectItem value="2025">"2025"</SelectItem>
                                            <SelectItem value="2026">"2026"</SelectItem>
                                            <SelectItem value="2027">"2027"</SelectItem>
                                            <SelectItem value="2028">"2028"</SelectItem>
                                            <SelectItem value="2029">"2029"</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </Field>
                                <Field>
                                    <FieldLabel label_for="checkout-7j9-cvv">"CVV"</FieldLabel>
                                    <Input id="checkout-7j9-cvv" placeholder="123" required=true />
                                </Field>
                            </div>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSeparator />
                    <FieldSet>
                        <FieldLegend>"Billing Address"</FieldLegend>
                        <FieldDescription>
                            "The billing address associated with your payment method"
                        </FieldDescription>
                        <FieldGroup>
                            <Field orientation="horizontal">
                                <Checkbox
                                    id="checkout-7j9-same-as-shipping-wgm"
                                    checked=RwSignal::new(true)
                                />
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
