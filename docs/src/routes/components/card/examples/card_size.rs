use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn CardSizeExample() -> impl IntoView {
    view! {
        <Card size="sm" class="mx-auto w-full max-w-sm">
            <CardHeader>
                <CardTitle>"Small Card"</CardTitle>
                <CardDescription>"This card uses the small size variant."</CardDescription>
            </CardHeader>
            <CardContent>
                <p>
                    "The card component supports a size prop that can be set to
                    \"sm\" for a more compact appearance."
                </p>
            </CardContent>
            <CardFooter>
                <Button variant="outline" size="sm" class="w-full">
                    "Action"
                </Button>
            </CardFooter>
        </Card>
    }
}
