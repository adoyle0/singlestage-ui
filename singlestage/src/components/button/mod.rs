use crate::{
    DropdownMenuContext, DropdownTriggerContext, InputGroupContext, PopoverContext,
    PopoverTriggerContext,
};
use leptos::prelude::*;

/// Creates a button.
#[component]
pub fn Button(
    children: Children,

    /// The type of the button. Defaults to `submit`:
    /// Button types: submit | button | reset
    #[prop(optional, into)]
    button_type: MaybeProp<String>,
    /// The size of the button. Leave this empty for the default size.
    /// Sizes: small | large | icon | sm-icon | lg-icon
    #[prop(optional, into)]
    size: MaybeProp<String>,
    /// The display variant of the button. Defaults to `primary`
    /// Variants: primary | secondary | outline | ghost | link | destructive
    #[prop(optional, into)]
    variant: MaybeProp<String>,

    // BUTTON ATTRIBUTES
    //
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
    /// The value associated with this button's `name` when submitted with form data.
    #[prop(optional, into)]
    value: MaybeProp<String>,

    // ARIA ATTRIBUTES
    //
    /// Provide a custom accessible name for this element.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,

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

    let button_attrs = view! {
        <{..}
            command=move || command.get()
            commandfor=move || commandfor.get()
            form=move || form.get()
            formaction=move || formaction.get()
            formenctype=move || formenctype.get()
            formmethod=move || formmethod.get()
            formnovalidate=move || formnovalidate.get()
            formtarget=move || formtarget.get()
            name=move || name.get()
            value=move || value.get()
        />
    };

    let button_is_trigger: bool = use_context::<DropdownTriggerContext>().is_some()
        || use_context::<PopoverTriggerContext>().is_some();

    view! {
        <button
            aria_controls=move || {
                if button_is_trigger {
                    if let Some(dropdown) = use_context::<DropdownMenuContext>() {
                        Some(dropdown.menu_id.get())
                    } else { use_context::<PopoverContext>().map(|popover| popover.menu_id.get()) }
                } else {
                    None
                }
            }
            aria_haspopup=move || { if button_is_trigger { Some("menu") } else { None } }
            aria_label=move || aria_label.get()
            class=move || {
                format!(
                    "{}{} {} {} {}",
                    if button_is_trigger { "singlestage-trigger " } else { "" },
                    match variant.get().unwrap_or_default().as_str() {
                        "primary" => "singlestage-btn-primary",
                        "secondary" => "singlestage-btn-secondary",
                        "outline" => "singlestage-btn-outline",
                        "ghost" => "singlestage-btn-ghost",
                        "link" => "singlestage-btn-link",
                        "destructive" => "singlestage-btn-destructive",
                        _ => {
                            if use_context::<InputGroupContext>().is_some()
                                && variant.get().is_none()
                            {
                                "singlestage-btn-ghost"
                            } else {
                                "singlestage-btn-primary"
                            }
                        }
                    },
                    match size.get().unwrap_or_default().as_str() {
                        "sm" => "singlestage-btn-sm",
                        "small" => "singlestage-btn-sm",
                        "lg" => "singlestage-btn-lg",
                        "large" => "singlestage-btn-lg",
                        "icon" => "singlestage-btn-icon",
                        "sm-icon" => "singlestage-btn-sm-icon",
                        "icon-sm" => "singlestage-btn-sm-icon",
                        "lg-icon" => "singlestage-btn-lg-icon",
                        "icon-lg" => "singlestage-btn-lg-icon",
                        _ => "",
                    },
                    if use_context::<InputGroupContext>().is_some() {
                        format!(
                            "singlestage-input-group-button {}",
                            match size.get().unwrap_or_default().as_str() {
                                "sm" => "singlestage-input-group-button-sm",
                                "icon-xs" => "singlestage-input-group-button-icon-xs",
                                "icon-sm" => "singlestage-input-group-button-icon-sm",
                                _ => "singlestage-input-group-button-xs",
                            },
                        )
                    } else {
                        "".to_string()
                    },
                    class.get().unwrap_or_default(),
                )
            }
            disabled=disabled.get_untracked()
            id=move || {
                if button_is_trigger {
                    let trigger_id = id.get().unwrap_or(uuid::Uuid::new_v4().to_string());
                    if let Some(dropdown) = use_context::<DropdownMenuContext>() {
                        dropdown.trigger_id.set(trigger_id.clone());
                    } else if let Some(popover) = use_context::<PopoverContext>() {
                        popover.trigger_id.set(trigger_id.clone());
                    }
                    Some(trigger_id)
                } else {
                    id.get()
                }
            }
            popovertarget=move || {
                if button_is_trigger {
                    let mut target_id = None;
                    if let Some(popovertarget) = popovertarget.get() {
                        target_id = Some(popovertarget);
                    } else if let Some(dropdown) = use_context::<DropdownMenuContext>() {
                        target_id = Some(dropdown.menu_id.get())
                    } else if let Some(popover) = use_context::<PopoverContext>() {
                        target_id = Some(popover.menu_id.get())
                    }
                    target_id
                } else {
                    popovertarget.get()
                }
            }
            popovertargetaction=move || {
                if button_is_trigger {
                    Some("toggle".to_string())
                } else {
                    popovertargetaction.get()
                }
            }
            prop:disabled=move || disabled.get()
            type=move || {
                if let Some(button_type) = button_type.get() {
                    Some(button_type)
                } else if use_context::<InputGroupContext>().is_some() {
                    Some("button".to_string())
                } else {
                    None
                }
            }

            {..global_attrs_1}
            {..global_attrs_2}
            {..button_attrs}
        >
            {children()}
        </button>
    }
}
