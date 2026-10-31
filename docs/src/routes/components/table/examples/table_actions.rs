use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn TableActionsExample() -> impl IntoView {
    #[derive(Clone)]
    struct Product {
        name: String,
        price: String,
    }

    let products = [
        Product {
            name: "Wireless Mouse".to_string(),
            price: "$29.99".to_string(),
        },
        Product {
            name: "Mechanical Keyboard".to_string(),
            price: "$129.99".to_string(),
        },
        Product {
            name: "USB-C Hub".to_string(),
            price: "$49.99".to_string(),
        },
    ];

    view! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>"Product"</TableHead>
                    <TableHead>"Price"</TableHead>
                    <TableHead class="text-right">"Actions"</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || products.to_owned()
                    key=|product| product.name.clone()
                    let(product)
                >
                    <TableRow>
                        <TableCell class="font-medium">{product.name}</TableCell>
                        <TableCell>{product.price}</TableCell>
                        <TableCell class="text-right">
                            <DropdownMenu>
                                <DropdownMenuTrigger>
                                    <Button variant="ghost" size="icon" class="size-8">
                                        {icon!(icondata::FiMoreHorizontal)}
                                        <span class="sr-only">"Open menu"</span>
                                    </Button>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent align="end">
                                    <DropdownMenuItem>"Edit"</DropdownMenuItem>
                                    <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem variant="destructive">
                                        "Delete"
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </TableCell>
                    </TableRow>
                </For>
            </TableBody>
        </Table>
    }
}
