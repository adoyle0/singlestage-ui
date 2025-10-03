use leptos::prelude::*;

#[component]
pub fn DialogClose(
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
    #[prop(optional, into)]
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

    // BUTTON ATTRIBUTES
    //
    /// Set the default behavior of the button.
    ///
    /// Accepted values: "submit" | "reset" | "button"
    #[prop(optional, into)]
    button_type: MaybeProp<String>,
    /// The action that's performed by the element this button controls.
    ///
    /// Accepted values: "show-modal" | "close" | "request-close" | "show-popover" | "hide-popover"
    /// | "toggle-popover" | "--[custom value]"
    #[prop(optional, into)]
    command: MaybeProp<String>,
    /// Turn this button into a command button for an element via id.
    #[prop(optional, into)]
    commandfor: MaybeProp<String>,
    /// Toggle whether or not the input is disabled.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(optional, into)]
    form: MaybeProp<String>,
    /// Defines the target for submitted form data. Overrides any parent `<form>` `action` values.
    #[prop(optional, into)]
    formaction: MaybeProp<String>,
    /// Defines the encoding type for submitted form data. Overrides any parent `<form>`
    /// `formenctype` values.
    ///
    /// Accepted values: "application/x-www-form-urlencoded" | "multipart/form-data" | "text/plain".
    #[prop(optional, into)]
    formenctype: MaybeProp<String>,
    /// Defines the HTTP method used to submit form data. Overrides any parent `<form>` `method`
    /// values.
    ///
    /// Accepted values: "get" | "post" | "dialog".
    #[prop(optional, into)]
    formmethod: MaybeProp<String>,
    /// Toggle whether the form data is validated or not before submission. Overrides any parent
    /// `<form>` `novalidate` values.
    #[prop(optional, into)]
    formnovalidate: MaybeProp<bool>,
    /// Defines where to display the response received after submission. Overrides any parent
    /// `<form>` `target` values.
    ///
    /// Accepted values: "_self" | "_blank" | "_parent" | "_top", or the `name` of any tab, window,
    /// or iframe
    #[prop(optional, into)]
    formtarget: MaybeProp<String>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Id of a popover to control.
    #[prop(optional, into)]
    popovertarget: MaybeProp<String>,
    /// The action to perform on the target popover.
    ///
    /// Accepted values: "hide" | "show" | "toggle"
    #[prop(optional, into)]
    popovertargetaction: MaybeProp<String>,
    /// The value associated with this button.
    #[prop(optional, into)]
    value: MaybeProp<String>,
) -> impl IntoView {
    let global_attrs_1 = view! {
        <{..}
            accesskey=move || accesskey.get()
            autocapitalize=move || autocapitalize.get()
            autofocus=move || autofocus.get()
            class=move || class.get()
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

    let button_attrs = view! {
        <{..}
            command=move || command.get()
            commandfor=move || commandfor.get()
            disabled=move || disabled.get()
            form=move || form.get()
            formaction=move || formaction.get()
            formenctype=move || formenctype.get()
            formmethod=move || formmethod.get()
            formnovalidate=move || formnovalidate.get()
            formtarget=move || formtarget.get()
            name=move || name.get()
            popovertarget=move || popovertarget.get()
            popovertargetaction=move || popovertargetaction.get()
            type=move || button_type.get()
            value=move || value.get()
        />
    };

    view! {
        <form method="dialog">
            <button
                aria-label="Close dialog"

                {..global_attrs_1}
                {..global_attrs_2}
                {..button_attrs}
            >
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
            </button>
        </form>
    }
}
