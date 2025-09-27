use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind violet palette
pub const Violet: Theme = Theme {
    common: r#"
  --color-violet-50: oklch(96.9% 0.016 293.756);
  --color-violet-100: oklch(94.3% 0.029 294.588);
  --color-violet-200: oklch(89.4% 0.057 293.283);
  --color-violet-300: oklch(81.1% 0.111 293.571);
  --color-violet-400: oklch(70.2% 0.183 293.541);
  --color-violet-500: oklch(60.6% 0.25 292.717);
  --color-violet-600: oklch(54.1% 0.281 293.009);
  --color-violet-700: oklch(49.1% 0.27 292.581);
  --color-violet-800: oklch(43.2% 0.232 292.759);
  --color-violet-900: oklch(38% 0.189 293.745);
  --color-violet-950: oklch(28.3% 0.141 291.089);

  --chart-1: var(--color-violet-300);
  --chart-2: var(--color-violet-500);
  --chart-3: var(--color-violet-600);
  --chart-4: var(--color-violet-700);
  --chart-5: var(--color-violet-800);
"#,
    dark: r#"
    --primary: var(--color-violet-500);
    --primary-foreground: var(--color-violet-50);
    --ring: var(--color-violet-900);
    --sidebar-primary: var(--color-violet-500);
    --sidebar-primary-foreground: var(--color-violet-50);
    --sidebar-ring: var(--color-violet-900);
"#,
    light: r#"
  --primary: var(--color-violet-600);
  --primary-foreground: var(--color-violet-50);
  --ring: var(--color-violet-400);
  --sidebar-primary: var(--color-violet-600);
  --sidebar-primary-foreground: var(--color-violet-50);
  --sidebar-ring: var(--color-violet-400);
"#,
};
