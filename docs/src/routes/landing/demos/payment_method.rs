use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PaymentMethod() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <h2>Payment Method</h2>
                <p>Add a new payment method to your account.</p>
            </CardHeader>
            <CardContent>
                <form class="grid gap-6 form">
                    <RadioGroup name="payment-method-type" class="grid-cols-3 gap-4">
                        <Label class="flex flex-col justify-between items-center p-4 text-sm font-medium leading-none rounded-md border-2 border-(--muted) has-checked:border-(--primary) hover:bg-(--muted)">
                            <Radio value="card" class="hidden" />
                            {icon!(icondata::FiCreditCard, class="mb-3 size-6")}
                            "Card"
                        </Label>

                        <Label class="flex flex-col justify-between items-center p-4 text-sm font-medium leading-none rounded-md border-2 border-(--muted) has-checked:border-(--primary) hover:bg-(--muted)">
                            <Radio value="paypal" class="hidden" />
                            {icon!(icondata::FaPaypalBrands, class="mb-3 size-6")}
                            "Paypal"
                        </Label>

                        <Label class="flex flex-col justify-between items-center p-4 text-sm font-medium leading-none rounded-md border-2 text-nowrap border-(--muted) has-checked:border-(--primary) hover:bg-(--muted)">
                            <Radio value="apple" class="hidden" />
                            {icon!(icondata::FaAppleBrands, class="mb-3 size-6")}
                            "Apple Pay"
                        </Label>
                    </RadioGroup>
                    <div class="grid gap-2">
                        <Label label_for="payment-method-name">Name</Label>
                        <Input id="payment-method-name" placeholder="John Doe" />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="payment-method-city">"City"</Label>
                        <Input id="payment-method-city" placeholder="New York" />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="payment-method-card-number">"Card Number"</Label>
                        <Input id="payment-method-card-number" placeholder="1234 5678 9012 3456" />
                    </div>
                    <div class="flex gap-4">
                        <div class="grid flex-1 gap-2">
                            <Label label_for="payment-method-expiration-month">"Expires"</Label>
                            <Select id="payment-method-expiration-month" class="w-full">
                                <SelectOption value="01">"January"</SelectOption>
                                <SelectOption value="02">"February"</SelectOption>
                                <SelectOption value="03">"March"</SelectOption>
                                <SelectOption value="04">"April"</SelectOption>
                                <SelectOption value="05">"May"</SelectOption>
                                <SelectOption value="06">"June"</SelectOption>
                                <SelectOption value="07">"July"</SelectOption>
                                <SelectOption value="08">"August"</SelectOption>
                                <SelectOption value="09">"September"</SelectOption>
                                <SelectOption value="10">"October"</SelectOption>
                                <SelectOption value="11">"November"</SelectOption>
                                <SelectOption value="12">"December"</SelectOption>
                            </Select>
                        </div>
                        <div class="grid flex-1 gap-2">
                            <Label label_for="payment-method-expiration-year">"Year"</Label>
                            <Select id="payment-method-expiration-year" class="w-full">
                                <SelectOption value="2024">"2024"</SelectOption>
                                <SelectOption value="2025">"2025"</SelectOption>
                                <SelectOption value="2026">"2026"</SelectOption>
                                <SelectOption value="2027">"2027"</SelectOption>
                                <SelectOption value="2028">"2028"</SelectOption>
                                <SelectOption value="2029">"2029"</SelectOption>
                                <SelectOption value="2030">"2030"</SelectOption>
                                <SelectOption value="2031">"2031"</SelectOption>
                                <SelectOption value="2032">"2032"</SelectOption>
                                <SelectOption value="2033">"2033"</SelectOption>
                                <SelectOption value="2034">"2034"</SelectOption>
                            </Select>
                        </div>
                        <div class="grid flex-1 gap-2">
                            <Label label_for="payment-method-cvv">"CVV"</Label>
                            <Input
                                id="payment-method-cvv"
                                placeholder="123"
                                maxlength=4
                                class="w-full"
                            />
                        </div>
                    </div>
                    <Button>"Continue"</Button>
                </form>
            </CardContent>
        </Card>
    }
}
