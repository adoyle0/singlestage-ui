use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn AlertDialogSmallWithMediaExample() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <Button variant="outline">"Show Dialog"</Button>
            </AlertDialogTrigger>
            <AlertDialogContent size="sm">
                <AlertDialogHeader>
                    <AlertDialogMedia>{icon!(icondata::LuBluetooth)}</AlertDialogMedia>
                    <AlertDialogTitle>"Allow accessory to connect?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "Do you want to allow the USB accessory to connect to this device?"
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>
                        <Button>"Don't allow"</Button>
                    </AlertDialogCancel>
                    <AlertDialogAction>
                        <Button>"Allow"</Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
}
