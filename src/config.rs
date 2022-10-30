use std::env::var;

use once_cell::sync::Lazy;

// Waiting for https://github.com/rust-lang/rust/issues/74465 to land, so we can reduce reliance on once_cell
static CONFIG: Lazy<Config> = Lazy::new(||Config::load());

#[derive(Default, serde::Deserialize)]
pub struct Config {
	sfw_only: Option<String>,
	default_theme: Option<String>,
	default_front_page: Option<String>,
	default_layout: Option<String>,
	default_wide: Option<String>,
	default_comment_sort: Option<String>,
	default_post_sort: Option<String>,
	default_show_nsfw: Option<String>,
	default_blur_nsfw: Option<String>,
	default_use_hls: Option<String>,
	default_hide_hls_notification: Option<String>,
}

impl Config {
	pub fn load() -> Self {
		// Read from ferris.toml config file. If for any reason, it fails, the default `Config` is used (all None values)
		let config: Config = toml::from_str(&std::fs::read_to_string("ferris.toml").unwrap_or_default()).unwrap_or_default();
		// This function defines the order of preference - first check for environment variables, then check the config, then if both are `None`,
		// return a `None` via the `map_or_else` function
		let parse = |key: &str| -> Option<String> { var(key).ok().map_or_else(|| get_setting_from_config(key, &config), Some) };
		Self {
			sfw_only: parse("FERRIT_SFW_ONLY"),
			default_theme: parse("FERRIT_DEFAULT_THEME"),
			default_front_page: parse("FERRIT_DEFAULT_FRONT_PAGE"),
			default_layout: parse("FERRIT_DEFAULT_LAYOUT"),
			default_post_sort: parse("FERRIT_DEFAULT_POST_SORT"),
			default_wide: parse("FERRIT_DEFAULT_WIDE"),
			default_comment_sort: parse("FERRIT_DEFAULT_COMMENT_SORT"),
			default_show_nsfw: parse("FERRIT_DEFAULT_SHOW_NSFW"),
			default_blur_nsfw: parse("FERRIT_DEFAULT_BLUR_NSFW"),
			default_use_hls: parse("FERRIT_DEFAULT_USE_HLS"),
			default_hide_hls_notification: parse("FERRIT_DEFAULT_HIDE_HLS"),
		}
	}
}
fn get_setting_from_config(name: &str, config: &Config) -> Option<String> {
	match name {
		"FERRIT_SFW_ONLY" => config.sfw_only.clone(),
		"FERRIT_DEFAULT_THEME" => config.default_theme.clone(),
		"FERRIT_DEFAULT_FRONT_PAGE" => config.default_front_page.clone(),
		"FERRIT_DEFAULT_LAYOUT" => config.default_layout.clone(),
		"FERRIT_DEFAULT_COMMENT_SORT" => config.default_comment_sort.clone(),
		"FERRIT_DEFAULT_POST_SORT" => config.default_post_sort.clone(),
		"FERRIT_DEFAULT_SHOW_NSFW" => config.default_show_nsfw.clone(),
		"FERRIT_DEFAULT_BLUR_NSFW" => config.default_blur_nsfw.clone(),
		"FERRIT_DEFAULT_USE_HLS" => config.default_use_hls.clone(),
		"FERRIT_DEFAULT_HIDE_HLS_NOTIFICATION" => config.default_hide_hls_notification.clone(),
		"FERRIT_DEFAULT_WIDE" => config.default_wide.clone(),
		_ => None,
	}
}
pub(crate) fn get_setting(name: &str) -> Option<String> {
	match name {
		"FERRIT_SFW_ONLY" => CONFIG.sfw_only.clone(),
		"FERRIT_DEFAULT_THEME" => CONFIG.default_theme.clone(),
		"FERRIT_DEFAULT_FRONT_PAGE" => CONFIG.default_front_page.clone(),
		"FERRIT_DEFAULT_LAYOUT" => CONFIG.default_layout.clone(),
		"FERRIT_DEFAULT_COMMENT_SORT" => CONFIG.default_comment_sort.clone(),
		"FERRIT_DEFAULT_POST_SORT" => CONFIG.default_post_sort.clone(),
		"FERRIT_DEFAULT_SHOW_NSFW" => CONFIG.default_show_nsfw.clone(),
		"FERRIT_DEFAULT_BLUR_NSFW" => CONFIG.default_blur_nsfw.clone(),
		"FERRIT_DEFAULT_USE_HLS" => CONFIG.default_use_hls.clone(),
		"FERRIT_DEFAULT_HIDE_HLS_NOTIFICATION" => CONFIG.default_hide_hls_notification.clone(),
		"FERRIT_DEFAULT_WIDE" => CONFIG.default_wide.clone(),
		_ => None,
	}
}
