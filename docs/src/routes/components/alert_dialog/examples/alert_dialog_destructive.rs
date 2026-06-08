use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogDestructiveExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <Button variant="destructive">"Delete Chat"</Button>
            </AlertDialogTrigger>
            <AlertDialogContent size="sm">
                <AlertDialogHeader>
                    <AlertDialogMedia variant="destructive">
                        {icon!(icondata::LuTrash2)}
                    </AlertDialogMedia>
                    <AlertDialogTitle>"Delete chat?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This will permanently delete this chat conversation. View "
                        <a href="#">"Settings"</a>
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>
                        <Button variant="ghost">"Cancel"</Button>
                    </AlertDialogCancel>
                    <AlertDialogAction>
                        <Button variant="destructive">"Delete"</Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
