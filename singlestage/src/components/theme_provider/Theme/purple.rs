use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind purple palette
pub const Purple: Theme = Theme {
    common: r#"
  --color-purple-50: oklch(97.7% 0.014 308.299);
  --color-purple-100: oklch(94.6% 0.033 307.174);
  --color-purple-200: oklch(90.2% 0.063 306.703);
  --color-purple-300: oklch(82.7% 0.119 306.383);
  --color-purple-400: oklch(71.4% 0.203 305.504);
  --color-purple-500: oklch(62.7% 0.265 303.9);
  --color-purple-600: oklch(55.8% 0.288 302.321);
  --color-purple-700: oklch(49.6% 0.265 301.924);
  --color-purple-800: oklch(43.8% 0.218 303.724);
  --color-purple-900: oklch(38.1% 0.176 304.987);
  --color-purple-950: oklch(29.1% 0.149 302.717);

  --chart-1: var(--color-purple-300);
  --chart-2: var(--color-purple-500);
  --chart-3: var(--color-purple-600);
  --chart-4: var(--color-purple-700);
  --chart-5: var(--color-purple-800);
"#,
    dark: r#"
    --primary: var(--color-purple-500);
    --primary-foreground: var(--color-purple-50);
    --ring: var(--color-purple-900);
    --sidebar-primary: var(--color-purple-500);
    --sidebar-primary-foreground: var(--color-purple-50);
    --sidebar-ring: var(--color-purple-900);
"#,
    light: r#"
  --primary: var(--color-purple-600);
  --primary-foreground: var(--color-purple-50);
  --ring: var(--color-purple-400);
  --sidebar-primary: var(--color-purple-600);
  --sidebar-primary-foreground: var(--color-purple-50);
  --sidebar-ring: var(--color-purple-400);
"#,
};
