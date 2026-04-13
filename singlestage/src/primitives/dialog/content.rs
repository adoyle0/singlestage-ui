use super::DialogContext;
use crate::Button;
use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub(crate) struct DialogCloseContext {}

/// Contains content to be rendered in the main body of the dialog.
#[component]
pub fn DialogContentPrimitive(
    children: Children,

    /// Toggles whether or not a close button should appear in the top right corner of the dialog.
    ///
    /// Defaults to `true` for `Dialog` and `false` for `AlertDialog`
    #[prop(optional, into)]
    close_button: MaybeProp<bool>,
    /// Set the size of dialog to render.
    ///
    /// Accepted values are "sm"
    #[prop(optional, into)]
    size: MaybeProp<String>,

    // GLOBAL ATTRIBUTES
    //
    /// A space separated list of keys to focus this element. The first key available on the user's
    /// keyboard layout is used.
    #[prop(optional, into)]
    accesskey: MaybeProp<String>,
    /// Sets whether the input value should be capitalized and how. If a parent `<form>` has
    /// `autocapitalize` rules set, it will override any rules set here.
    ///
    /// Accepted values: "none" or "off" | "sentences" or "on" | "words" | "characters".
    #[prop(optional, into)]
    autocapitalize: MaybeProp<String>,
    /// Grabs focus once the page has finished loading. Only one element on the page can be focused
    /// at a time.
    #[prop(optional, into)]
    autofocus: MaybeProp<bool>,
    /// Apply classes to the element.
    #[prop(into)]
    class: MaybeProp<String>,
    /// Allows client-side editing of the element by the user.
    ///
    /// Accepted values: "true" | "false" | "plaintext-only"
    #[prop(optional, into)]
    contenteditable: MaybeProp<String>,
    /// Indicate directionality of the element's text.
    ///
    /// Accepted values: "ltr" | "rtl" | "auto"
    #[prop(optional, into)]
    dir: MaybeProp<String>,
    /// Toggle whether the element can be dragged.
    #[prop(optional, into)]
    draggable: MaybeProp<bool>,
    /// Modifies the appearance of the enter key on virtual keyboards.
    #[prop(optional, into)]
    enterkeyhint: MaybeProp<String>,
    /// Expose elements in the shadow DOM to be manipulated by the DOM.
    #[prop(optional, into)]
    exportparts: MaybeProp<String>,
    /// Controls hidden status of the element.
    #[prop(optional, into)]
    hidden: MaybeProp<String>,
    /// Set the id of this element.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Toggle if the browser reacts to input events from this element.
    #[prop(optional, into)]
    inert: MaybeProp<bool>,
    /// Hints to the browser of what type of virtual keyboard to display when editing this element
    /// or its children.
    #[prop(optional, into)]
    inputmode: MaybeProp<String>,
    /// Used to render a standard element as a custom element.
    #[prop(optional, into)]
    is: MaybeProp<String>,
    /// Unique global identifier of an item.
    #[prop(optional, into)]
    itemid: MaybeProp<String>,
    /// Used to add properties to an item.
    #[prop(optional, into)]
    itemprop: MaybeProp<String>,
    /// Used to associate an item with a related non-parent element that's using `itemscope`.
    #[prop(optional, into)]
    itemref: MaybeProp<String>,
    /// Used to declare that children elements are related to a particular item.
    #[prop(optional, into)]
    itemscope: MaybeProp<String>,
    /// URL of data used to define `itemprops`.
    #[prop(optional, into)]
    itemtype: MaybeProp<String>,
    /// Defines the language of an element.
    #[prop(optional, into)]
    lang: MaybeProp<String>,
    /// Cryptographic "number used once".
    #[prop(optional, into)]
    nonce: MaybeProp<String>,
    /// List of the part names of the element.
    #[prop(optional, into)]
    part: MaybeProp<String>,
    /// Designate an element as a popover element.
    #[prop(optional, into)]
    popover: MaybeProp<String>,
    /// Define the semantic meaning of content.
    #[prop(optional, into)]
    role: MaybeProp<String>,
    /// Assigns a slot to an element.
    #[prop(optional, into)]
    slot: MaybeProp<String>,
    /// Toggle spellcheck for this input.
    ///
    /// Accepted values: "default" | "true" | "false".
    #[prop(optional, into)]
    spellcheck: MaybeProp<String>,
    /// Define CSS to be applied to the element.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Controls how an element behaves when a user navigates using the tab key.
    #[prop(optional, into)]
    tabindex: MaybeProp<usize>,
    /// Describes the content of the element to screen readers.
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Defines localization behavior for the element.
    #[prop(optional, into)]
    translate: MaybeProp<String>,
) -> impl IntoView {
    let dialog_context = expect_context::<DialogContext>();
    let dialog_ref = NodeRef::<leptos::html::Dialog>::new();

    Effect::new(move || {
        if let Some(dialog) = dialog_ref.get() {
            match dialog_context.open.get() {
                true => {
                    let _ = dialog.show_modal();
                }
                false => {
                    dialog.close();
                }
            }
        }
    });

    let global_attrs_1 = view! {
        <{..}
            accesskey=move || accesskey.get()
            autocapitalize=move || autocapitalize.get()
            autofocus=move || autofocus.get()
            contenteditable=move || contenteditable.get()
            dir=move || dir.get()
            draggable=move || draggable.get()
            enterkeyhint=move || enterkeyhint.get()
            exportparts=move || exportparts.get()
            hidden=move || hidden.get()
            id=move || id.get()
            inert=move || inert.get()
            inputmode=move || inputmode.get()
            is=move || is.get()
            itemid=move || itemid.get()
        />
    };

    let global_attrs_2 = view! {
        <{..}
            itemprop=move || itemprop.get()
            itemref=move || itemref.get()
            itemscope=move || itemscope.get()
            itemtype=move || itemtype.get()
            lang=move || lang.get()
            nonce=move || nonce.get()
            part=move || part.get()
            popover=move || popover.get()
            role=move || role.get()
            slot=move || slot.get()
            spellcheck=move || spellcheck.get()
            style=move || style.get()
            tabindex=move || tabindex.get()
            title=move || title.get()
            translate=move || translate.get()
        />
    };

    view! {
        <div class="singlestage-dialog">
            <dialog
                aria_describedby=move || dialog_context.described_by.get()
                aria_labelledby=move || dialog_context.labelled_by.get()
                aria_modal="true"
                class=move || {
                    format!(
                        "singlestage-dialog-content{} {}",
                        match size.get().unwrap_or_default().as_str() {
                            "sm" | "small" => " singlestage-dialog-size-sm",
                            _ => "",
                        },
                        class.get().unwrap_or_default(),
                    )
                }
                node_ref=dialog_ref
                on:click=move |ev| {
                    if let Some(dialog) = dialog_ref.get_untracked() {
                        let x = ev.x() as f64;
                        let y = ev.y() as f64;
                        let r = dialog.get_bounding_client_rect();
                        let r_clicked = r.top() <= y && y <= (r.top() + r.height()) && r.left() <= x
                            && x <= (r.left() + r.width());
                        if !r_clicked && !dialog_context.alert {
                            dialog.close()
                        }
                    }
                }

                {..global_attrs_1}
                {..global_attrs_2}
            >
                {children()}
                <Show when=move || close_button.get().unwrap_or(!dialog_context.alert)>
                    <Provider value=DialogCloseContext {}>
                        <Button class="singlestage-dialog-close" variant="ghost" size="icon-sm">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-x-icon lucide-x"
                            >
                                <path d="M18 6 6 18" />
                                <path d="m6 6 12 12" />
                            </svg>
                            <span class="sr-only">"Close"</span>
                        </Button>
                    </Provider>
                </Show>
            </dialog>
        </div>
    }
}
