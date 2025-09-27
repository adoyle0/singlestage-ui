use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind orange palette
pub const Orange: Theme = Theme {
    common: r#"
  --color-orange-50: oklch(98% 0.016 73.684);
  --color-orange-100: oklch(95.4% 0.038 75.164);
  --color-orange-200: oklch(90.1% 0.076 70.697);
  --color-orange-300: oklch(83.7% 0.128 66.29);
  --color-orange-400: oklch(75% 0.183 55.934);
  --color-orange-500: oklch(70.5% 0.213 47.604);
  --color-orange-600: oklch(64.6% 0.222 41.116);
  --color-orange-700: oklch(55.3% 0.195 38.402);
  --color-orange-800: oklch(47% 0.157 37.304);
  --color-orange-900: oklch(40.8% 0.123 38.172);
  --color-orange-950: oklch(26.6% 0.079 36.259);

  --chart-1: var(--color-orange-300);
  --chart-2: var(--color-orange-500);
  --chart-3: var(--color-orange-600);
  --chart-4: var(--color-orange-700);
  --chart-5: var(--color-orange-800);
"#,
    dark: r#"
    --primary: var(--color-orange-500);
    --primary-foreground: var(--color-orange-50);
    --ring: var(--color-orange-900);
    --sidebar-primary: var(--color-orange-500);
    --sidebar-primary-foreground: var(--color-orange-50);
    --sidebar-ring: var(--color-orange-900);
"#,
    light: r#"
  --primary: var(--color-orange-600);
  --primary-foreground: var(--color-orange-50);
  --ring: var(--color-orange-400);
  --sidebar-primary: var(--color-orange-600);
  --sidebar-primary-foreground: var(--color-orange-50);
  --sidebar-ring: var(--color-orange-400);
"#,
};
