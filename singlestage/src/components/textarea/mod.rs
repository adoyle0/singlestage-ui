use crate::{FieldContext, FieldLabel, Reactive};
use leptos::prelude::*;

/// Creates a textarea that takes children as a default value.
#[component]
pub fn Textarea(
    #[prop(optional)] children: Option<Children>,

    /// Sets the default value of the element. Setting `value` sets this once at page load.
    /// Use this for subsequent updates.
    #[prop(optional, into)]
    default: MaybeProp<String>,
    /// Toggle whether or not the input is disabled.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// Toggle invalid appearance.
    #[prop(optional, into)]
    invalid: MaybeProp<bool>,
    /// The reactive value signal of this input. Also sets initial `default` value, but doesn't
    /// update it.
    #[prop(optional, into)]
    value: Reactive<String>,

    // LEPTOS ATTRIBUTES
    /// A reactive reference to a DOM node that can be used with the node_ref attribute.
    #[prop(optional, into)]
    node_ref: MaybeProp<NodeRef<leptos::html::Textarea>>,

    // TEXTAREA ATTRIBUTES
    //
    /// Controls [autocomplete](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete).
    ///
    /// Works with `color`, `date`, `datetime-local`, `email`, `hidden`, `month`, `number`,
    /// `password`, `range`, `search`, `tel`, `text`, `time`, `url`, and `week`.
    #[prop(optional, into)]
    autocomplete: MaybeProp<String>,
    /// Set the width of the `<textarea>`. Must be positive. The default value is `20`.
    #[prop(optional, into)]
    cols: MaybeProp<usize>,
    /// Submits the user's text directionality (`ltr`, `rtl`) set by the browser along with the
    /// regular form data.
    #[prop(optional, into)]
    dirname: MaybeProp<String>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(optional, into)]
    form: MaybeProp<String>,
    /// Set the maximum length of accepted input.
    #[prop(optional, into)]
    maxlength: MaybeProp<usize>,
    /// Set the minimum length of accepted input.
    #[prop(optional, into)]
    minlength: MaybeProp<usize>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Text that shows while there is no value set. Carriage returns and line-feeds are treated as
    /// line breaks.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Toggle whether or not the user can modify the value of this element.
    #[prop(optional, into)]
    readonly: MaybeProp<bool>,
    /// Set whether this element requires a value for form submission
    #[prop(optional, into)]
    required: MaybeProp<bool>,
    /// The number of visible text lines. Must be positive. Default is 2.
    #[prop(optional, into)]
    rows: MaybeProp<usize>,
    /// How the control should wrap the value. NOTE: Not all values are standard across browsers.
    ///
    /// Accepted values: "hard" | "soft" | "off".
    #[prop(optional, into)]
    wrap: MaybeProp<String>,

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
) -> impl IntoView {
    let textarea_ref = {
        if let Some(node_ref) = node_ref.get_untracked() {
            node_ref
        } else {
            NodeRef::<leptos::html::Textarea>::new()
        }
    };

    let on_input = move |ev| {
        value.set(event_target_value(&ev));
    };

    Effect::new(move || {
        if let Some(textarea) = textarea_ref.get_untracked()
            && let Some(default_value) = default.get()
        {
            let _ = textarea.set_default_value(&default_value);
        }
    });

    Effect::new(move || {
        if let Some(textarea) = textarea_ref.get_untracked()
            && let Some(disabled) = disabled.get()
        {
            textarea.set_disabled(disabled)
        }
    });

    Effect::new(move || {
        if let Some(textarea) = textarea_ref.get_untracked() {
            textarea.set_value(&value.get());
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

    let textarea_attrs = view! {
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
                match invalid.get() {
                    Some(true) => Some("true"),
                    _ => None,
                }
            }
            aria_labelledby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    Some(field.label_id.get())
                } else if has_children {
                    Some(label_id.to_string())
                } else {
                    None
                }
            }
            autocomplete=move || autocomplete.get()
            cols=move || cols.get()
            class=move || { format!("singlestage-textarea {}", class.get().unwrap_or_default()) }
            dirname=move || dirname.get()
            disabled=move || disabled.get()
            form=move || form.get()
            maxlength=move || maxlength.get()
            minlength=move || minlength.get()
            name=move || name.get()
            node_ref=textarea_ref
            on:input=on_input
            placeholder=move || placeholder.get()
            readonly=move || readonly.get()
            required=move || required.get()
            rows=move || rows.get()
            wrap=move || wrap.get()
        />
    };

    if let Some(children) = children {
        view! {
            {if use_context::<FieldContext>().is_some() {
                view! {
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
                            format!(
                                "singlestage-label singlestage-textarea-label {}",
                                class.get().unwrap_or_default(),
                            )
                        }
                        for=move || id.get().unwrap_or(input_id.to_string())
                        id=label_id.to_string()
                    >
                        {children()}
                    </label>
                }
                    .into_any()
            }}
            <textarea
                id=move || id.get().unwrap_or(input_id.to_string())
                {..global_attrs_1}
                {..global_attrs_2}
                {..textarea_attrs}
            >
                {if let Some(default) = default.get_untracked() {
                    default
                } else {
                    value.get_untracked()
                }}
            </textarea>
        }
        .into_any()
    } else {
        view! {
            <textarea
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
                {..textarea_attrs}
            >
                {if let Some(default) = default.get_untracked() {
                    default
                } else {
                    value.get_untracked()
                }}
            </textarea>
        }
        .into_any()
    }
}
