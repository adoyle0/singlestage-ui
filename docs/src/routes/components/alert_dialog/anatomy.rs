use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogAnatomy -> impl IntoView {
    view!{
        <AlertDialog>
            <AlertDialogTrigger>
                <Button />
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogMedia />
                    <AlertDialogTitle />
                    <AlertDialogDescription />
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel />
                    <AlertDialogAction />
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
