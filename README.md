# Singlestage UI

A familiar UI library for [Leptos](https://leptos.dev) made with [Tailwind
CSS](https://tailwindcss.com), and based on [Basecoat](https://basecoatui.com),
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
singlestage = "0.4"
```

### Nightly

```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.8", features = ["nightly"] }
...
singlestage = { version = "0.4", features = ["nightly"] }
```

Add the nightly feature to singlestage just as you do with Leptos, if using nightly.

### Islands

Add the islands feature to singlestage just as you do with Leptos, if using islands.

```toml
# Cargo.toml

[dependencies]
leptos = { version = "0.8", features = ["islands"] }
...
singlestage = { version = "0.4", features = ["islands"] }
```

### SSR

Nothing special needs to happen whether you you use SSR or not.

### Reactive Stores

Add the stores feature to singlestage if you're using reactive stores. This will enable using store
`Field`s as `Reactive` values.

```toml
# Cargo.toml

[dependencies]
leptos = "0.8"
...
reactive_stores = "0.3"
singlestage = { version = "0.4", features = ["stores"] }
```

### Tailwind BYOB

Singlestage uses tailwind during the build process. It first checks for `tailwindcss` in
`PATH` (system install, typical for Linux). If that doesn't work then it tries to download the
latest tailwind binary from their github release page. If for some reason this fails, or for
whatever reason you want to use your own tailwind executable, you can bring your own binary by
setting the `SINGLESTAGE_TAILWIND_PATH` environment variable to the full path (from root) to your
tailwind binary. Note that if you download the binary from github on Linux or MacOS then you'll
probably have to make it executable (`chmod +x`).

```bash
SINGLESTAGE_TAILWIND_PATH=/path/to/tailwindcss cargo leptos watch
```

## Contributing

Contributions are welcome.
