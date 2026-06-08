use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn LabelsInFieldsExample() -> impl IntoView {
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
                                <Label>"Name on Card"</Label>
                                <Input placeholder="Evil Rabbit" />
                            </Field>
                            <Field>
                                <Label>"Card Number"</Label>
                                <Input placeholder="1234 5678 9012 3456" />
                                <FieldDescription>
                                    "Enter your 16-digit card number"
                                </FieldDescription>
                            </Field>
                            <div class="grid grid-cols-3 gap-4">
                                <Field>
                                    <Label>"Month"</Label>
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
                                    <Label>"Year"</Label>
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
                                    <Label>"CVV"</Label>
                                    <Input placeholder="123" />
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
                                <Checkbox checked=true />
                                <Label class="font-normal">"Same as shipping address"</Label>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSet>
                        <FieldGroup>
                            <Field>
                                <Label>"Comments"</Label>
                                <Textarea
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
