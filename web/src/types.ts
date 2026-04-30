export type NamedHues = {
  red: string;
  orange: string;
  yellow: string;
  green: string;
  cyan: string;
  blue: string;
  purple: string;
  magenta: string;
};

export type Surfaces = {
  surface_low: string;
  base: string;
  base_high: string;
  surface_high: string;
  surface_higher: string;
  surface_highest: string;
  faint: string;
  muted: string;
  subtext: string;
  text: string;
  black: string;
  white: string;
};

export type Scheme = {
  is_light_theme: boolean;
  surfaces: Surfaces;
  high_contrast_fg_accents: string[];
  fg_accents: string[];
  rg_accents: string[];
  bg_accents: string[];
  high_contrast_fg_named: NamedHues;
  fg_named: NamedHues;
  rg_named: NamedHues;
  bg_named: NamedHues;
};

export type ThemeConfig = {
  base_lightness_minimum: number;
  base_lightness_maximum: number;
  surface_distance: number;
  set_2_lightness_correction: number;
  faint_dps_contrast: number;
  set_3_dps_contrast: number;
  set_4_dps_contrast: number;
  set_5_dps_contrast: number;
  neutral_chroma_blend: number;
  bg_neutral_chroma_intercept: number;
  bg_neutral_lightness_to_chroma_slope: number;
  bg_neutral_hue_low_point: number;
  bg_neutral_low_point_chroma_intercept: number;
  fg_neutral_chroma_intercept: number;
  fg_neutral_lightness_to_chroma_slope: number;
  fg_neutral_hue_low_point: number;
  fg_neutral_low_point_chroma_intercept: number;
  prefered_hue_angle: number;
  minimum_hue_angle: number;
  chroma_weight_priority: number;
  penalty_weight_priority: number;
  maximum_accent_hue_center_translation: number;
  high_contrast_fg_accent_radius_baseline: number;
  fg_accent_radius_baseline: number;
  rg_accent_radius_baseline: number;
  bg_accent_radius_baseline: number;
  red_chroma_minimum: number;
  orange_chroma_minimum: number;
  yellow_chroma_minimum: number;
  green_chroma_minimum: number;
};

export type Config = {
  k_means_count: number;
  light_theme_threshold: number;
  light: ThemeConfig;
  dark: ThemeConfig;
};

export const DEFAULT_LIGHT: ThemeConfig = {
  base_lightness_minimum: 0.964,
  base_lightness_maximum: 0.964,
  surface_distance: 0.0225,
  set_2_lightness_correction: 0.01,
  faint_dps_contrast: 15.0,
  set_3_dps_contrast: 40.0,
  set_4_dps_contrast: 65.0,
  set_5_dps_contrast: 80.0,
  neutral_chroma_blend: 1.0,
  bg_neutral_chroma_intercept: 0.492,
  bg_neutral_lightness_to_chroma_slope: -0.482,
  bg_neutral_hue_low_point: 230.0,
  bg_neutral_low_point_chroma_intercept: 0.466,
  fg_neutral_chroma_intercept: 0.05,
  fg_neutral_lightness_to_chroma_slope: 0.0,
  fg_neutral_hue_low_point: 90.0,
  fg_neutral_low_point_chroma_intercept: 0.03,
  prefered_hue_angle: 50.0,
  minimum_hue_angle: 45.0,
  chroma_weight_priority: 0.015,
  penalty_weight_priority: 2.0,
  maximum_accent_hue_center_translation: 0.005,
  high_contrast_fg_accent_radius_baseline: 0.02,
  fg_accent_radius_baseline: 0.09,
  rg_accent_radius_baseline: 0.065,
  bg_accent_radius_baseline: 0.0175,
  red_chroma_minimum: 0.12,
  orange_chroma_minimum: 0.08,
  yellow_chroma_minimum: 0.08,
  green_chroma_minimum: 0.07,
};

export const DEFAULT_DARK: ThemeConfig = {
  base_lightness_minimum: 0.1,
  base_lightness_maximum: 0.185,
  surface_distance: 0.06,
  set_2_lightness_correction: 0.01,
  faint_dps_contrast: 15.0,
  set_3_dps_contrast: 30.0,
  set_4_dps_contrast: 72.5,
  set_5_dps_contrast: 82.5,
  neutral_chroma_blend: 1.0,
  bg_neutral_chroma_intercept: 0.07,
  bg_neutral_lightness_to_chroma_slope: 0.0833,
  bg_neutral_hue_low_point: 270.0,
  bg_neutral_low_point_chroma_intercept: 0.025,
  fg_neutral_chroma_intercept: 0.02,
  fg_neutral_lightness_to_chroma_slope: 0.0,
  fg_neutral_hue_low_point: 90.0,
  fg_neutral_low_point_chroma_intercept: 0.01,
  prefered_hue_angle: 50.0,
  minimum_hue_angle: 45.0,
  chroma_weight_priority: 0.015,
  penalty_weight_priority: 2.0,
  maximum_accent_hue_center_translation: 0.005,
  high_contrast_fg_accent_radius_baseline: 0.035,
  fg_accent_radius_baseline: 0.08,
  rg_accent_radius_baseline: 0.07,
  bg_accent_radius_baseline: 0.06,
  red_chroma_minimum: 0.12,
  orange_chroma_minimum: 0.08,
  yellow_chroma_minimum: 0.08,
  green_chroma_minimum: 0.07,
};

export const DEFAULT_CONFIG: Config = {
  k_means_count: 255,
  light_theme_threshold: 0.55,
  light: DEFAULT_LIGHT,
  dark: DEFAULT_DARK,
};
