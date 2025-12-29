pub const BG_GIF_BASE64: &str = include_str!("../bg.b64");
pub const GLASS_CSS: &str = include_str!("glass.css");

pub fn get_custom_css() -> String {
    GLASS_CSS.replace("BG_PLACEHOLDER", &format!("data:image/gif;base64,{}", BG_GIF_BASE64))
}
