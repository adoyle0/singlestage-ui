use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind blue palette
pub const Blue: Theme = Theme {
    common: r#"
  --color-blue-50: oklch(97% 0.014 254.604);
  --color-blue-100: oklch(93.2% 0.032 255.585);
  --color-blue-200: oklch(88.2% 0.059 254.128);
  --color-blue-300: oklch(80.9% 0.105 251.813);
  --color-blue-400: oklch(70.7% 0.165 254.624);
  --color-blue-500: oklch(62.3% 0.214 259.815);
  --color-blue-600: oklch(54.6% 0.245 262.881);
  --color-blue-700: oklch(48.8% 0.243 264.376);
  --color-blue-800: oklch(42.4% 0.199 265.638);
  --color-blue-900: oklch(37.9% 0.146 265.522);
  --color-blue-950: oklch(28.2% 0.091 267.935);

  --chart-1: var(--color-blue-300);
  --chart-2: var(--color-blue-500);
  --chart-3: var(--color-blue-600);
  --chart-4: var(--color-blue-700);
  --chart-5: var(--color-blue-800);
"#,
    dark: r#"
    --primary: var(--color-blue-500);
    --primary-foreground: var(--color-blue-50);
    --ring: var(--color-blue-900);
    --sidebar-primary: var(--color-blue-500);
    --sidebar-primary-foreground: var(--color-blue-50);
    --sidebar-ring: var(--color-blue-900);
"#,
    light: r#"
  --primary: var(--color-blue-600);
  --primary-foreground: var(--color-blue-50);
  --ring: var(--color-blue-400);
  --sidebar-primary: var(--color-blue-600);
  --sidebar-primary-foreground: var(--color-blue-50);
  --sidebar-ring: var(--color-blue-400);
"#,
};
