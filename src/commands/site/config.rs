use cloudreve_api::SiteConfigValue;
use log::info;

pub fn display_config(config: &SiteConfigValue, section: &str) {
    match config {
        SiteConfigValue::V4(v4_config) => {
            display_v4_config(v4_config, section);
        }
        SiteConfigValue::V3(v3_config) => {
            display_v3_config(v3_config, section);
        }
    }
}

fn display_v4_config(config: &cloudreve_api::api::v4::models::SiteConfig, section: &str) {
    info!("Site Configuration [{}]:", section);

    match section {
        "basic" => {
            if let Some(title) = &config.title {
                info!("  Title: {}", title);
            }
            if let Some(instance_id) = &config.instance_id {
                info!("  Instance ID: {}", instance_id);
            }
            if let Some(logo) = &config.logo {
                info!("  Logo: {}", logo);
            }
            if let Some(logo_light) = &config.logo_light {
                info!("  Logo Light: {}", logo_light);
            }
            if let Some(default_theme) = &config.default_theme {
                info!("  Default Theme: {}", default_theme);
            }
        }
        "login" => {
            if let Some(login_captcha) = config.login_captcha {
                info!("  Login Captcha: {}", login_captcha);
            }
            if let Some(reg_captcha) = config.reg_captcha {
                info!("  Registration Captcha: {}", reg_captcha);
            }
            if let Some(register_enabled) = config.register_enabled {
                info!("  Registration Enabled: {}", register_enabled);
            }
            if let Some(authn) = config.authn {
                info!("  Passkey Auth: {}", authn);
            }
            if let Some(captcha_type) = &config.captcha_type {
                info!("  Captcha Type: {}", captcha_type);
            }
            if let Some(tos_url) = &config.tos_url {
                info!("  ToS URL: {}", tos_url);
            }
            if let Some(privacy_policy_url) = &config.privacy_policy_url {
                info!("  Privacy Policy URL: {}", privacy_policy_url);
            }
        }
        "explorer" => {
            if let Some(site_notice) = &config.site_notice {
                info!("  Site Notice: {}", site_notice);
            }
            if let Some(map_provider) = &config.map_provider {
                info!("  Map Provider: {}", map_provider);
            }
            if let Some(max_batch_size) = config.max_batch_size {
                info!("  Max Batch Size: {}", max_batch_size);
            }
            if let Some(thumbnail_width) = config.thumbnail_width {
                info!("  Thumbnail Width: {}", thumbnail_width);
            }
            if let Some(thumbnail_height) = config.thumbnail_height {
                info!("  Thumbnail Height: {}", thumbnail_height);
            }
            if let Some(custom_props) = &config.custom_props {
                info!("  Custom Properties ({}):", custom_props.len());
                for prop in custom_props {
                    info!("    - {}: {} ({})", prop.name, prop.r#type, prop.key);
                }
            }
        }
        "emojis" => {
            if let Some(emoji_preset) = &config.emoji_preset {
                info!("  Emoji Preset: {}", emoji_preset);
            }
        }
        "vas" => {
            if let Some(point_enabled) = config.point_enabled {
                info!("  Point Enabled: {}", point_enabled);
            }
            if let Some(share_point_gain_rate) = config.share_point_gain_rate {
                info!("  Share Point Gain Rate: {}", share_point_gain_rate);
            }
            if let Some(app_promotion) = config.app_promotion {
                info!("  App Promotion: {}", app_promotion);
            }
        }
        "app" => {
            if let Some(app_promotion) = config.app_promotion {
                info!("  App Promotion: {}", app_promotion);
            }
            if let Some(app_feedback) = &config.app_feedback {
                info!("  App Feedback: {}", app_feedback);
            }
            if let Some(app_forum) = &config.app_forum {
                info!("  App Forum: {}", app_forum);
            }
        }
        "thumb" => {
            if let Some(thumbnail_width) = config.thumbnail_width {
                info!("  Thumbnail Width: {}", thumbnail_width);
            }
            if let Some(thumbnail_height) = config.thumbnail_height {
                info!("  Thumbnail Height: {}", thumbnail_height);
            }
            if let Some(thumb_exts) = &config.thumb_exts {
                info!("  Thumbnail Extensions: {:?}", thumb_exts);
            }
        }
        _ => {
            info!("(Section '{}' not handled, showing raw JSON)", section);
        }
    }
}

fn display_v3_config(config: &cloudreve_api::api::v3::models::SiteConfig, section: &str) {
    info!("Site Configuration [V3 - {}]:", section);

    match section {
        "basic" => {
            info!("  Title: {}", config.title);
            if !config.default_theme.is_empty() {
                info!("  Default Theme: {}", config.default_theme);
            }
            if !config.home_view_method.is_empty() {
                info!("  Home View Method: {}", config.home_view_method);
            }
            if !config.share_view_method.is_empty() {
                info!("  Share View Method: {}", config.share_view_method);
            }
            if config.app_promotion {
                info!("  App Promotion: {}", config.app_promotion);
            }
            if config.direct_link_batch_size > 0 {
                info!(
                    "  Direct Link Batch Size: {}",
                    config.direct_link_batch_size
                );
            }
        }
        "login" => {
            info!("  Login Captcha: {}", config.login_captcha);
            info!("  Registration Captcha: {}", config.reg_captcha);
            info!("  Forget Captcha: {}", config.forget_captcha);
            info!("  Registration Enabled: {}", config.register_enabled);
            info!("  Email Active: {}", config.email_active);
            info!("  Passkey Auth: {}", config.authn);
            if !config.captcha_type.is_empty() {
                info!("  Captcha Type: {}", config.captcha_type);
            }
            if !config.captcha_recaptcha_key.is_empty() {
                info!("  reCAPTCHA Key: {}", config.captcha_recaptcha_key);
            }
        }
        "explorer" => {
            // V3 doesn't have a separate explorer section
            info!("  (Explorer section not available in V3 API)");
        }
        _ => {
            info!("(Section '{}' not handled for V3 API)", section);
        }
    }
}
