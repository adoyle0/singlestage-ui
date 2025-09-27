use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TableExample() -> impl IntoView {
    view! {
        <Table class="overflow-x-auto w-full">
            <TableCaption>"A list of your recent invoices."</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead>"Invoice"</TableHead>
                    <TableHead>"Status"</TableHead>
                    <TableHead>"Method"</TableHead>
                    <TableHead class="text-right">"Amount"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell class="font-medium">"INV001"</TableCell>
                    <TableCell>"Paid"</TableCell>
                    <TableCell>"Credit Card"</TableCell>
                    <TableCell class="text-right">"$250.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV002"</TableCell>
                    <TableCell>"Pending"</TableCell>
                    <TableCell>"PayPal"</TableCell>
                    <TableCell class="text-right">"$150.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV003"</TableCell>
                    <TableCell>"Unpaid"</TableCell>
                    <TableCell>"Bank Transfer"</TableCell>
                    <TableCell class="text-right">"$350.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV004"</TableCell>
                    <TableCell>"Paid"</TableCell>
                    <TableCell>"Paypal"</TableCell>
                    <TableCell class="text-right">"$450.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV005"</TableCell>
                    <TableCell>"Paid"</TableCell>
                    <TableCell>"Credit Card"</TableCell>
                    <TableCell class="text-right">"$550.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV006"</TableCell>
                    <TableCell>"Pending"</TableCell>
                    <TableCell>"Bank Transfer"</TableCell>
                    <TableCell class="text-right">"$200.00"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell class="font-medium">"INV007"</TableCell>
                    <TableCell>"Unpaid"</TableCell>
                    <TableCell>"Credit Card"</TableCell>
                    <TableCell class="text-right">"$300.00"</TableCell>
                </TableRow>
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell colspan=3>"Total"</TableCell>
                    <TableCell class="text-right">"$2,500.00"</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
    }
}
