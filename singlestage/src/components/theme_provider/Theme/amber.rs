use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind amber palette
pub const Amber: Theme = Theme {
    common: r#"
  --color-amber-50: oklch(98.7% 0.022 95.277);
  --color-amber-100: oklch(96.2% 0.059 95.617);
  --color-amber-200: oklch(92.4% 0.12 95.746);
  --color-amber-300: oklch(87.9% 0.169 91.605);
  --color-amber-400: oklch(82.8% 0.189 84.429);
  --color-amber-500: oklch(76.9% 0.188 70.08);
  --color-amber-600: oklch(66.6% 0.179 58.318);
  --color-amber-700: oklch(55.5% 0.163 48.998);
  --color-amber-800: oklch(47.3% 0.137 46.201);
  --color-amber-900: oklch(41.4% 0.112 45.904);
  --color-amber-950: oklch(27.9% 0.077 45.635);

  --chart-1: var(--color-amber-300);
  --chart-2: var(--color-amber-500);
  --chart-3: var(--color-amber-600);
  --chart-4: var(--color-amber-700);
  --chart-5: var(--color-amber-800);
"#,
    dark: r#"
    --primary: var(--color-amber-500);
    --primary-foreground: var(--color-amber-950);
    --ring: var(--color-amber-900);
    --sidebar-primary: var(--color-amber-500);
    --sidebar-primary-foreground: var(--color-amber-950);
    --sidebar-ring: var(--color-amber-900);
"#,
    light: r#"
  --primary: var(--color-amber-600);
  --primary-foreground: var(--color-amber-950);
  --ring: var(--color-amber-400);
  --sidebar-primary: var(--color-amber-600);
  --sidebar-primary-foreground: var(--color-amber-950);
  --sidebar-ring: var(--color-amber-400);
"#,
};
