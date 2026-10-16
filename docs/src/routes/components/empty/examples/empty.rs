use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyExample() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant="icon">{icon!(icondata::LuFolderCode)}</EmptyMedia>
                <EmptyTitle>"No Projects Yet"</EmptyTitle>
                <EmptyDescription>
                    "You haven't created any projects yet. Get started by creating
                    your first project."
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent class="flex-row justify-center gap-2">
                <Button>"Create Project"</Button>
                <Button variant="outline">"Import Project"</Button>
            </EmptyContent>
            <Link
                render_as="button"
                variant="link"
                size="sm"
                class="text-(--muted-foreground)"
                href="#"
            >
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </Link>
        </Empty>
    }
}
