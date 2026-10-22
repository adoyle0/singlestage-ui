use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogMediaExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <Button variant="outline">"Share Project"</Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogMedia>{icon!(icondata::LuCircleFadingPlus)}</AlertDialogMedia>
                    <AlertDialogTitle>"Share this project?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "Anyone with the link will be able to view and edit this project."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>
                        <Button>"Cancel"</Button>
                    </AlertDialogCancel>
                    <AlertDialogAction>
                        <Button>"Share"</Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
