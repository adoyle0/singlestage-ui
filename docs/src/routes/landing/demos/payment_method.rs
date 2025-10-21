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
                <form class="form grid gap-6">
                    <RadioGroup name="payment-method-type" class="grid-cols-3 gap-4">
                        <Label class="text-sm font-medium leading-none flex flex-col items-center justify-between rounded-md border-2 border-(--muted) p-4 hover:bg-(--muted) has-checked:border-(--primary) [&amp;&gt;svg]:mb-3 [&amp;&gt;svg]:size-6">
                            <Radio value="card" class="hidden" />
                            {icon!(icondata::FiCreditCard)}
                            "Card"
                        </Label>

                        <Label class="text-sm font-medium leading-none flex flex-col items-center justify-between rounded-md border-2 border-(--muted) p-4 hover:bg-(--muted) has-checked:border-(--primary) [&amp;&gt;svg]:mb-3 [&amp;&gt;svg]:size-6">
                            <Radio value="paypal" class="hidden" />
                            {icon!(icondata::FaPaypalBrands)}
                            "Paypal"
                        </Label>

                        <Label class="text-nowrap text-sm font-medium leading-none flex flex-col items-center justify-between rounded-md border-2 border-(--muted) p-4 hover:bg-(--muted) has-checked:border-(--primary) [&amp;&gt;svg]:mb-3 [&amp;&gt;svg]:size-6">
                            <Radio value="apple" class="hidden" />
                            {icon!(icondata::FaAppleBrands)}
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
                        <div class="grid gap-2 flex-1">
                            <Label label_for="payment-method-expiration-month">"Expires"</Label>
                            <Select id="payment-method-expiration-month" class="w-full">
                                <SelectItem value="01">"January"</SelectItem>
                                <SelectItem value="02">"February"</SelectItem>
                                <SelectItem value="03">"March"</SelectItem>
                                <SelectItem value="04">"April"</SelectItem>
                                <SelectItem value="05">"May"</SelectItem>
                                <SelectItem value="06">"June"</SelectItem>
                                <SelectItem value="07">"July"</SelectItem>
                                <SelectItem value="08">"August"</SelectItem>
                                <SelectItem value="09">"September"</SelectItem>
                                <SelectItem value="10">"October"</SelectItem>
                                <SelectItem value="11">"November"</SelectItem>
                                <SelectItem value="12">"December"</SelectItem>
                            </Select>
                        </div>
                        <div class="grid gap-2 flex-1">
                            <Label label_for="payment-method-expiration-year">"Year"</Label>
                            <Select id="payment-method-expiration-year" class="w-full">
                                <SelectItem value="2024">"2024"</SelectItem>
                                <SelectItem value="2025">"2025"</SelectItem>
                                <SelectItem value="2026">"2026"</SelectItem>
                                <SelectItem value="2027">"2027"</SelectItem>
                                <SelectItem value="2028">"2028"</SelectItem>
                                <SelectItem value="2029">"2029"</SelectItem>
                                <SelectItem value="2030">"2030"</SelectItem>
                                <SelectItem value="2031">"2031"</SelectItem>
                                <SelectItem value="2032">"2032"</SelectItem>
                                <SelectItem value="2033">"2033"</SelectItem>
                                <SelectItem value="2034">"2034"</SelectItem>
                            </Select>
                        </div>
                        <div class="grid gap-2 flex-1">
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
