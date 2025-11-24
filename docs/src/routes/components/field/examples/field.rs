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
                                    <FieldLabel>"Year"</FieldLabel>
                                    <Select placeholder="YYYY">
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
