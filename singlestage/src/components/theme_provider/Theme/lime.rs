use super::Theme;
use std::borrow::Cow;

#[allow(non_upper_case_globals)]
/// Uses tailwind lime palette
pub const Lime: Theme = Theme {
    common: Cow::Borrowed(
        r#"
  --color-lime-50: oklch(98.6% 0.031 120.757);
  --color-lime-100: oklch(96.7% 0.067 122.328);
  --color-lime-200: oklch(93.8% 0.127 124.321);
  --color-lime-300: oklch(89.7% 0.196 126.665);
  --color-lime-400: oklch(84.1% 0.238 128.85);
  --color-lime-500: oklch(76.8% 0.233 130.85);
  --color-lime-600: oklch(64.8% 0.2 131.684);
  --color-lime-700: oklch(53.2% 0.157 131.589);
  --color-lime-800: oklch(45.3% 0.124 130.933);
  --color-lime-900: oklch(40.5% 0.101 131.063);
  --color-lime-950: oklch(27.4% 0.072 132.109);

  --chart-1: var(--color-lime-300);
  --chart-2: var(--color-lime-500);
  --chart-3: var(--color-lime-600);
  --chart-4: var(--color-lime-700);
  --chart-5: var(--color-lime-800);
"#,
    ),
    dark: Cow::Borrowed(
        r#"
    --primary: var(--color-lime-600);
    --primary-foreground: var(--color-lime-50);
    --ring: var(--color-lime-900);
    --sidebar-primary: var(--color-lime-600);
    --sidebar-primary-foreground: var(--color-lime-50);
    --sidebar-ring: var(--color-lime-900);
"#,
    ),
    light: Cow::Borrowed(
        r#"
  --primary: var(--color-lime-600);
  --primary-foreground: var(--color-lime-50);
  --ring: var(--color-lime-400);
  --sidebar-primary: var(--color-lime-600);
  --sidebar-primary-foreground: var(--color-lime-50);
  --sidebar-ring: var(--color-lime-400);
"#,
    ),
};
