use crate::{FieldContext, FieldLabel, RadioGroupContext, Reactive};
use leptos::prelude::*;

/// An item in the group that can be checked
#[component]
pub fn Radio(
    #[prop(optional)] children: Option<Children>,

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

    // RADIO ATTRIBUTES
    //
    /// Whether the command or control is checked
    #[prop(optional, into)]
    checked: Reactive<bool>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(optional, into)]
    form: MaybeProp<String>,
    // /// Name of this element. Submitted with the form as part of a name/value pair.
    // #[prop(optional, into)]
    // name: MaybeProp<String>,
    /// Toggle whether or not the user can modify the value of this element.
    #[prop(optional, into)]
    readonly: MaybeProp<bool>,
    /// Toggle whether or not this element requires a value for form submission.
    #[prop(optional, into)]
    required: MaybeProp<bool>,
    /// Whether the form control is disabled
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// The value of the control. When specified in the HTML, corresponds to the initial value
    #[prop(optional, into)]
    value: MaybeProp<String>,
) -> impl IntoView {
    let radio_group = expect_context::<RadioGroupContext>();
    let radio_ref = NodeRef::<leptos::html::Input>::new();

    let on_change = move |ev| {
        checked.set(event_target_checked(&ev));
        radio_group.value.set(event_target_value(&ev));
    };

    if let Some(value) = value.get_untracked()
        && radio_group.value.get_untracked() == value
    {
        if let Some(radio) = radio_ref.get_untracked() {
            radio.set_checked(true);
        }
        checked.set(true);
    }

    Effect::new(move || {
        if radio_group.value.get() == value.get().unwrap_or_default() {
            checked.set(true);
        } else {
            checked.set(false);
        }
    });

    Effect::new(move || {
        if let Some(radio) = radio_ref.get_untracked() {
            radio.set_checked(checked.get());
        }
    });

    Effect::new(move || {
        if let Some(radio) = radio_ref.get_untracked() {
            radio.set_disabled(disabled.get().unwrap_or_default());
        }
    });

    let global_attrs_1 = view! {
        <{..}
            accesskey=move || accesskey.get()
            autocapitalize=move || autocapitalize.get()
            autofocus=move || autofocus.get()
            // class=move || class.get()
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

    let input_id = uuid::Uuid::new_v4();
    let label_id = uuid::Uuid::new_v4();
    let has_children = children.is_some();

    let radio_attrs = view! {
        <{..}
            aria_describedby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    let description_id = field.description_id.get();
                    if description_id.is_empty() { None } else { Some(description_id) }
                } else {
                    None
                }
            }
            aria-invalid=move || {
                if let Some(radio_group) = use_context::<RadioGroupContext>() {
                    radio_group.invalid.get().to_string()
                } else {
                    false.to_string()
                }
            }
            aria_labelledby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    Some(field.label_id.get())
                } else {
                    if has_children { Some(label_id.to_string()) } else { None }
                }
            }
            checked=move || checked.get_untracked()
            class=move || { format!("singlestage-input {}", class.get().unwrap_or_default()) }
            disabled=disabled.get_untracked()
            form=move || form.get()
            name=radio_group.name.clone()
            node_ref=radio_ref
            on:change=on_change
            readonly=move || readonly.get()
            required=move || required.get()
            type="radio"
            value=move || value.get()
        />
    };

    if let Some(children) = children {
        view! {
            {if use_context::<FieldContext>().is_some() {
                view! {
                    <input
                        id=move || id.get().unwrap_or(input_id.to_string())

                        {..global_attrs_1}
                        {..global_attrs_2}
                        {..radio_attrs}
                    />
                    <FieldLabel
                        class=class.get_untracked()
                        label_for=id.get_untracked().unwrap_or(input_id.to_string())
                    >
                        {children()}
                    </FieldLabel>
                }
                    .into_any()
            } else {
                view! {
                    <label
                        class=move || {
                            format!("singlestage-label {}", class.get().unwrap_or_default())
                        }
                        for=move || id.get().unwrap_or(input_id.to_string())
                        id=label_id.to_string()
                    >
                        <input
                            id=move || id.get().unwrap_or(input_id.to_string())

                            {..global_attrs_1}
                            {..global_attrs_2}
                            {..radio_attrs}
                        />
                        {children()}
                    </label>
                }
                    .into_any()
            }}
        }
        .into_any()
    } else {
        view! {
            <input
                id={if let Some(field) = use_context::<FieldContext>() {
                    if let Some(id) = id.get_untracked() {
                        field.input_id.set(id.clone());
                        Some(id)
                    } else {
                        field.input_id.set(input_id.to_string());
                        Some(input_id.to_string())
                    }
                } else {
                    id.get_untracked()
                }}

                {..global_attrs_1}
                {..global_attrs_2}
                {..radio_attrs}
            />
        }
        .into_any()
    }
}
