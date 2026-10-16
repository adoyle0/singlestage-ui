use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PaginationSimpleExample() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
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
                    <PaginationLink href="#">"4"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="#">"5"</PaginationLink>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
