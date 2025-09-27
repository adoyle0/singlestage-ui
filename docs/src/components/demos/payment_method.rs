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
                                <option value="01">January</option>
                                <option value="02">February</option>
                                <option value="03">March</option>
                                <option value="04">April</option>
                                <option value="05">May</option>
                                <option value="06">June</option>
                                <option value="07">July</option>
                                <option value="08">August</option>
                                <option value="09">September</option>
                                <option value="10">October</option>
                                <option value="11">November</option>
                                <option value="12">December</option>
                            </Select>
                        </div>
                        <div class="grid gap-2 flex-1">
                            <Label label_for="payment-method-expiration-year">"Year"</Label>
                            <Select id="payment-method-expiration-year" class="w-full">
                                <option value="2024">"2024"</option>
                                <option value="2025">"2025"</option>
                                <option value="2026">"2026"</option>
                                <option value="2027">"2027"</option>
                                <option value="2028">"2028"</option>
                                <option value="2029">"2029"</option>
                                <option value="2030">"2030"</option>
                                <option value="2031">"2031"</option>
                                <option value="2032">"2032"</option>
                                <option value="2033">"2033"</option>
                                <option value="2034">"2034"</option>
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
