use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn PaginationIconsOnlyExample() -> impl IntoView {
    view! {
        <div class="flex items-center justify-between gap-4">
            <Field orientation="horizontal" class="w-fit">
                <Label>"Rows per page"</Label>
                <Select class="w-20" value="25">
                    <SelectOption value="10">"10"</SelectOption>
                    <SelectOption value="25">"25"</SelectOption>
                    <SelectOption value="50">"50"</SelectOption>
                    <SelectOption value="100">"100"</SelectOption>
                </Select>
            </Field>
            <Pagination class="mx-0 w-auto">
                <PaginationContent>
                    <PaginationItem>
                        <PaginationPrevious href="#" />
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationNext href="#" />
                    </PaginationItem>
                </PaginationContent>
            </Pagination>
        </div>
    }
}
