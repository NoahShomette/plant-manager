use std::collections::HashMap;

use thaw::{ColorTheme, CommonTheme, Theme};

pub(super) fn update_theme() -> Theme {
    let mut theme = Theme {
        name: "dark".into(),
        common: CommonTheme::new(),
        color: ColorTheme::custom_dark(&HashMap::from([
            (10, "#020402"),
            (20, "#020402"),
            (30, "#1B2E1A"),
            (40, "#213B20"),
            (50, "#264926"),
            (60, "#2C572C"),
            (70, "#326632"),
            (80, "#377538"),
            (90, "#439545"),
            (100, "#439545"),
            (110, "#48A54C"),
            (120, "#58B45A"),
            (130, "#76C074"),
            (140, "#92CD8E"),
            (150, "#ACD9A8"),
            (160, "#C6E5C3"),
        ])),
    };

    theme
        .color
        .set_color_neutral_background_static("#1a211a".to_string());
    theme
        .color
        .set_color_neutral_background_inverted("#1a211a".to_string());
    theme
        .color
        .set_color_neutral_background_disabled("#1a211a".to_string());

    // Button
    theme
        .color
        .set_color_neutral_background_1("#1f291f".to_string());
    theme
        .color
        .set_color_neutral_background_1_hover("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_background_1_pressed("#4d442a".to_string());

    theme
        .color
        .set_color_neutral_background_3("#1f291f".to_string());
    theme
        .color
        .set_color_neutral_background_3_hover("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_background_3_pressed("#4d442a".to_string());

    theme
        .color
        .set_color_neutral_background_4("#1f291f".to_string());
    theme
        .color
        .set_color_neutral_background_4_hover("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_background_4_pressed("#4d442a".to_string());

    theme
        .color
        .set_color_neutral_background_5("#1f291f".to_string());

    theme
        .color
        .set_color_neutral_background_6("#1f291f".to_string());

    ////

    theme
        .color
        .set_color_neutral_foreground_static_inverted("#c8d4c8".to_string());
    theme
        .color
        .set_color_neutral_foreground_disabled("#c8d4c8".to_string());

    theme
        .color
        .set_color_neutral_foreground_1("#e0e6e0".to_string());
    theme
        .color
        .set_color_neutral_foreground_1_hover("#e0e6e0".to_string());
    theme
        .color
        .set_color_neutral_foreground_1_pressed("#f0e6d0".to_string());

    theme
        .color
        .set_color_neutral_foreground_2("#e0e6e0".to_string());
    theme
        .color
        .set_color_neutral_foreground_2_hover("#94a3b8".to_string());
    theme
        .color
        .set_color_neutral_foreground_2_pressed("#f0e6d0".to_string());

    theme
        .color
        .set_color_neutral_foreground_2_brand_hover("#d4af37".to_string());
    theme
        .color
        .set_color_neutral_foreground_2_brand_pressed("#d4af37".to_string());
    theme
        .color
        .set_color_neutral_foreground_2_brand_selected("#d4af37".to_string());

    theme
        .color
        .set_color_neutral_foreground_3("#c8d4c8".to_string());

    theme
        .color
        .set_color_neutral_foreground_4("#94a3b8".to_string());

    theme
        .color
        .set_color_neutral_foreground_on_brand("#d4af37".to_string());

    theme
        .color
        .set_color_neutral_foreground_inverted("#d4af37".to_string());

    //

    theme
        .color
        .set_color_neutral_stroke_disabled("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_1("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_1_hover("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_1_pressed("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_2("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible_hover("#2e3b2e".to_string());
    theme
        .color
        .set_color_neutral_stroke_accessible_pressed("#2e3b2e".to_string());

    //

    theme
        .color
        .set_color_neutral_shadow_ambient("#1a211a".to_string());
    theme
        .color
        .set_color_neutral_shadow_key("#1a211a".to_string());

    theme
        .color
        .set_color_neutral_stencil_1("#c8d4c8".to_string());
    theme
        .color
        .set_color_neutral_stencil_2("#c8d4c8".to_string());

    theme
        .color
        .set_color_compound_brand_foreground_1("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_foreground_1_hover("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_foreground_1_pressed("#d4af37".to_string());

    theme
        .color
        .set_color_compound_brand_background("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_background_hover("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_background_pressed("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_stroke("#d4af37".to_string());
    theme
        .color
        .set_color_compound_brand_stroke_pressed("#d4af37".to_string());

    theme
        .color
        .set_color_brand_background("#d4af37".to_string());
    theme
        .color
        .set_color_brand_background_hover("#d4af37".to_string());
    theme
        .color
        .set_color_brand_background_pressed("#d4af37".to_string());

    theme
        .color
        .set_color_brand_background_2("#d4af37".to_string());
    theme
        .color
        .set_color_brand_foreground_1("#d4af37".to_string());
    theme
        .color
        .set_color_brand_foreground_2("#d4af37".to_string());
    theme.color.set_color_brand_stroke_1("#d4af37".to_string());
    theme.color.set_color_brand_stroke_2("#d4af37".to_string());
    theme
        .color
        .set_color_brand_stroke_2_contrast("#d4af37".to_string());
    theme
        .color
        .set_color_brand_foreground_link("#c8d4c8".to_string());
    theme
        .color
        .set_color_brand_foreground_link_hover("#e0e6e0".to_string());
    theme
        .color
        .set_color_brand_foreground_link_pressed("#d4af37".to_string());

    theme
}

// .dark {
//   --background: #1a211a;
//   --foreground: #c8d4c8;
//   --card: #1f291f;
//   --card-foreground: #e0e6e0;
//   --popover: #1f291f;
//   --popover-foreground: #e0e6e0;
//   --primary: #4caf50;
//   --primary-foreground: #ffffff;
//   --secondary: #d4af37;
//   --secondary-foreground: #1a211a;
//   --muted: #2e3b2e;
//   --muted-foreground: #94a3b8;
//   --accent: #4d442a;
//   --accent-foreground: #f0e6d0;
//   --destructive: #7f1d1d;
//   --destructive-foreground: #fde8e8;
//   --border: #2e3b2e;
//   --input: #2e3b2e;
//   --ring: #4caf50;
//   --chart-1: #4caf50;
//   --chart-2: #8bc34a;
//   --chart-3: #cddc39;
//   --chart-4: #ffeb3b;
//   --chart-5: #ffc107;
//   --sidebar: #1f291f;
//   --sidebar-foreground: #c8d4c8;
//   --sidebar-primary: #d4af37;
//   --sidebar-primary-foreground: #1a211a;
//   --sidebar-accent: #4d442a;
//   --sidebar-accent-foreground: #f0e6d0;
//   --sidebar-border: #2e3b2e;
//   --sidebar-ring: #d4af37;
//   --font-sans: Inter, sans-serif;
//   --font-serif: Lora, serif;
//   --font-mono: IBM Plex Mono, monospace;
//   --radius: 0.5rem;
//   --shadow-2xs: 0px 4px 12px -2px hsl(0 0% 0% / 0.05);
//   --shadow-xs: 0px 4px 12px -2px hsl(0 0% 0% / 0.05);
//   --shadow-sm: 0px 4px 12px -2px hsl(0 0% 0% / 0.10), 0px 1px 2px -3px hsl(0 0% 0% / 0.10);
//   --shadow: 0px 4px 12px -2px hsl(0 0% 0% / 0.10), 0px 1px 2px -3px hsl(0 0% 0% / 0.10);
//   --shadow-md: 0px 4px 12px -2px hsl(0 0% 0% / 0.10), 0px 2px 4px -3px hsl(0 0% 0% / 0.10);
//   --shadow-lg: 0px 4px 12px -2px hsl(0 0% 0% / 0.10), 0px 4px 6px -3px hsl(0 0% 0% / 0.10);
//   --shadow-xl: 0px 4px 12px -2px hsl(0 0% 0% / 0.10), 0px 8px 10px -3px hsl(0 0% 0% / 0.10);
//   --shadow-2xl: 0px 4px 12px -2px hsl(0 0% 0% / 0.25);
// }

// @theme inline {
//   --color-background: var(--background);
//   --color-foreground: var(--foreground);
//   --color-card: var(--card);
//   --color-card-foreground: var(--card-foreground);
//   --color-popover: var(--popover);
//   --color-popover-foreground: var(--popover-foreground);
//   --color-primary: var(--primary);
//   --color-primary-foreground: var(--primary-foreground);
//   --color-secondary: var(--secondary);
//   --color-secondary-foreground: var(--secondary-foreground);
//   --color-muted: var(--muted);
//   --color-muted-foreground: var(--muted-foreground);
//   --color-accent: var(--accent);
//   --color-accent-foreground: var(--accent-foreground);
//   --color-destructive: var(--destructive);
//   --color-destructive-foreground: var(--destructive-foreground);
//   --color-border: var(--border);
//   --color-input: var(--input);
//   --color-ring: var(--ring);
//   --color-chart-1: var(--chart-1);
//   --color-chart-2: var(--chart-2);
//   --color-chart-3: var(--chart-3);
//   --color-chart-4: var(--chart-4);
//   --color-chart-5: var(--chart-5);
//   --color-sidebar: var(--sidebar);
//   --color-sidebar-foreground: var(--sidebar-foreground);
//   --color-sidebar-primary: var(--sidebar-primary);
//   --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
//   --color-sidebar-accent: var(--sidebar-accent);
//   --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
//   --color-sidebar-border: var(--sidebar-border);
//   --color-sidebar-ring: var(--sidebar-ring);

//   --font-sans: var(--font-sans);
//   --font-mono: var(--font-mono);
//   --font-serif: var(--font-serif);

//   --radius-sm: calc(var(--radius) - 4px);
//   --radius-md: calc(var(--radius) - 2px);
//   --radius-lg: var(--radius);
//   --radius-xl: calc(var(--radius) + 4px);

//   --shadow-2xs: var(--shadow-2xs);
//   --shadow-xs: var(--shadow-xs);
//   --shadow-sm: var(--shadow-sm);
//   --shadow: var(--shadow);
//   --shadow-md: var(--shadow-md);
//   --shadow-lg: var(--shadow-lg);
//   --shadow-xl: var(--shadow-xl);
//   --shadow-2xl: var(--shadow-2xl);

//   --tracking-tighter: calc(var(--tracking-normal) - 0.05em);
//   --tracking-tight: calc(var(--tracking-normal) - 0.025em);
//   --tracking-normal: var(--tracking-normal);
//   --tracking-wide: calc(var(--tracking-normal) + 0.025em);
//   --tracking-wider: calc(var(--tracking-normal) + 0.05em);
//   --tracking-widest: calc(var(--tracking-normal) + 0.1em);
// }

// body {
//   letter-spacing: var(--tracking-normal);
// }
