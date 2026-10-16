use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TableExample() -> impl IntoView {
    #[derive(Clone)]
    struct Invoice {
        invoice: String,
        payment_status: String,
        total_amount: String,
        payment_method: String,
    }

    let invoices = [
        Invoice {
            invoice: "INV001".to_string(),
            payment_status: "Paid".to_string(),
            total_amount: "$250.00".to_string(),
            payment_method: "Credit Card".to_string(),
        },
        Invoice {
            invoice: "INV002".to_string(),
            payment_status: "Pending".to_string(),
            total_amount: "$150.00".to_string(),
            payment_method: "PayPal".to_string(),
        },
        Invoice {
            invoice: "INV003".to_string(),
            payment_status: "Unpaid".to_string(),
            total_amount: "$350.00".to_string(),
            payment_method: "Bank Transfer".to_string(),
        },
        Invoice {
            invoice: "INV004".to_string(),
            payment_status: "Paid".to_string(),
            total_amount: "$450.00".to_string(),
            payment_method: "Credit Card".to_string(),
        },
        Invoice {
            invoice: "INV005".to_string(),
            payment_status: "Paid".to_string(),
            total_amount: "$550.00".to_string(),
            payment_method: "PayPal".to_string(),
        },
        Invoice {
            invoice: "INV006".to_string(),
            payment_status: "Pending".to_string(),
            total_amount: "$200.00".to_string(),
            payment_method: "Bank Transfer".to_string(),
        },
        Invoice {
            invoice: "INV007".to_string(),
            payment_status: "Unpaid".to_string(),
            total_amount: "$300.00".to_string(),
            payment_method: "Credit Card".to_string(),
        },
    ];

    view! {
        <Table class="overflow-x-auto w-full">
            <TableCaption>"A list of your recent invoices."</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-[100px]">"Invoice"</TableHead>
                    <TableHead>"Status"</TableHead>
                    <TableHead>"Method"</TableHead>
                    <TableHead class="text-right">"Amount"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || invoices.to_owned()
                    key=|invoice| invoice.invoice.clone()
                    let(invoice)
                >
                    <TableRow>
                        <TableCell class="font-medium">{invoice.invoice}</TableCell>
                        <TableCell>{invoice.payment_status}</TableCell>
                        <TableCell>{invoice.payment_method}</TableCell>
                        <TableCell class="text-right">{invoice.total_amount}</TableCell>
                    </TableRow>
                </For>
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
