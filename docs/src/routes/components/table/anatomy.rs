use leptos::prelude::*;
use singlestage::table::*;

#[component]
pub fn TableAnatomy() -> impl IntoView {
    view! {
        <Table>
            <TableCaption />
            <TableHeader>
                <TableRow>
                    <TableHead />
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell />
                </TableRow>
            </TableBody>
            <TableFooter>
                <TableRow>
                    <TableCell />
                </TableRow>
            </TableFooter>
        </Table>
    }
}
