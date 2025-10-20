use super::Theme;
use std::borrow::Cow;

#[allow(non_upper_case_globals)]
/// Monospace fonts and a blocky appearance
pub const Mono: Theme = Theme {
    common: Cow::Borrowed(
        r#"
  --color-stone-50: oklch(98.5% 0.001 106.423);
  --color-stone-100: oklch(97% 0.001 106.424);
  --color-stone-200: oklch(92.3% 0.003 48.717);
  --color-stone-300: oklch(86.9% 0.005 56.366);
  --color-stone-400: oklch(70.9% 0.01 56.259);
  --color-stone-500: oklch(55.3% 0.013 58.071);
  --color-stone-600: oklch(44.4% 0.011 73.639);
  --color-stone-700: oklch(37.4% 0.01 67.558);
  --color-stone-800: oklch(26.8% 0.007 34.298);
  --color-stone-900: oklch(21.6% 0.006 56.043);

  --chart-1: var(--color-stone-300);
  --chart-2: var(--color-stone-500);
  --chart-3: var(--color-stone-600);
  --chart-4: var(--color-stone-700);
  --chart-5: var(--color-stone-800);

  --font-sans: var(--font-mono);

  @media (min-width: 1024px) {
    --font-sans: var(--font-mono);
    --radius: 0.45em;
    --text-lg: 1rem;
    --text-xl: 1.1rem;
    --text-2xl: 1.2rem;
    --text-3xl: 1.3rem;
    --text-4xl: 1.4rem;
    --text-5xl: 1.5rem;
    --text-6xl: 1.6rem;
    --text-7xl: 1.7rem;
    --text-8xl: 1.8rem;
    --text-base: 0.85rem;
    --text-sm: 0.8rem;
    --spacing: 0.222222rem;
  }

  .rounded-xs,
  .rounded-sm,
  .rounded-md,
  .rounded-lg,
  .rounded-xl {
    border-radius: 0;
  }

  .shadow-xs,
  .shadow-sm,
  .shadow-md,
  .shadow-lg,
  .shadow-xl {
    box-shadow: none;
  }

  [data-slot="toggle-group"],
  [data-slot="toggle-group-item"] {
    border-radius: 0 !important;
    box-shadow: 0 0 #0000 !important;
  }
"#,
    ),
    dark: Cow::Borrowed(
        r#"
    --primary: var(--color-stone-500);
    --primary-foreground: var(--color-stone-50);
    --ring: var(--color-stone-900);
    --sidebar-primary: var(--color-stone-500);
    --sidebar-primary-foreground: var(--color-stone-50);
    --sidebar-ring: var(--color-stone-900);
"#,
    ),
    light: Cow::Borrowed(
        r#"
  --primary: var(--color-stone-600);
  --primary-foreground: var(--color-stone-50);
  --ring: var(--color-stone-400);
  --sidebar-primary: var(--color-stone-600);
  --sidebar-primary-foreground: var(--color-stone-50);
  --sidebar-ring: var(--color-stone-400);
"#,
    ),
};
