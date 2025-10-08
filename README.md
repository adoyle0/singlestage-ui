# Singlestage UI

A familiar UI library for [Leptos](https://leptos.dev) made with [Tailwind
CSS](https://tailwindcss.com), Based on [Basecoat](https://basecoatui.com),
[shadcn/ui](https://ui.shadcn.com), and [Radix](https://radix-ui.com).

[https://singlestage.doordesk.net](https://singlestage.doordesk.net)

## Introduction

Singlestage takes the familiar styling of [Radix](https://radix-ui.com/) and
[shadcn/ui](https://ui.shadcn.com/) and brings it to [Leptos](https://leptos.dev). It takes a
modular approach, meaning every component is feature flagged so you can take as much or as little
as you need.

Each component tries to stay lean, using semantic HTML elements, modern APIs, and CSS instead of
Rust code wherever possible. This keeps WASM binaries small while staying current with emerging
accessibility features.

## Features

- Conforms to modern web standards
- WAI-ARIA compliant
- Focused on minimalism, modularity, and performance
- Feature flagged components
- Modular theme system
- Dark mode that just works
- Arbitrary theme support
- HTML-like API

## Install

Add singlestage with cargo:
`cargo add singlestage`

Or add singlestage to your Cargo.toml:

```toml
# Cargo.toml

[dependencies]
leptos = "0.8"
...
singlestage = "0.2"
```

### Nightly

```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.8", features = ["nightly"] }
...
singlestage = {version = "0.2", features = ["nightly"] }
```

Add the nightly feature to singlestage just as you do with Leptos, if using nightly.

### SSR

Nothing special needs to happen whether you you use SSR or not.

## Contributing

Contributions are welcome.
