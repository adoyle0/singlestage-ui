#[allow(non_snake_case)]
pub mod Theme;
mod mode;

use crate::CSS;
pub use mode::*;

use leptos::prelude::*;
use leptos_meta::Style;

// TODO: CSS should be in an actual css file and run through tailwind somehow. This works for now

#[derive(Clone)]
pub struct ThemeProviderContext {
    /// Set the theme's light/dark mode behavior. Defaults to `Mode::Auto`.
    pub mode: RwSignal<Mode>,
    /// The get/set/update the current theme in use.
    pub theme: RwSignal<Theme::Theme>,
}

#[component]
/// Provides theme support to children. Note: Setting `mode` and `theme` here are only used for
/// initial values. Updates should be done via `ThemeProviderContext`.
pub fn ThemeProvider(
    children: Children,
    /// Set the initial light/dark mode behavior. Defaults to `auto`/`Mode::Auto`.
    ///
    /// Accepted values: `auto` | `dark` | `light` or a `Mode`
    #[prop(optional, into)]
    mode: String,
    /// Set the initial Theme.
    #[prop(optional, into)]
    theme: MaybeProp<Theme::Theme>,
) -> impl IntoView {
    let mode = RwSignal::<Mode>::new(mode.into());
    let theme = RwSignal::new(theme.get_untracked().unwrap_or(Theme::Default));

    let context = ThemeProviderContext { theme, mode };
    provide_context(context);

    // TODO: Don't have css here
    let dark_common = r#"
    --scrollbar-thumb: rgba(255, 255, 255, 0.3);
    color-scheme: dark;
  
    --surface: oklch(0.2 0 0);
    --code-highlight: oklch(0.27 0 0);
    --code-number: oklch(0.72 0 0);
    --selection: oklch(0.922 0 0);
    --selection-foreground: oklch(0.205 0 0);
  
    --background: oklch(0.145 0 0);
    --foreground: oklch(0.985 0 0);
    --card: oklch(0.205 0 0);
    --card-foreground: oklch(0.985 0 0);
    --popover: oklch(0.269 0 0);
    --popover-foreground: oklch(0.985 0 0);
    --secondary: oklch(0.269 0 0);
    --secondary-foreground: oklch(0.985 0 0);
    --muted: oklch(0.269 0 0);
    --muted-foreground: oklch(0.708 0 0);
    --accent: oklch(0.371 0 0);
    --accent-foreground: oklch(0.985 0 0);
    --destructive: oklch(0.704 0.191 22.216);
    --border: oklch(1 0 0 / 10%);
    --input: oklch(1 0 0 / 15%);
    --sidebar: oklch(0.205 0 0);
    --sidebar-foreground: oklch(0.985 0 0);
    --sidebar-accent: oklch(0.269 0 0);
    --sidebar-accent-foreground: oklch(0.985 0 0);
    --sidebar-border: oklch(1 0 0 / 10%);
"#;

    // TODO: Don't have css here
    // TODO: This code is dead and here for future reference
    let _dark_overrides_tw = r#"
@layer components {
  .singlestage-btn-primary {
    .singlestage-input {
      @apply border-primary-foreground/30!;
    }
  }

  .singlestage-textarea {
    @apply aria-invalid:ring-destructive/40
    bg-input/30;
  }

  select.singlestage-select {
    @apply aria-invalid:ring-destructive/40
    bg-input/30
    hover:bg-input/50;
  }

  .singlestage-input[type="radio"] {
    @apply aria-invalid:ring-destructive/40
    bg-input/30;
  }

  .singlestage-input[type="color"],
  .singlestage-input[type="date"],
  .singlestage-input[type="datetime-local"],
  .singlestage-input[type="email"],
  .singlestage-input[type="file"],
  .singlestage-input[type="month"],
  .singlestage-input[type="number"],
  .singlestage-input[type="password"],
  .singlestage-input[type="search"],
  .singlestage-input[type="tel"],
  .singlestage-input[type="text"],
  .singlestage-input[type="time"],
  .singlestage-input[type="url"],
  .singlestage-input[type="week"] {
    @apply bg-input/30
    aria-invalid:ring-destructive/40;
  }

  .singlestage-input[type="checkbox"]:not([role="switch"]) {
    @apply aria-invalid:ring-destructive/40
    bg-input/30
    checked:bg-primary;
  }

  .singlestage-tabs {
    [role="tablist"] {
      [role="tab"] {
        @apply text-muted-foreground;
        &[aria-selected="true"] {
          @apply bg-input/30
          border-input
          text-foreground;
        }
      }
    }
  }

  .singlestage-input[type="checkbox"][role="switch"] {
    @apply before:bg-foreground
    bg-input/80
    checked:before:bg-primary-foreground
    checked:bg-primary;
  }

  .singlestage-badge-primary,
  .singlestage-badge-secondary,
  .singlestage-badge-destructive,
  .singlestage-badge-outline {
    @apply aria-invalid:ring-destructive/40;
  }

  .singlestage-badge-destructive {
    @apply bg-destructive/60
    focus-visible:ring-destructive/40;
  }

  .singlestage-btn-primary,
  .singlestage-btn-secondary,
  .singlestage-btn-outline,
  .singlestage-btn-ghost,
  .singlestage-btn-link,
  .singlestage-btn-destructive,
  .singlestage-btn-icon {
    @apply aria-invalid:ring-destructive/40;
  }

  .singlestage-btn-outline {
    @apply bg-input/30
    border-input;

    &:hover,
    &[aria-pressed="true"] {
      @apply bg-accent/50;
    }
  }

  .singlestage-btn-ghost {
    &:hover,
    &[aria-pressed="true"] {
      @apply bg-accent/50;
    }
    .singlestage-btn-destructive {
      @apply focus-visible:ring-destructive/40
      bg-destructive/60;

      &:hover,
      &[aria-pressed="true"] {
        @apply bg-destructive/50;
      }
    }
  }
}"#;

    let dark_overrides = r#"@layer components {
  .singlestage-btn-primary {
    .singlestage-input[type="checkbox"],
    .singlestage-input[type="radio"] {
      @supports (color: color-mix(in lab, red, red)) {
        border-color: color-mix(in oklab, var(--primary-foreground) 30%, transparent) !important;
      }
    }
  }

  .singlestage-textarea {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  select.singlestage-select {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &:hover {
      @media (hover: hover) {
        background-color: var(--input);
        @supports (color: color-mix(in lab, red, red)) {
          background-color: color-mix(in oklab, var(--input) 50%, transparent);
        }
      }
    }
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-input[type="radio"] {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-input[type="color"],
  .singlestage-input[type="date"],
  .singlestage-input[type="datetime-local"],
  .singlestage-input[type="email"],
  .singlestage-input[type="file"],
  .singlestage-input[type="month"],
  .singlestage-input[type="number"],
  .singlestage-input[type="password"],
  .singlestage-input[type="search"],
  .singlestage-input[type="tel"],
  .singlestage-input[type="time"],
  .singlestage-input[type="url"],
  .singlestage-input[type="week"],
  .singlestage-input[type="text"] {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-input[type="checkbox"]:not([role="switch"]) {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &:checked {
      background-color: var(--primary);
    }
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-tabs {
    [role="tablist"] {
      [role="tab"] {
        color: var(--muted-foreground);
        &[aria-selected="true"] {
          border-color: var(--input);
          background-color: var(--input);
          @supports (color: color-mix(in lab, red, red)) {
            background-color: color-mix(
              in oklab,
              var(--input) 30%,
              transparent
            );
          }
          color: var(--foreground);
        }
      }
    }
  }
  .singlestage-input[type="checkbox"][role="switch"] {
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 80%, transparent);
    }
    &::before {
      content: var(--tw-content);
      background-color: var(--foreground);
    }
    &:checked {
      background-color: var(--primary);
    }
    &:checked {
      &::before {
        content: var(--tw-content);
        background-color: var(--primary-foreground);
      }
    }
  }
  .singlestage-badge-primary,
  .singlestage-badge-secondary,
  .singlestage-badge-destructive,
  .singlestage-badge-outline {
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-badge-destructive {
    background-color: var(--destructive);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(
        in oklab,
        var(--destructive) 60%,
        transparent
      );
    }
    &:focus-visible {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-btn-primary,
  .singlestage-btn-secondary,
  .singlestage-btn-outline,
  .singlestage-btn-ghost,
  .singlestage-btn-link,
  .singlestage-btn-destructive,
  .singlestage-btn-icon {
    &[aria-invalid="true"] {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
  }
  .singlestage-btn-outline {
    border-color: var(--input);
    background-color: var(--input);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(in oklab, var(--input) 30%, transparent);
    }
    &:hover,
    &[aria-pressed="true"] {
      background-color: var(--accent);
      @supports (color: color-mix(in lab, red, red)) {
        background-color: color-mix(in oklab, var(--accent) 50%, transparent);
      }
    }
  }
  .singlestage-btn-ghost {
    &:hover,
    &[aria-pressed="true"] {
      background-color: var(--accent);
      @supports (color: color-mix(in lab, red, red)) {
        background-color: color-mix(in oklab, var(--accent) 50%, transparent);
      }
    }
  }
  .singlestage-btn-destructive {
    background-color: var(--destructive);
    @supports (color: color-mix(in lab, red, red)) {
      background-color: color-mix(
        in oklab,
        var(--destructive) 60%,
        transparent
      );
    }
    &:focus-visible {
      --tw-ring-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        --tw-ring-color: color-mix(
          in oklab,
          var(--destructive) 40%,
          transparent
        );
      }
    }
    &:hover,
    &[aria-pressed="true"] {
      background-color: var(--destructive);
      @supports (color: color-mix(in lab, red, red)) {
        background-color: color-mix(
          in oklab,
          var(--destructive) 50%,
          transparent
        );
      }
    }
  }
}"#;

    view! {
        <Style id="singlestage">{CSS}</Style>
        <style
            id="theme"
            inner_html=move || {
                let theme = theme.get();
                match mode.get() {
                    Mode::Dark => {
                        format!(
                            ":root{{ {} {} {}}}\n{}\n",
                            dark_common,
                            theme.common,
                            theme.dark,
                            dark_overrides,
                        )
                    }
                    Mode::Light => format!(":root{{ {} {}}}\n", theme.common, theme.light),
                    Mode::Auto => {
                        format!(
                            ":root{{ {} {}}}\n\n@media (prefers-color-scheme: dark) {{\n  :root {{ {} {}  }}\n{}}}",
                            theme.common,
                            theme.light,
                            dark_common,
                            theme.dark,
                            dark_overrides,
                        )
                    }
                }
            }
        ></style>
        {children()}
    }
}
