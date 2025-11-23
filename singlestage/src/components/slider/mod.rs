use crate::{FieldContext, FieldLabel, Reactive};
use leptos::prelude::*;

#[component]
pub fn Slider(
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

    // RANGE ATTRIBUTES
    //
    /// Controls [autocomplete](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete).
    ///
    /// Works with `color`, `date`, `datetime-local`, `email`, `hidden`, `month`, `number`,
    /// `password`, `range`, `search`, `tel`, `text`, `time`, `url`, and `week`.
    #[prop(optional, into)]
    autocomplete: MaybeProp<String>,
    /// Toggle whether or not the input is disabled.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(optional, into)]
    form: MaybeProp<String>,
    /// Link this input with a `<datalist>` element for value suggestions with its `id`.
    #[prop(optional, into)]
    list: MaybeProp<String>,
    /// The greatest value in the range of permitted values.
    #[prop(optional, into)]
    max: MaybeProp<f64>,
    /// The lowest value in the range of permitted values.
    #[prop(optional, into)]
    min: MaybeProp<f64>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Define the granularity of expected input value.
    #[prop(optional, into)]
    step: MaybeProp<f64>,

    /// Sets the default value of the element. Setting `value` sets this once at page load.
    /// Use this for subsequent updates.
    #[prop(optional, into)]
    default: MaybeProp<f64>,
    /// The reactive value signal of this input. Also sets initial `default` value, but doesn't
    /// update it.
    #[prop(optional, into)]
    value: Reactive<f64>,
) -> impl IntoView {
    let slider_ref = NodeRef::<leptos::html::Input>::new();

    let max_default = 100.;
    let min_default = 0.;
    let step_default = 1.;

    let init_value = {
        if let Some(default_value) = default.get_untracked() {
            default_value
        } else {
            value.get_untracked()
        }
    };

    if let Some(slider) = slider_ref.get_untracked() {
        slider.set_value_as_number(init_value);
    };

    let slider_value = RwSignal::new({
        let max = max.get_untracked().unwrap_or(max_default);
        let min = min.get_untracked().unwrap_or(min_default);

        let percent = if max == min {
            0.
        } else {
            ((init_value - min) / (max - min)) * 100.
        };

        format!("{}%", &percent.to_string())
    });

    // On default
    Effect::new(move || {
        if let Some(default) = default.get()
            && let Some(slider) = slider_ref.get_untracked()
        {
            slider.set_default_value(&default.to_string());
        }
    });

    // On disabled
    Effect::new(move || {
        if let Some(slider) = slider_ref.get_untracked() {
            slider.set_disabled(disabled.get().unwrap_or_default());
        }
    });

    let update_slider = move |min: f64, max: f64, current_value: f64| {
        // Update slider
        let percent: f64 = if max == min {
            0.
        } else {
            ((current_value - min) / (max - min)) * 100.
        };
        slider_value.set(format!("{}%", &percent.to_string()));
    };

    let on_input = move |ev| {
        let target_value = event_target_value(&ev);
        if let Ok(mut current_value) = target_value.parse::<f64>() {
            let min = min.get_untracked().unwrap_or(min_default);
            let max = max.get_untracked().unwrap_or(max_default);

            if current_value < min {
                current_value = min
            };

            if current_value > max {
                current_value = max
            };

            value.set(current_value);

            update_slider(min, max, current_value);
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
            role=move || role.get()
            slot=move || slot.get()
            spellcheck=move || spellcheck.get()
            style=move || style.get()
            tabindex=move || tabindex.get()
            title=move || title.get()
            translate=move || translate.get()
        />
    };

    let range_attrs = view! {
        <{..}
            autocomplete=move || autocomplete.get()
            form=move || form.get()
            list=move || list.get()
            name=move || name.get()
        />
    };

    let update_max = move || {
        let current_value = value.get_untracked();
        let max = max.get().unwrap_or(max_default);
        let min = min.get_untracked().unwrap_or(min_default);

        if current_value > max {
            value.set(max)
        }

        update_slider(min, max, current_value);
        max
    };

    let update_min = move || {
        let current_value = value.get_untracked();
        let max = max.get_untracked().unwrap_or(max_default);
        let min = min.get().unwrap_or(min_default);

        if current_value < min {
            value.set(min)
        }

        update_slider(min, max, current_value);
        min
    };

    let update_prop_value = move || {
        let mut new_value = value.get();
        let max = max.get_untracked().unwrap_or(max_default);
        let min = min.get_untracked().unwrap_or(min_default);

        // Make sure value is in range
        if new_value > max {
            new_value = max;
            value.set(max);
        } else if new_value < min {
            new_value = min;
            value.set(min);
        }

        update_slider(min, max, new_value);
        new_value.to_string()
    };

    let update_step = move || {
        let current_value = value.get();
        let max = max.get_untracked().unwrap_or(max_default);
        let min = min.get_untracked().unwrap_or(min_default);
        let step = step.get().unwrap_or(step_default);
        update_slider(min, max, current_value);
        step
    };

    let input_id = uuid::Uuid::new_v4();
    let label_id = uuid::Uuid::new_v4();
    let has_children = children.is_some();

    let slider_attrs = view! {
        <{..}
            aria_describedby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    let description_id = field.description_id.get();
                    if description_id.is_empty() { None } else { Some(description_id) }
                } else {
                    None
                }
            }
            aria_labelledby=move || {
                if let Some(field) = use_context::<FieldContext>() {
                    Some(field.label_id.get())
                } else {
                    if has_children { Some(label_id.to_string()) } else { None }
                }
            }
            class=move || format!("singlestage-input {}", class.get().unwrap_or_default())
            disabled=disabled.get_untracked()
            max=update_max
            min=update_min
            node_ref=slider_ref
            on:input=on_input
            prop:value=update_prop_value
            step=update_step
            style:--slider-value=move || slider_value.get()
            type="range"
            value=init_value.to_string()
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
                                "singlestage-label singlestage-slider-label {}",
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
            <input
                id=move || id.get().unwrap_or(input_id.to_string())

                {..global_attrs_1}
                {..global_attrs_2}
                {..range_attrs}
                {..slider_attrs}
            />
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
                {..range_attrs}
                {..slider_attrs}
            />
        }
        .into_any()
    }
}
