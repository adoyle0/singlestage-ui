use super::Theme;

#[allow(non_upper_case_globals)]
/// Uses tailwind rose palette
pub const Rose: Theme = Theme {
    common: r#"
  --color-rose-50: oklch(96.9% 0.015 12.422);
  --color-rose-100: oklch(94.1% 0.03 12.58);
  --color-rose-200: oklch(89.2% 0.058 10.001);
  --color-rose-300: oklch(81% 0.117 11.638);
  --color-rose-400: oklch(71.2% 0.194 13.428);
  --color-rose-500: oklch(64.5% 0.246 16.439);
  --color-rose-600: oklch(58.6% 0.253 17.585);
  --color-rose-700: oklch(51.4% 0.222 16.935);
  --color-rose-800: oklch(45.5% 0.188 13.697);
  --color-rose-900: oklch(41% 0.159 10.272);
  --color-rose-950: oklch(27.1% 0.105 12.094);

  --chart-1: var(--color-rose-300);
  --chart-2: var(--color-rose-500);
  --chart-3: var(--color-rose-600);
  --chart-4: var(--color-rose-700);
  --chart-5: var(--color-rose-800);
"#,
    dark: r#"
    --primary: var(--color-rose-500);
    --primary-foreground: var(--color-rose-50);
    --ring: var(--color-rose-900);
    --sidebar-primary: var(--color-rose-500);
    --sidebar-primary-foreground: var(--color-rose-50);
    --sidebar-ring: var(--color-rose-900);
"#,
    light: r#"
  --primary: var(--color-rose-600);
  --primary-foreground: var(--color-rose-50);
  --ring: var(--color-rose-400);
  --sidebar-primary: var(--color-rose-600);
  --sidebar-primary-foreground: var(--color-rose-50);
  --sidebar-ring: var(--color-rose-400);
"#,
};
