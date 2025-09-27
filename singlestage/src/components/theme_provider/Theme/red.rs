use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind red palette
pub const Red: Theme = Theme {
    common: r#"
  --color-red-50: oklch(97.1% 0.013 17.38);
  --color-red-100: oklch(93.6% 0.032 17.717);
  --color-red-200: oklch(88.5% 0.062 18.334);
  --color-red-300: oklch(80.8% 0.114 19.571);
  --color-red-400: oklch(70.4% 0.191 22.216);
  --color-red-500: oklch(63.7% 0.237 25.331);
  --color-red-600: oklch(57.7% 0.245 27.325);
  --color-red-700: oklch(50.5% 0.213 27.518);
  --color-red-800: oklch(44.4% 0.177 26.899);
  --color-red-900: oklch(39.6% 0.141 25.723);
  --color-red-950: oklch(25.8% 0.092 26.042);

  --chart-1: var(--color-red-300);
  --chart-2: var(--color-red-500);
  --chart-3: var(--color-red-600);
  --chart-4: var(--color-red-700);
  --chart-5: var(--color-red-800);
"#,
    dark: r#"
    --primary: var(--color-red-500);
    --primary-foreground: var(--color-red-50);
    --ring: var(--color-red-900);
    --sidebar-primary: var(--color-red-500);
    --sidebar-primary-foreground: var(--color-red-50);
    --sidebar-ring: var(--color-red-900);
"#,
    light: r#"
  --primary: var(--color-red-600);
  --primary-foreground: var(--color-red-50);
  --ring: var(--color-red-400);
  --sidebar-primary: var(--color-red-600);
  --sidebar-primary-foreground: var(--color-red-50);
  --sidebar-ring: var(--color-red-400);
"#,
};
