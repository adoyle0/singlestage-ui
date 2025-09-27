use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind teal palette
pub const Teal: Theme = Theme {
    common: r#"
  --color-teal-50: oklch(98.4% 0.014 180.72);
  --color-teal-100: oklch(95.3% 0.051 180.801);
  --color-teal-200: oklch(91% 0.096 180.426);
  --color-teal-300: oklch(85.5% 0.138 181.071);
  --color-teal-400: oklch(77.7% 0.152 181.912);
  --color-teal-500: oklch(70.4% 0.14 182.503);
  --color-teal-600: oklch(60% 0.118 184.704);
  --color-teal-700: oklch(51.1% 0.096 186.391);
  --color-teal-800: oklch(43.7% 0.078 188.216);
  --color-teal-900: oklch(38.6% 0.063 188.416);
  --color-teal-950: oklch(27.7% 0.046 192.524);

  --chart-1: var(--color-teal-300);
  --chart-2: var(--color-teal-500);
  --chart-3: var(--color-teal-600);
  --chart-4: var(--color-teal-700);
  --chart-5: var(--color-teal-800);
"#,
    dark: r#"
    --primary: var(--color-teal-500);
    --primary-foreground: var(--color-teal-50);
    --ring: var(--color-teal-900);
    --sidebar-primary: var(--color-teal-500);
    --sidebar-primary-foreground: var(--color-teal-50);
    --sidebar-ring: var(--color-teal-900);
"#,
    light: r#"
  --primary: var(--color-teal-600);
  --primary-foreground: var(--color-teal-50);
  --ring: var(--color-teal-400);
  --sidebar-primary: var(--color-teal-600);
  --sidebar-primary-foreground: var(--color-teal-50);
  --sidebar-ring: var(--color-teal-400);
"#,
};
