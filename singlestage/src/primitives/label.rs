use crate::primitives::*;
use crate::{FieldContext, RadioGroupContext, Reactive};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub enum LabelPrimitiveType {
    Label,
    FieldLabel,
    MenuLabel,
}

#[component]
pub fn LabelPrimitive(
    primitive_type: LabelPrimitiveType,

    children: Children,

    /// Whether the element renders as disabled
    #[prop(into)]
    disabled: Reactive<bool>,
    /// Set whether or not this element should display inset from its normal position.
    /// (For use in popover menus)
    #[prop(optional, into)]
    inset: MaybeProp<bool>,
    /// Whether the element renders as invalid
    #[prop(into)]
    invalid: Reactive<bool>,
    /// The id of the labeled element if it's not a child
    #[prop(into)]
    label_for: MaybeProp<String>,

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
    #[prop(into)]
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
        <label
            aria_disabled=move || {
                if disabled.get() {
                    Some("true".to_string())
                } else {
                    if let Some(field) = use_context::<FieldContext>() && field.disabled.get() {
                        Some("true".to_string())
                    } else {
                        None
                    }
                }
            }
            aria_invalid=move || {
                if invalid.get() {
                    Some("true".to_string())
                } else {
                    if let Some(field) = use_context::<FieldContext>() && field.invalid.get() {
                        Some("true".to_string())
                    } else if let Some(radio_group) = use_context::<RadioGroupContext>()
                        && radio_group.invalid.get()
                    {
                        Some("true".to_string())
                    } else {
                        None
                    }
                }
            }

            // TODO: Do this with CSS.
            class=move || {
                format!(
                    "{} {}",
                    match primitive_type {
                        LabelPrimitiveType::Label => "singlestage-label".to_string(),
                        LabelPrimitiveType::FieldLabel => "singlestage-field-label".to_string(),
                        LabelPrimitiveType::MenuLabel => {
                            format!(
                                "singlestage-dropdown-menu-label{}",
                                if inset.get().unwrap_or_default() {
                                    " singlestage-dropdown-menu-inset"
                                } else {
                                    ""
                                },
                            )
                        }
                    },
                    class.get().unwrap_or_default(),
                )
            }

            for=move || {
                match primitive_type {
                    LabelPrimitiveType::FieldLabel => {
                        if let Some(field_context) = use_context::<FieldContext>() {
                            Some(field_context.input_id.get())
                        } else {
                            label_for.get()
                        }
                    }
                    _ => label_for.get(),
                }
            }

            id={match primitive_type {
                LabelPrimitiveType::MenuLabel => {
                    let label_id = id.get().unwrap_or(uuid::Uuid::new_v4().to_string());
                    let group = expect_context::<PopoverMenuGroupContext>();
                    group.heading_id.set(label_id.clone());
                    Some(label_id.to_owned())
                }
                LabelPrimitiveType::FieldLabel => {
                    if let Some(field_context) = use_context::<FieldContext>() {
                        let label_id = id.get().unwrap_or(uuid::Uuid::new_v4().to_string());
                        field_context.label_id.set(label_id.clone());
                        Some(label_id.to_owned())
                    } else {
                        id.get()
                    }
                }
                LabelPrimitiveType::Label => id.get(),
            }}

            {..global_attrs_1}
            {..global_attrs_2}
        >
            {children()}
        </label>
    }
}
