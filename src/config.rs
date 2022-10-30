use std::env::var;

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
	pub fn new() -> Self {
		// Read from ferris.toml config file. If for any reason, it fails, the default `Config` is used (all None values)
		let config: Config = toml::from_str(&std::fs::read_to_string("ferris.toml").unwrap_or_default()).unwrap_or_default();
		// This function defines the order of preference - first check for environment variables, then check the config, then if both are `None`,
		// return a `None` via the `map_or_else` function
		let parse = |key: &str| -> Option<String> { var(key).ok().map_or_else(|| config.get_setting(key), Some) };
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
	pub(crate) fn get_setting(&self, name: &str) -> Option<String> {
		match name {
			"FERRIT_SFW_ONLY" => self.sfw_only.clone(),
			"FERRIT_DEFAULT_THEME" => self.default_theme.clone(),
			"FERRIT_DEFAULT_FRONT_PAGE" => self.default_front_page.clone(),
			"FERRIT_DEFAULT_LAYOUT" => self.default_layout.clone(),
			"FERRIT_DEFAULT_COMMENT_SORT" => self.default_comment_sort.clone(),
			"FERRIT_DEFAULT_POST_SORT" => self.default_post_sort.clone(),
			"FERRIT_DEFAULT_SHOW_NSFW" => self.default_show_nsfw.clone(),
			"FERRIT_DEFAULT_BLUR_NSFW" => self.default_blur_nsfw.clone(),
			"FERRIT_DEFAULT_USE_HLS" => self.default_use_hls.clone(),
			"FERRIT_DEFAULT_HIDE_HLS_NOTIFICATION" => self.default_hide_hls_notification.clone(),
			"FERRIT_DEFAULT_WIDE" => self.default_wide.clone(),
			_ => None,
		}
	}
}
