use super::*;
use crate::{Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle};
use leptos::prelude::*;

// Large/small screen breakpoint magic numbers for show/hide
const BREAKPOINT_REM: usize = 48;
const BREAKPOINT_PX: usize = 768;

/// Get current viewport width
fn get_screen_width() -> Option<f64> {
    if let Ok(js_width) = window().inner_width()
        && let Some(f64_width) = js_width.as_f64()
    {
        return Some(f64_width);
    }

    None
}

/// Get the font size setting of the browser viewing the page
fn get_font_size() -> Option<usize> {
    if let Some(dom) = document().document_element()
        && let Ok(Some(css)) = window().get_computed_style(&dom)
        && let Ok(mut font_size) = css.get_property_value("font-size")
    {
        // Cut off "px"
        let _ = font_size.split_off(font_size.len() - 2);

        if let Ok(parsed_font_size) = font_size.parse::<usize>() {
            return Some(parsed_font_size);
        }
    }

    None
}

/// Determine if the viewport is smaller than 48rem wide
fn screen_is_small() -> bool {
    let breakpoint;
    if let Some(font_size) = get_font_size() {
        breakpoint = font_size * BREAKPOINT_REM;
    } else {
        breakpoint = BREAKPOINT_PX;
    }

    if let Some(screen_width) = get_screen_width()
        && screen_width < breakpoint as f64
    {
        return true;
    }

    false
}

/// The sidebar container.
#[component]
pub fn Sidebar(
    children: ChildrenFn,

    #[prop(optional, into)] collapsible: MaybeProp<String>,
    #[prop(optional, into)] default_open: MaybeProp<bool>,
    #[prop(optional, into)] side: MaybeProp<String>,
    #[prop(optional, into)] variant: MaybeProp<String>,

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
    let sidebar = expect_context::<SidebarContext>();
    sidebar.open.set(default_open.get().unwrap_or(true));

    if let Some(side) = side.get() {
        sidebar.side.set(side);
    };
    let side: Reactive<String> = sidebar.side;

    // client init
    Effect::new(move || {
        if default_open.get_untracked().is_some() {
            return;
        }

        sidebar.is_mobile.set(screen_is_small());
        sidebar.open.set(!sidebar.is_mobile.get_untracked());
    });

    window_event_listener(leptos::ev::resize, move |_| {
        // possibly useless optimization
        let screen_is_small = screen_is_small();

        // this should only run when breakpoint is hit
        if sidebar.is_mobile.get_untracked() != screen_is_small {
            if screen_is_small {
                sidebar.open.set(false);
            }

            sidebar.is_mobile.set(screen_is_small);
        }
    });

    let children = StoredValue::new(children);

    view! {
        <Show
            when=move || collapsible.get().unwrap_or_default().as_str() != "none"
            fallback=move || {
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

                view! {
                    <div
                        class=move || {
                            format!(
                                "singlestage-sidebar-collapsible-none {}",
                                class.get().unwrap_or_default(),
                            )
                        }

                        {..global_attrs_1}
                        {..global_attrs_2}
                    >
                        {children.read_value()()}
                    </div>
                }
            }
        >
            <Show
                when=move || !sidebar.is_mobile.get()
                fallback=move || {
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

                    view! {
                        <Sheet open=sidebar.open>
                            <SheetContent class="singlestage-sidebar-mobile" side>
                                <SheetHeader class="sr-only">
                                    <SheetTitle>"Sidebar"</SheetTitle>
                                    <SheetDescription>
                                        "Displays the mobile sidebar."
                                    </SheetDescription>
                                </SheetHeader>
                                <div
                                    class=move || {
                                        format!(
                                            "singlestage-sidebar-inner {}",
                                            class.get().unwrap_or_default(),
                                        )
                                    }

                                    {..global_attrs_1}
                                    {..global_attrs_2}
                                >
                                    {children.read_value()()}
                                </div>
                            </SheetContent>
                        </Sheet>
                    }
                }
            >
                {
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

                    view! {
                        <div
                            aria_expanded=move || {
                                match sidebar.open.get() {
                                    true => Some("true"),
                                    false => None,
                                }
                            }
                            class=move || {
                                format!(
                                    "singlestage-sidebar{} {} {}",
                                    match sidebar.open.get() {
                                        true => "",
                                        false => {
                                            match collapsible.get().unwrap_or_default().as_str() {
                                                "icon" => " singlestage-sidebar-collapsible-icon",
                                                _ => " singlestage-sidebar-collapsible-offcanvas",
                                            }
                                        }
                                    },
                                    match side.get().as_str() {
                                        "right" => "singlestage-sidebar-side-right",
                                        _ => "singlestage-sidebar-side-left",
                                    },
                                    match variant.get().unwrap_or_default().as_str() {
                                        "floating" => "singlestage-sidebar-variant-floating",
                                        "inset" => "singlestage-sidebar-variant-inset",
                                        _ => "singlestage-sidebar-variant-sidebar",
                                    },
                                )
                            }
                        >
                            <div class="singlestage-sidebar-gap" />
                            <div class="singlestage-sidebar-container">
                                <div
                                    class=move || {
                                        format!(
                                            "singlestage-sidebar-inner {}",
                                            class.get().unwrap_or_default(),
                                        )
                                    }

                                    {..global_attrs_1}
                                    {..global_attrs_2}
                                >
                                    {children.read_value()()}
                                </div>
                            </div>
                        </div>
                    }
                }
            </Show>
        </Show>
    }
}
