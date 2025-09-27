use leptos::prelude::*;
use singlestage::pagination::*;

#[component]
pub fn PaginationAnatomy() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious />
                    <PaginationLink />
                    <PaginationEllipsis />
                    <PaginationNext />
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
