use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <Button variant="outline">"Show Dialog"</Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone. This will permanently delete your
                        account from our servers."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>
                        <Button>"Cancel"</Button>
                    </AlertDialogCancel>
                    <AlertDialogAction>
                        <Button>"Continue"</Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
