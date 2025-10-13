# Singlestage Manifesto

If you're reading this then there's a good chance you're interested in contributing, thank you! Let's go
over a few things:

## Goals

Singlestage aims to be an easy to use, one-size-fits-all component library that generally
follows the styling and features of [Basecoat](https://basecoatui.com), [shadcn/ui](https://ui.shadcn.com), and [Radix](https://radix-ui.com).

### Singlestage is not a port

While Singlestage components mimic the look and feel of components from other libraries,
similarities in their implementation are purely coincidental. If you're building a new component
for Singlestage based on an existing component from another library, matching the API/feature
structure and styling are great places to start, but think about whether its implementation makes
sense for Rust, Wasm, and Leptos, where there are different strengths and weaknesses. This is not
to say that their choices are wrong. It may simply be the case that new standards exist now that
were not around when these other libraries were written. Binary size is another variable that has
more impact in the Rust/Wasm world than Javascript.

### Lean heavily on native browser/CSS features for performance and accessibility

This one may not be immediately obvious, but opting for native HTML/CSS solutions rather than
bespoke Rust code can have considerable advantages when it comes to Wasm binary size where we're
already at a disadvantage. Together with semantic HTML elements, these new features provide great
accessibility for screen readers, keyboard navigation, and the like. Some examples of this already
used are:

- **Accordion** - The `Accordion` component uses `details` and `summary` elements to create the
  collapsible effect. Setting the `name` attribute for each `details` to the same value makes the
  browser allow only one `summary` to be expanded at a time, for free.

- **Sidebar** - The `Sidebar` makes use of `header`, `footer`, `nav`, and `section`, semantic
  elements to assist screen readers in understanding the layout of sidebar content.

### Add additional accessibility where applicable

Many elements in HTML now have ARIA roles, for example an `<input type="radio" />` is
automatically treated as `<input type="radio" role="radio" />`. This is not always the case
however, the `Tabs` component for example is made up of elements that combine `role="tab"`,
`role="tabpanel"`, and `role="tablist"` to provide context about the component to both the browser
and screen readers. This is also another example of getting things from the browser for free and
saving on Rust code/binary size. [More Info](https://developer.mozilla.org/en-US/docs/Web/Accessibility).

### Components should "just work" out of the box with sane defaults

When building a component you should tailor the defaults to the most common use case. For
example, a `Button` with no `variant` attribute set displays with the equivalent of
`variant="primary"`.

### Implementation changes are cheap, API changes are expensive

If you're building a new component for Singlestage based on an existing component from
another library, particularly if you're only partially implementing the component for whatever
reason, carry over the full API even if it feels unnecessary. If building a new component from
scratch, try to wrap all the elements that make each layer of your larger component with their own
sub-component. There are some components that may feel like useless wrappers, maybe they are,
do you really need all of these elements? Sometimes you may feel like you can get away with
eliminating a sub-component by combining it with what would be its parent component. There are a
few problems with doing this:

1. **Which element do attributes act on?** - We can't predict how the user is going to use
   this component. One component may behave as expected when setting `class="some-class"` but
   another component doesn't work unless the user writes `attr:class="some-class"`. This is
   ugly and unintuitive, and brings inconsistency to the library as a whole.

1. **It limits flexibility for expansion** - For components to communicate with one another
   via a context, the context needs to be provided by a common parent. An example of this is
   the `Sidebar` component. The `SidebarTrigger` lives outside of the `Sidebar`, but needs
   `SidebarProvider` to contain both to provide `SidebarContext`. If `SidebarTrigger` isn't part of
   version 1 of `Sidebar` then adding it would be a breaking change that requires all users to
   refactor their code.

### Prefer `Strings` over `Enums` for attributes

Strings look nicer in a view macro than enums, also typically you're going to pass that string to
an underlying HTML element anyway, which takes a string, so it might as well just be a string.
Leptos also uses strings for its API, so it makes converting a Leptos `<button>` to a Singlestage
`<Button>` easier. Enums also add to the wasm binary size. Also consider that if you have something
like a `<Button>` that has n variants, and in the future you add one variant, anyone who for some
reason may be matching the `ButtonVariant` enum is gonna have their code break just because we
added a feature that they may not even care about. Enums are great and have their place, this isn't
the place, match the string.

### Use a full HTML-like API

Singlestage components should all implement the entire API of the HTML element they wrap.
This has many benefits:

- **API Consistency** - No `class=` vs `attr:class=` across different components.

- **API Stability** - The component that used to use `attr:class=`, but has been updated to
  use `class=` is not suddenly broken after the update. The behavior of this component may suddenly
  change with no clear reason why, and possibly without any error message.

- **Intuitive API** - Users may be used to all elements having global attributes, this way
  everything just works. They can "guess" the API without having to look anything up.

- **Full Reactivity** - Again, we don't know how a user may use each component. Any attributes can
  be updated reactively. This also goes back to consistency.

### Components are not opinionated

If a user wants a different icon in a place where a default is provided, it should be easy to
change. An example of this being used is in the `AccordionTrigger` component. A chevron icon is
provided by default, but it can be overridden by adding an `AccordionTriggerIcon` child. Other
components display their defaults when they have no children, but override those defaults with
arbitrary content provided as children.

### Avoid using other Singlestage components internally

An example of this is the `DropdownMenuTrigger`. Instead of using a `Button`, it re-implements the
`Button` itself. This avoids a layer of complexity and prevents updates to `Button` breaking
`DropdownMenuTrigger`. Remember to add feature dependencies to `Cargo.toml` when using CSS classes
from other components.

### Prefix CSS classes with `singlestage-` to avoid name collisions

Instead of `button` call it `singlestage-button`, users may want to define their own `button`
class.

## Architecture

Singlestage is pretty simple, the entire library really can be understood just by looking at
[build.rs](https://github.com/adoyle0/singlestage-ui/blob/main/singlestage/build.rs).

Basically:

1. Every component is just a Leptos component or group of Leptos components, usually with an
   associated CSS file, each behind its own feature flag.
1. The build step concatenates all CSS files for enabled features into one big CSS file
   starting with `ThemeProvider`.
1. The large file then goes through Tailwind to reduce class duplication and is minified.
1. The resulting CSS is injected into the `head` via `ThemeProvider`.

## Docs

Please update the docs to reflect any changes or new features that you may add.

### Adding a new component page

The docs/demo website is mostly auto-generated via macros from a few source files.
[Click here for some examples](https://github.com/adoyle0/singlestage-ui/tree/main/docs/src/routes/components).
All you have to do is create a dir here with the same files/structure as the others and the
rest is automated.

## Code style

This isn't a big deal, just try to blend in with existing code. Personally I use `rustfmt` and
`leptosfmt`, I try to keep lines under 100char unless it's a svg or something, I like my lists
vertical (if it doesn't piss off rustfmt), and typically I sort things alphabetically.

## Conclusion

By now you should have a pretty clear understanding of the goals and direction of Singlestage.
Think of this document less like a rubric that all contributions are scored against that's going to
bounce all your PRs, and more just a general guideline to promote teamwork. It makes everyone's
life easier if we're all working toward the same clearly defined goal.

If for some reason you won't contribute code, consider opening an issue with a new idea, or
providing any other feedback. Contributions don't all have to be code. If you think you have a good
idea but are unsure how to implement it, it's probably still a good idea. Make it known so someone
else can implement it!
