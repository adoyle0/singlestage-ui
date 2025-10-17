use crate::{CheckboxGroupContext, Reactive};
use leptos::prelude::*;

#[component]
pub fn Switch(
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

    // CHECKBOX ATTRIBUTES
    //
    /// Whether the command or control is checked
    #[prop(optional, into)]
    checked: Reactive<bool>,
    /// Associate this element with a form element that may not be its parent by its `id`.
    #[prop(optional, into)]
    form: MaybeProp<String>,
    /// Name of this element. Submitted with the form as part of a name/value pair.
    #[prop(optional, into)]
    name: MaybeProp<String>,
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
    let switch_ref = NodeRef::<leptos::html::Input>::new();

    let on_change = move |ev| {
        let switch_checked = event_target_checked(&ev);

        checked.set(switch_checked);

        if let Some(checkbox_value) = value.get_untracked()
            && let Some(checkbox_group) = use_context::<CheckboxGroupContext>()
        {
            match switch_checked {
                true => checkbox_group.value.update(|group_value| {
                    group_value.push(checkbox_value);
                }),
                false => checkbox_group.value.update(|group_value| {
                    if let Some(index) = group_value.iter().position(|el| *el == checkbox_value) {
                        group_value.swap_remove(index);
                    }
                }),
            }
        }
    };

    if let Some(value) = value.get_untracked()
        && let Some(checkbox_group) = use_context::<CheckboxGroupContext>()
        && checkbox_group.value.get_untracked().contains(&value)
        && let Some(switch) = switch_ref.get_untracked()
    {
        switch.set_checked(true)
    }

    Effect::new(move || {
        if let Some(checkbox_group) = use_context::<CheckboxGroupContext>()
            && let Some(value) = value.get_untracked()
        {
            if checkbox_group.value.get().contains(&value) {
                checked.set(true);
            } else {
                checked.set(false);
            }
        }
    });

    Effect::new(move || {
        if let Some(switch) = switch_ref.get_untracked() {
            switch.set_checked(checked.get());
        }
    });

    Effect::new(move || {
        if let Some(switch) = switch_ref.get_untracked()
            && let Some(disabled) = disabled.get()
        {
            switch.set_disabled(disabled);
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
            // role=move || role.get()
            slot=move || slot.get()
            spellcheck=move || spellcheck.get()
            style=move || style.get()
            tabindex=move || tabindex.get()
            title=move || title.get()
            translate=move || translate.get()
        />
    };

    view! {
        {if let Some(children) = children {
            view! {
                <label class="singlestage-label">
                    <input
                        checked=checked.get_untracked()
                        form=move || form.get()
                        name=move || name.get()
                        readonly=move || readonly.get()
                        required=move || required.get()
                        disabled=disabled.get_untracked()
                        class=move || {
                            format!("singlestage-input {}", class.get().unwrap_or_default())
                        }
                        node_ref=switch_ref
                        on:change=on_change
                        role="switch"
                        type="checkbox"
                        value=move || value.get()

                        {..global_attrs_1}
                        {..global_attrs_2}
                    />
                    {children()}
                </label>
            }
                .into_any()
        } else {
            view! {
                <input
                    checked=checked.get_untracked()
                    form=move || form.get()
                    name=move || name.get()
                    readonly=move || readonly.get()
                    required=move || required.get()
                    disabled=disabled.get_untracked()
                    class=move || format!("singlestage-input {}", class.get().unwrap_or_default())
                    node_ref=switch_ref
                    on:change=on_change
                    role="switch"
                    type="checkbox"
                    value=move || value.get()

                    {..global_attrs_1}
                    {..global_attrs_2}
                />
            }
                .into_any()
        }}
    }
}
