use leptos::prelude::*;

/// Used to wrap other elements and trigger the dialog on click.
#[slot]
pub struct DialogTrigger {
    children: ChildrenFn,
}
