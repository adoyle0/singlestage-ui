use crate::{BreadcrumbLinkContext, InputGroupContext, SheetContext, SidebarMenuButtonContext};
use leptos::prelude::*;
use leptos_router::components::A;

/// Creates a styled hyperlink.
#[component]
pub fn Link(
    children: Children,

    /// This component will render without styling
    #[prop(optional, into)]
    as_child: MaybeProp<bool>,

    /// Set whether or not this `Link` should appear as something else. This is similar to
    /// `asChild`.
    ///
    /// Accepted values: "button" | "badge"
    #[prop(optional, into)]
    render_as: MaybeProp<String>,
    /// For use with `render_as`, specify the size to render the element.
    #[prop(optional, into)]
    size: MaybeProp<String>,
    /// For use with `render_as`, specify the variant of the element to render.
    #[prop(optional, into)]
    variant: MaybeProp<String>,

    // A ATTRIBUTES
    //
    /// Causes the browser to treat the linked URL as a download. Can be used with or without a
    /// `filename` value.
    #[prop(optional, into)]
    download: MaybeProp<String>,
    /// The URL the link points to.
    #[prop(optional, into)]
    href: MaybeProp<String>,
    /// Hints at the language of the content at the linked URL.
    #[prop(optional, into)]
    hreflang: MaybeProp<String>,
    /// Hint the MIME type of the linked data.
    #[prop(optional, into)]
    mimetype: MaybeProp<String>,
    /// Space separated list of URLs to ping with a `POST` request when the link is clicked.
    #[prop(optional, into)]
    ping: MaybeProp<String>,
    /// How much of the referrer header to send when following the link.
    #[prop(optional, into)]
    referrerpolicy: MaybeProp<String>,
    /// Relationship between the linked resource and the current document.
    #[prop(optional, into)]
    rel: MaybeProp<String>,
    /// Specify where to display the linked content
    #[prop(optional, into)]
    target: MaybeProp<String>,

    // ARIA ATTRIBUTES
    //
    #[prop(optional, into)] aria_current: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,

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

    let a_attrs = view! {
        <{..}
            download=move || download.get()
            href=move || href.get()
            hreflang=move || hreflang.get()
            type=move || mimetype.get()
            ping=move || ping.get()
            referrerpolicy=move || referrerpolicy.get()
            rel=move || rel.get()
            target=move || target.get()
        />
    };

    let in_sidebar_menu_button: bool = use_context::<SidebarMenuButtonContext>().is_some();
    let in_breadcrumb_link: bool = use_context::<BreadcrumbLinkContext>().is_some();

    view! {
        <A
            attr:aria_current=move || aria_current.get()
            attr:aria_label=move || aria_label.get()
            attr:class=move || {
                format!(
                    "{} {}",
                    match render_as.get().unwrap_or_default().as_str() {
                        "button" => {
                            format!(
                                "singlestage-button {} {} {}{}",
                                match variant.get().unwrap_or_default().as_str() {
                                    "secondary" => "singlestage-button-variant-secondary",
                                    "outline" => "singlestage-button-variant-outline",
                                    "ghost" => "singlestage-button-variant-ghost",
                                    "link" => "singlestage-button-variant-link",
                                    "destructive" => "singlestage-button-variant-destructive",
                                    _ => {
                                        if use_context::<InputGroupContext>().is_some()
                                            && variant.get().is_none()
                                        {
                                            "singlestage-button-variant-ghost"
                                        } else {
                                            "singlestage-button-variant-default"
                                        }
                                    }
                                },
                                match size.get().unwrap_or_default().as_str() {
                                    "xs" | "extra small" => "singlestage-button-size-xs",
                                    "sm" | "small" => "singlestage-button-size-sm",
                                    "lg" | "large" => "singlestage-button-size-lg",
                                    "icon" => "singlestage-button-size-icon",
                                    "xs-icon" | "icon-xs" | "icon extra small"
                                    | "extra small icon" => "singlestage-button-size-icon-xs",
                                    "sm-icon" | "icon-sm" | "icon small" | "small icon" => {
                                        "singlestage-button-size-icon-sm"
                                    }
                                    "lg-icon" | "icon-lg" | "icon large" | "large icon" => {
                                        "singlestage-button-size-icon-lg"
                                    }
                                    _ => "singlestage-button-size-default",
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
                                if in_sidebar_menu_button { " w-full" } else { "" },
                            )
                        }
                        "badge" => {
                            format!(
                                "singlestage-badge {}",
                                match variant.get().unwrap_or_default().as_str() {
                                    "secondary" => "singlestage-badge-variant-secondary",
                                    "destructive" => "singlestage-badge-variant-destructive",
                                    "outline" => "singlestage-badge-variant-outline",
                                    "ghost" => "singlestage-badge-variant-ghost",
                                    "link" => "singlestage-badge-variant-link",
                                    _ => "singlestage-badge-variant-default",
                                },
                            )
                        }
                        _ => {
                            if as_child.get().unwrap_or_default() {
                                "".to_string()
                            } else if in_sidebar_menu_button {
                                format!(
                                    "singlestage-sidebar-menu-button {} {}",
                                    match size.get().unwrap_or_default().as_str() {
                                        "sm" | "small" => "singlestage-sidebar-menu-button-size-sm",
                                        "lg" | "large" => "singlestage-sidebar-menu-button-size-lg",
                                        _ => "singlestage-sidebar-menu-button-size-default",
                                    },
                                    match variant.get().unwrap_or_default().as_str() {
                                        "outline" => {
                                            "singlestage-sidebar-menu-button-variant-outline"
                                        }
                                        _ => "singlestage-sidebar-menu-button-variant-default",
                                    },
                                )
                            } else if in_breadcrumb_link {
                                "singlestage-breadcrumb-link".to_string()
                            } else {
                                "singlestage-link".to_string()
                            }
                        }
                    },
                    class.get().unwrap_or_default(),
                )
            }
            href=move || href.get().unwrap_or_default()
            on:click=move |_| {
                if let Some(sheet) = use_context::<SheetContext>() && in_sidebar_menu_button {
                    sheet.open.set(false)
                }
            }

            {..global_attrs_1}
            {..global_attrs_2}
            {..a_attrs}
        >
            {children()}
        </A>
    }
}
