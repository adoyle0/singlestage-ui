use super::Theme;
use std::borrow::Cow;

#[allow(non_upper_case_globals)]
/// Uses tailwind yellow palette
pub const Yellow: Theme = Theme {
    common: Cow::Borrowed(
        r#"
  --color-yellow-50: oklch(98.7% 0.026 102.212);
  --color-yellow-100: oklch(97.3% 0.071 103.193);
  --color-yellow-200: oklch(94.5% 0.129 101.54);
  --color-yellow-300: oklch(90.5% 0.182 98.111);
  --color-yellow-400: oklch(85.2% 0.199 91.936);
  --color-yellow-500: oklch(79.5% 0.184 86.047);
  --color-yellow-600: oklch(68.1% 0.162 75.834);
  --color-yellow-700: oklch(55.4% 0.135 66.442);
  --color-yellow-800: oklch(47.6% 0.114 61.907);
  --color-yellow-900: oklch(42.1% 0.095 57.708);
  --color-yellow-950: oklch(28.6% 0.066 53.813);

  --chart-1: var(--color-yellow-300);
  --chart-2: var(--color-yellow-500);
  --chart-3: var(--color-yellow-600);
  --chart-4: var(--color-yellow-700);
  --chart-5: var(--color-yellow-800);
"#,
    ),
    dark: Cow::Borrowed(
        r#"
    --primary: var(--color-yellow-500);
    --primary-foreground: var(--color-yellow-900);
    --ring: var(--color-yellow-900);
    --sidebar-primary: var(--color-yellow-500);
    --sidebar-primary-foreground: var(--color-yellow-50);
    --sidebar-ring: var(--color-yellow-900);
"#,
    ),
    light: Cow::Borrowed(
        r#"
  --primary: var(--color-yellow-400);
  --primary-foreground: var(--color-yellow-900);
  --ring: var(--color-yellow-400);
  --sidebar-primary: var(--color-yellow-600);
  --sidebar-primary-foreground: var(--color-yellow-50);
  --sidebar-ring: var(--color-yellow-400);
"#,
    ),
};
