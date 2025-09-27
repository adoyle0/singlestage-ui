use leptos::prelude::*;
use singlestage::pagination::*;

#[component]
pub fn PaginationExample() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious href="">"Previous"</PaginationPrevious>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#" active=true>
                        "2"
                    </PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">"3"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="#">"Next"</PaginationNext>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
