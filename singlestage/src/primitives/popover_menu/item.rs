use super::{MenuSubContext, PopoverMenuContext};
use crate::{Button, Checkbox, Radio, Reactive};
use leptos::{context::Provider, prelude::*};

#[derive(Clone, Copy)]
pub enum MenuItemPrimitiveType {
    Item,
    Checkbox,
    Radio,
    SubTrigger,
}

#[derive(Clone)]
pub(crate) struct MenuItemContext {
    pub dismiss: Reactive<bool>,
}

#[component]
pub fn MenuItemPrimitive(
    primitive_type: MenuItemPrimitiveType,

    children: Children,

    #[prop(optional, into)] checked: Reactive<bool>,

    /// This component will render without styling
    #[prop(optional, into)]
    as_child: MaybeProp<bool>,
    /// Controls whether the item appears disabled and is clickable.
    #[prop(optional, into)]
    disabled: Reactive<bool>,
    /// Toggle whether clicking this item dismisses its parent menu
    #[prop(optional, into, default = Reactive::new(true))]
    dismiss: Reactive<bool>,
    /// Set whether or not this element should display inset from its normal position.
    #[prop(optional, into)]
    inset: MaybeProp<bool>,
    /// Set the display variant of the item.
    ///
    /// Accepted values: "destructive"
    #[prop(optional, into)]
    variant: MaybeProp<String>,

    // LI ATTRIBRUTES
    //
    /// The current ordinal value of the item.
    #[prop(into)]
    value: MaybeProp<String>,

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
            slot=move || slot.get()
            spellcheck=move || spellcheck.get()
            style=move || style.get()
            tabindex=move || tabindex.get()
            title=move || title.get()
            translate=move || translate.get()
        />
    };

    view! {
        <li
            class=move || {
                format!(
                    "{}{}{}{} {}",
                    match primitive_type {
                        MenuItemPrimitiveType::SubTrigger => "singlestage-dropdown-menu-sub-trigger",
                        _ => "singlestage-dropdown-menu-item",
                    },
                    if disabled.get() {
                        " singlestage-dropdown-menu-item-variant-disabled"
                    } else {
                        ""
                    },
                    if inset.get().unwrap_or_default() {
                        " singlestage-dropdown-menu-inset"
                    } else {
                        ""
                    },
                    match variant.get().unwrap_or_default().as_str() {
                        "destructive" => " singlestage-dropdown-menu-item-variant-destructive",
                        _ => "",
                    },
                    class.get().unwrap_or_default(),
                )
            }
            on:click=move |_| {
                match primitive_type {
                    MenuItemPrimitiveType::Checkbox | MenuItemPrimitiveType::Radio => {
                        if dismiss.get() {
                            if let Some(menu) = use_context::<PopoverMenuContext>()
                                && !menu.modal.get()
                            {
                                menu.open.set(false);
                            } else if let Some(menu) = use_context::<PopoverMenuContext>() {
                                menu.open.set(false);
                            }
                        }
                    }
                    MenuItemPrimitiveType::Item => {
                        if as_child.get().unwrap_or_default() && dismiss.get() {
                            if let Some(menu) = use_context::<PopoverMenuContext>()
                                && !menu.modal.get()
                            {
                                menu.open.set(false);
                            }
                        }
                    }
                    _ => {}
                }
            }

            role="menuitem"
            value=move || value.get()

            {..global_attrs_1}
            {..global_attrs_2}
        >
            {match primitive_type {
                MenuItemPrimitiveType::Item => {
                    view! {
                        <Provider value=MenuItemContext {
                            dismiss,
                        }>
                            {if as_child.get().unwrap_or_default() {
                                children().into_any()
                            } else {
                                view! {
                                    <Button disabled id>
                                        {children()}
                                    </Button>
                                }
                                    .into_any()
                            }}
                        </Provider>
                    }
                        .into_any()
                }
                MenuItemPrimitiveType::Checkbox | MenuItemPrimitiveType::Radio => {
                    view! {
                        <label aria_disabled=move || {
                            if disabled.get() { Some("true") } else { None }
                        }>
                            {children()}
                            {match primitive_type {
                                MenuItemPrimitiveType::Checkbox => {
                                    view! {
                                        <Checkbox
                                            class="singlestage-dropdown-menu-checkbox-item"
                                            checked
                                            disabled
                                            id
                                        />
                                    }
                                        .into_any()
                                }
                                MenuItemPrimitiveType::Radio => {
                                    view! {
                                        <Radio
                                            class="singlestage-dropdown-menu-radio-item"
                                            checked
                                            disabled
                                            id
                                            value
                                        />
                                    }
                                        .into_any()
                                }
                                _ => view! {}.into_any(),
                            }}
                        </label>
                    }
                        .into_any()
                }
                MenuItemPrimitiveType::SubTrigger => {
                    let sub = expect_context::<MenuSubContext>();

                    view! {
                        <button
                            aria_controls=move || sub.menu_id.get()
                            aria_haspopup="menu"
                            id={
                                let trigger_id = id
                                    .get()
                                    .unwrap_or(uuid::Uuid::new_v4().to_string());
                                sub.trigger_id.set(trigger_id.clone());
                                trigger_id
                            }
                            popovertarget=move || sub.menu_id.get()
                            popovertargetaction="toggle"
                            style:anchor-name=move || format!("--{}", sub.trigger_id.get())
                            type="button"
                        >

                            {children()}
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
                                class="ml-auto"
                            >
                                <path d="m9 18 6-6-6-6" />
                            </svg>
                        </button>
                    }
                        .into_any()
                }
            }}
        </li>
    }
}
