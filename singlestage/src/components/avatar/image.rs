use leptos::prelude::*;

/// Renders an image inside the avatar.
#[component]
pub fn AvatarImage(
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
    // // TODO: Waiting on https://github.com/leptos-rs/leptos/pull/4308 and next release (>0.8.9)
    // /// Expose elements in the shadow DOM to be manipulated by the DOM.
    // #[prop(optional, into)]
    // exportparts: MaybeProp<String>,
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

    // IMG ATTRIBUTES
    //
    /// The text that shows when an image is not loaded.
    #[prop(optional, into)]
    alt: MaybeProp<String>,
    /// Indicates if the fetching of the image must be done using a `CORS` request.
    #[prop(optional, into)]
    crossorigin: MaybeProp<String>,
    /// Specifies how the browser should handle displaying the image or lack of image during
    /// rendering.
    ///
    /// Accepted values: "sync" | "async" | "auto"
    #[prop(optional, into)]
    decoding: MaybeProp<String>,
    /// Marks the image for observation by the `PerformanceElementTiming` API.
    #[prop(optional, into)]
    elementtiming: MaybeProp<String>,
    /// Set the priority of this image compared to other images on the same page.
    ///
    /// Accepted values: "high" | "low" | "auto"
    #[prop(optional, into)]
    fetchpriority: MaybeProp<String>,
    /// The height to render the image.
    #[prop(optional, into)]
    height: MaybeProp<usize>,
    /// Specify whether the image is part of a map and the coordinates should be sent back to the
    /// server..
    #[prop(optional, into)]
    ismap: MaybeProp<bool>,
    /// How the browser should load the image.
    ///
    /// Accepted values: "eager" | "lazy"
    #[prop(optional, into)]
    loading: MaybeProp<String>,
    /// Specify which referrer to use when fetching the image.
    #[prop(optional, into)]
    referrerpolicy: MaybeProp<String>,
    /// A comma separated list of sizes that the image can be displayed, or "auto".
    #[prop(optional, into)]
    sizes: MaybeProp<String>,
    /// The source of the image file.
    #[prop(optional, into)]
    src: MaybeProp<String>,
    /// A comma separated list of possible sources for the image.
    #[prop(optional, into)]
    srcset: MaybeProp<String>,
    /// A partial URL of an associated image map.
    #[prop(optional, into)]
    usemap: MaybeProp<String>,
    /// The width to render the image.
    #[prop(optional, into)]
    width: MaybeProp<String>,
) -> impl IntoView {
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
            // TODO: Waiting on https://github.com/leptos-rs/leptos/pull/4308 and next release (>0.8.9)
            // exportparts=move || exportparts.get()
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
        <img
            alt=move || alt.get()
            class=move || format!("singlestage-avatar-image {}", class.get().unwrap_or_default())
            crossorigin=move || crossorigin.get()
            decoding=move || decoding.get()
            elementtiming=move || elementtiming.get()
            fetchpriority=move || fetchpriority.get()
            height=move || height.get()
            ismap=move || ismap.get()
            loading=move || loading.get()
            referrerpolicy=move || referrerpolicy.get()
            sizes=move || sizes.get()
            src=move || src.get()
            srcset=move || srcset.get()
            usemap=move || usemap.get()
            width=move || width.get()

            {..global_attrs_1}
            {..global_attrs_2}
        />
    }
}
