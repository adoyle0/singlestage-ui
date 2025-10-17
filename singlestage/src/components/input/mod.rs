use crate::Reactive;
use leptos::prelude::*;

#[component]
pub fn Input(
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

    // INPUT ATTRIBUTES
    //
    /// Defines which file types are selectable. Only works with the 'file' input type.
    #[prop(optional, into)]
    accept: MaybeProp<String>,
    /// Fallback string used with `image` input type. Renders while the image isn't loaded.
    #[prop(optional, into)]
    alt: MaybeProp<String>,
    /// Controls [autocomplete](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete).
    ///
    /// Works with `color`, `date`, `datetime-local`, `email`, `hidden`, `month`, `number`,
    /// `password`, `range`, `search`, `tel`, `text`, `time`, `url`, and `week`.
    #[prop(optional, into)]
    autocomplete: MaybeProp<String>,
    /// Specify which camera to use for capture of an image or video. Use `accept` to define the
    /// expected data type.
    #[prop(optional, into)]
    capture: MaybeProp<String>,
    /// Submits the user's text directionality (`ltr`, `rtl`) set by the browser along with the
    /// regular form data. Works with `email`, `hidden`, `search`, `tel`, `text`, `url`.
    #[prop(optional, into)]
    dirname: MaybeProp<String>,
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
    /// Set the height of the image defined by `src` when `input_type` is set to "image".
    #[prop(optional, into)]
    height: MaybeProp<usize>,
    /// Link this input with a `<datalist>` element for value suggestions with its `id`.
    #[prop(optional, into)]
    list: MaybeProp<String>,
    /// The latest date the form will accept in `yyyy-mm-dd` format.
    #[prop(optional, into)]
    max: MaybeProp<String>,
    /// Set the maximum length of accepted input.
    #[prop(optional, into)]
    maxlength: MaybeProp<usize>,
    /// The earliest date the form will accept in `yyyy-mm-dd` format.
    #[prop(optional, into)]
    min: MaybeProp<String>,
    /// Set the minimum length of accepted input.
    #[prop(optional, into)]
    minlength: MaybeProp<usize>,
    /// Toggle accepting multiple input values for certain input types.
    #[prop(optional, into)]
    multiple: MaybeProp<bool>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// A regular expression that the input's value must match to pass validation.
    #[prop(optional, into)]
    pattern: MaybeProp<String>,
    /// Text that shows while there is no value set.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Toggle whether or not the user can modify the value of this element.
    #[prop(optional, into)]
    readonly: MaybeProp<bool>,
    /// Toggle whether or not this element requires a value for form submission.
    #[prop(optional, into)]
    required: MaybeProp<bool>,
    /// Set how many characters wide the field should be. Defaults to `20`.
    #[prop(optional, into)]
    size: MaybeProp<usize>,
    /// Define the URL of the image to display when `input_type` is set to "image".
    #[prop(optional, into)]
    src: MaybeProp<String>,
    /// Define the granularity of expected input value.
    #[prop(optional, into)]
    step: MaybeProp<String>,
    /// Define the width of the image defined by `src` when `input_type` is set to "image".
    #[prop(optional, into)]
    width: MaybeProp<usize>,

    /// Sets the default value of the element. Setting `value` sets this once at page load.
    /// Use this for subsequent updates.
    #[prop(optional, into)]
    default: MaybeProp<String>,
    /// Set the input type. Supported types: `color`, `date`, `datetime-local`, `email`, `file`,
    /// `hidden`, `image`, `month`, `number`, `password`, `search`, `tel`, `text`, `time`, `url`,
    /// `week`.
    #[prop(optional, into)]
    input_type: MaybeProp<String>,
    /// Toggle invalid appearance.
    #[prop(optional, into)]
    invalid: MaybeProp<bool>,
    /// The reactive value signal of this input. Also sets initial `default` value, but doesn't
    /// update it.
    #[prop(optional, into)]
    value: Reactive<String>,
) -> impl IntoView {
    let supported_types = [
        "color",
        "date",
        "datetime-local",
        "email",
        "file",
        "hidden",
        "image",
        "month",
        "number",
        "password",
        "search",
        "tel",
        "text",
        "time",
        "url",
        "week",
    ];

    let input_ref = NodeRef::<leptos::html::Input>::new();

    Effect::new(move || {
        if let Some(input) = input_ref.get_untracked()
            && let Some(default) = default.get()
        {
            input.set_default_value(&default);
        }
    });

    Effect::new(move || {
        if let Some(input) = input_ref.get_untracked()
            && let Some(disabled) = disabled.get()
        {
            input.set_disabled(disabled);
        }
    });

    Effect::new(move || {
        if let Some(input) = input_ref.get_untracked() {
            input.set_value(&value.get());
        }
    });

    let on_input = move |ev| {
        value.set(event_target_value(&ev));
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
            role=move || role.get()
            slot=move || slot.get()
            spellcheck=move || spellcheck.get()
            style=move || style.get()
            tabindex=move || tabindex.get()
            title=move || title.get()
            translate=move || translate.get()
        />
    };

    let input_attrs_1 = view! {
        <{..}
            accept=move || accept.get()
            alt=move || alt.get()
            autocomplete=move || autocomplete.get()
            capture=move || capture.get()
            dirname=move || dirname.get()
            form=move || form.get()
            formaction=move || formaction.get()
            formenctype=move || formenctype.get()
            formmethod=move || formmethod.get()
            formnovalidate=move || formnovalidate.get()
            formtarget=move || formtarget.get()
            height=move || height.get()
            list=move || list.get()
        />
    };

    let input_attrs_2 = view! {
        <{..}
            max=move || max.get()
            maxlength=move || maxlength.get()
            min=move || min.get()
            minlength=move || minlength.get()
            multiple=move || multiple.get()
            name=move || name.get()
            pattern=move || pattern.get()
            placeholder=move || placeholder.get()
            readonly=move || readonly.get()
            required=move || required.get()
            size=move || size.get()
            src=move || src.get()
            step=move || step.get()
            width=move || width.get()
        />
    };

    let custom_attrs = view! {
        <{..}
            aria-invalid=move || {
                match invalid.get() {
                    Some(true) => "true",
                    _ => "",
                }
            }
            class=move || { format!("singlestage-input {}", class.get().unwrap_or_default()) }
            disabled=move || disabled.get()
            node_ref=input_ref
            on:input=on_input
            type=move || {
                let input_type = input_type.get().unwrap_or_default();
                if supported_types.contains(&input_type.as_str()) {
                    input_type
                } else {
                    "text".to_string()
                }
            }
            value=value.get_untracked()
        />
    };

    view! {
        {if let Some(children) = children {
            let uuid = uuid::Uuid::new_v4();

            view! {
                <label
                    class="singlestage-label singlestage-input-label"
                    for=move || id.get().unwrap_or(uuid.to_string())
                >
                    {children()}
                </label>
                <input
                    id=move || id.get().unwrap_or(uuid.to_string())
                    {..global_attrs_1}
                    {..global_attrs_2}
                    {..input_attrs_1}
                    {..input_attrs_2}
                    {..custom_attrs}
                />
            }
                .into_any()
        } else {

            view! {
                <input
                    id=move || id.get()
                    {..global_attrs_1}
                    {..global_attrs_2}
                    {..input_attrs_1}
                    {..input_attrs_2}
                    {..custom_attrs}
                />
            }
                .into_any()
        }}
    }
}
