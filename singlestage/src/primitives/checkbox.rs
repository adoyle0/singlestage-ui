use crate::{CheckboxGroupContext, FieldContext, Label, RadioGroupContext, Reactive};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub enum CheckboxPrimitiveType {
    Checkbox,
    Switch,
    Radio,
}

#[component]
pub fn CheckboxPrimitive(
    primitive_type: CheckboxPrimitiveType,

    #[prop(optional)] children: Option<Children>,

    /// Whether the input is invalid
    #[prop(into)]
    invalid: Reactive<bool>,
    /// Set the size to render the Switch
    #[prop(optional, into)]
    size: MaybeProp<String>,

    // CHECKBOX ATTRIBUTES
    //
    /// Reactive signal coupled to the checkbox's checked value.
    #[prop(into)]
    checked: Reactive<bool>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(into)]
    form: MaybeProp<String>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(into)]
    name: MaybeProp<String>,
    /// Toggle whether or not the user can modify the value of this element.
    #[prop(into)]
    readonly: MaybeProp<bool>,
    /// Toggle whether or not this element requires a value for form submission.
    #[prop(into)]
    required: MaybeProp<bool>,
    /// Whether the form control is disabled
    #[prop(into)]
    disabled: Reactive<bool>,
    /// The value of the control. When specified in the HTML, corresponds to the initial value
    #[prop(into)]
    value: MaybeProp<String>,

    // LEPTOS ATTRIBUTES
    /// A reactive reference to a DOM node that can be used with the node_ref attribute.
    #[prop(into)]
    node_ref: MaybeProp<NodeRef<leptos::html::Input>>,

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
    // /// Define the semantic meaning of content.
    // #[prop(optional, into)]
    // role: MaybeProp<String>,
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
    let input_ref = {
        if let Some(node_ref) = node_ref.get_untracked() {
            node_ref
        } else {
            NodeRef::<leptos::html::Input>::new()
        }
    };

    let update_checked = move || {
        if let Some(value) = value.get() {
            match primitive_type {
                CheckboxPrimitiveType::Checkbox | CheckboxPrimitiveType::Switch => {
                    if let Some(checkbox_group) = use_context::<CheckboxGroupContext>() {
                        let is_checked = checkbox_group.value.get().contains(&value);
                        checked.set(is_checked);
                        is_checked
                    } else {
                        checked.get()
                    }
                }
                CheckboxPrimitiveType::Radio => {
                    if let Some(radio_group) = use_context::<RadioGroupContext>() {
                        let is_checked = radio_group.value.get() == value;
                        checked.set(is_checked);
                        is_checked
                    } else {
                        checked.get()
                    }
                }
            }
        } else {
            checked.get()
        }
    };

    let update_disabled = move || {
        if let Some(field) = use_context::<FieldContext>() {
            let is_disabled = field.disabled.get();
            disabled.set(is_disabled);
            is_disabled
        } else {
            disabled.get()
        }
    };

    let update_invalid = move || {
        if let Some(field) = use_context::<FieldContext>() {
            let is_invalid = field.invalid.get() || invalid.get();
            is_invalid
        } else {
            match primitive_type {
                CheckboxPrimitiveType::Checkbox | CheckboxPrimitiveType::Switch => {
                    if let Some(checkbox_group) = use_context::<CheckboxGroupContext>() {
                        let is_invalid = checkbox_group.invalid.get() || invalid.get();
                        is_invalid
                    } else {
                        invalid.get()
                    }
                }
                CheckboxPrimitiveType::Radio => {
                    if let Some(radio_group) = use_context::<RadioGroupContext>() {
                        let is_invalid = radio_group.invalid.get() || invalid.get();
                        is_invalid
                    } else {
                        invalid.get()
                    }
                }
            }
        }
    };

    let on_change = move |ev| {
        let input_checked = event_target_checked(&ev);

        checked.set(input_checked);

        match primitive_type {
            CheckboxPrimitiveType::Checkbox | CheckboxPrimitiveType::Switch => {
                if let Some(input_value) = value.get_untracked()
                    && let Some(checkbox_group) = use_context::<CheckboxGroupContext>()
                {
                    match input_checked {
                        true => checkbox_group.value.update(|group_value| {
                            group_value.push(input_value);
                        }),
                        false => checkbox_group.value.update(|group_value| {
                            if let Some(index) =
                                group_value.iter().position(|el| *el == input_value)
                            {
                                group_value.swap_remove(index);
                            }
                        }),
                    }
                }
            }
            CheckboxPrimitiveType::Radio => {
                if let Some(radio_group) = use_context::<RadioGroupContext>() {
                    radio_group.value.set(event_target_value(&ev));
                }
            }
        }
    };

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

    let input_attrs = view! {
        <{..}
            aria_checked=move || { if update_checked() { Some("true") } else { None } }
            aria_describedby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    let description_id = field.description_id.get();
                    if description_id.is_empty() { None } else { Some(description_id) }
                } else {
                    None
                }
            }
            aria_disabled=move || { if update_disabled() { Some("true") } else { None } }
            aria_invalid=move || { if update_invalid() { Some("true") } else { None } }
            aria_labelledby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    Some(field.label_id.get())
                } else if has_children {
                    Some(label_id.to_string())
                } else {
                    None
                }
            }
            checked=update_checked
            prop:checked=update_checked
            class=move || {
                format!(
                    "{} {}",
                    match primitive_type {
                        CheckboxPrimitiveType::Checkbox => {
                            if class
                                .get()
                                .unwrap_or_default()
                                .contains("singlestage-dropdown-menu-checkbox-item")
                            {
                                "".to_owned()
                            } else {
                                "singlestage-checkbox".to_owned()
                            }
                        }
                        CheckboxPrimitiveType::Switch => {
                            format!(
                                "singlestage-switch {}",
                                match size.get().unwrap_or_default().as_str() {
                                    "small" | "sm" => "singlestage-switch-size-sm",
                                    _ => "singlestage-switch-size-default",
                                },
                            )
                        }
                        CheckboxPrimitiveType::Radio => {
                            if class
                                .get()
                                .unwrap_or_default()
                                .contains("singlestage-dropdown-menu-radio-item")
                            {
                                "".to_owned()
                            } else {
                                "singlestage-radio".to_owned()
                            }
                        }
                    },
                    class.get().unwrap_or_default(),
                )
            }
            disabled=update_disabled
            prop:disabled=update_disabled
            form=move || form.get()
            id={if let Some(field) = use_context::<FieldContext>() {
                if let Some(id) = id.get_untracked() {
                    field.input_id.set(id.clone());
                    Some(id)
                } else {
                    field.input_id.set(input_id.to_string());
                    Some(input_id.to_string())
                }
            } else if let Some(id) = id.get_untracked() {
                Some(id)
            } else {
                if has_children { Some(input_id.to_string()) } else { None }
            }}
            name=move || {
                if let Some(name) = name.get() {
                    Some(name)
                } else {
                    match primitive_type {
                        CheckboxPrimitiveType::Checkbox | CheckboxPrimitiveType::Switch => {
                            if let Some(checkbox_group) = use_context::<CheckboxGroupContext>() {
                                Some(format!("{}[]", checkbox_group.name.clone()))
                            } else {
                                None
                            }
                        }
                        CheckboxPrimitiveType::Radio => {
                            if let Some(radio_group) = use_context::<RadioGroupContext>() {
                                Some(radio_group.name.clone())
                            } else {
                                None
                            }
                        }
                    }
                }
            }
            node_ref=input_ref
            on:change=on_change
            readonly=move || readonly.get()
            required=move || required.get()
            role=match primitive_type {
                CheckboxPrimitiveType::Switch => Some("switch"),
                _ => None,
            }
            type=match primitive_type {
                CheckboxPrimitiveType::Radio => "radio",
                _ => "checkbox",
            }
            value=move || value.get()
            prop:value=move || value.get()
        />
    };

    if let Some(children) = children {
        view! {
            <Label
                class
                disabled
                id=label_id.to_string()
                invalid
                label_for=id.get_untracked().unwrap_or(input_id.to_string())
            >
                <input

                    {..global_attrs_1}
                    {..global_attrs_2}
                    {..input_attrs}
                />
                {children()}
            </Label>
        }
        .into_any()
    } else {
        view! {
            <input

                {..global_attrs_1}
                {..global_attrs_2}
                {..input_attrs}
            />
        }
        .into_any()
    }
}
