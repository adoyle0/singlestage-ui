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
            <EmptyContent>
                <div class="flex gap-2">
                    <Button>"Create Project"</Button>
                    <Button variant="outline">"Import Project"</Button>
                </div>
            </EmptyContent>
            <a
                class="
                inline-flex
                items-center
                justify-center
                whitespace-nowrap
                text-sm
                font-medium
                transition-all
                disabled:pointer-events-none
                disabled:opacity-50
                [&_svg]:pointer-events-none
                [&_svg:not([class*='size-'])]:size-4
                shrink-0
                [&_svg]:shrink-0
                outline-none
                focus-visible:border-ring
                focus-visible:ring-ring/50
                focus-visible:ring-[3px]
                aria-invalid:ring-destructive/20
                dark:aria-invalid:ring-destructive/40
                aria-invalid:border-destructive
                underline-offset-4
                hover:underline
                h-8
                rounded-md
                gap-1.5
                px-3
                has-[&>svg]:px-2.5
                text-(--muted-foreground)
                "

                href="#"
            >
                "Learn More"
                {icon!(icondata::LuArrowUpRight)}
            </a>
        </Empty>
    }
}
