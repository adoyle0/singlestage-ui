use super::*;
use leptos::prelude::*;

/// Sidebar component
#[component]
pub fn Sidebar(
    children: Children,

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

    // Initialize on component load
    let sidebar_ref = NodeRef::<leptos::html::Aside>::new();

    // Close sidebar on screen resize if the screen becomes small.
    // Like rotating a phone from landscape to portrait
    let sidebar_close = sidebar.close_if_small_screen.clone();
    window_event_listener(leptos::ev::resize, move |_| {
        sidebar_close();
    });

    let sidebar_close = sidebar.close_if_small_screen.clone();
    Effect::new(move || {
        sidebar_close();

        // CSS hides the sidebar until we tell it otherwise
        // to avoid it appearing and then disappearing in a server render
        // since the server can't predict the client's screen size.
        // Now we tell it otherwise.
        if let Some(writer) = sidebar_ref.write().as_ref() {
            let _ = writer.set_attribute("data-sidebar-initialized", "");
        }
    });

    // Ideally we'd have an ev.stop_propagation() on the nav element
    // but that breaks client-side navigation.
    //
    // So we'll do a bit of math to calculate if the click was over the nav area.
    // If the screen is small and the click was off the bar, it closes.
    // SidebarGroupContent and SidebarMenuContent fire their own close events.
    //
    // It's written out again instead of using the helpers to save a few calls
    // and allow returning early cause I'm into that type of stuff.
    let on_click = move |ev: MouseEvent| {
        let screen_width = get_screen_width().unwrap_or(0.) as usize;
        if screen_width == 0 {
            return;
        }

        let font_size;
        if let Some(size) = get_font_size() {
            font_size = size;
        } else {
            font_size = DEFAULT_FONT_PX;
        }

        // Check if screen is small
        let breakpoint = font_size * BREAKPOINT_REM;
        if screen_width > breakpoint {
            // Screen is large, bail
            return;
        }

        // Check if click was on the bar
        let click_x = ev.client_x() as usize;
        let bar_width = SIDEBAR_MOBILE_WIDTH_REM * font_size;
        match sidebar.side.get_untracked().as_str() {
            "right" => {
                if click_x < (screen_width - bar_width) {
                    sidebar.hidden.set(true);
                }
            }
            _ => {
                if click_x > bar_width {
                    sidebar.hidden.set(true);
                }
            }
        }
    };

    let global_attrs_1 = view! {
        <{..}
            accesskey=move || accesskey.get()
            autocapitalize=move || autocapitalize.get()
            autofocus=move || autofocus.get()
            class=move || class.get()
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
        <aside
            node_ref=sidebar_ref
            class="singlestage-sidebar"
            on:click=on_click
            data-side=move || sidebar.side.get()
            aria-hidden=move || sidebar.hidden.get().to_string()
        >
            <nav
                aria-label="Sidebar navigation"

                {..global_attrs_1}
                {..global_attrs_2}
            >
                {children()}
            </nav>
        </aside>
    }
}
