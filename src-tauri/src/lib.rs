mod theme;

use tauri::WebviewWindowBuilder;
use theme::BG_GIF_BASE64;

fn get_custom_css() -> String {
    format!(r#"
:root {{
    --glass: rgba(0, 0, 0, 0.35);
    --glass-border: rgba(255, 255, 255, 0.1);
    --accent: #f50;
}}

html {{
    background: transparent !important;
}}

body {{
    background: url('data:image/gif;base64,{bg}') center center / cover no-repeat fixed !important;
}}

div, section, main, aside, nav, article, footer, .l-main, .l-content, 
.l-container, .l-fluid-fixed, .stream, .userMain, .l-listen-wrapper,
.streamContext, .l-about-main, .l-body, .l-nav, .l-fixed-top-spacer {{
    background-color: transparent !important;
    background-image: none !important;
}}

.header, .l-top-full-width, header, .headerScene, .g-z-index-header {{
    background: var(--glass) !important;
    backdrop-filter: blur(25px) !important;
    -webkit-backdrop-filter: blur(25px) !important;
    border-bottom: 1px solid var(--glass-border) !important;
}}

.playControls {{
    background: var(--glass) !important;
    backdrop-filter: blur(30px) !important;
    -webkit-backdrop-filter: blur(30px) !important;
    border-top: 1px solid var(--glass-border) !important;
}}

.playControls__bg, .playControls__inner {{
    background: transparent !important;
}}

.l-sidebar-right {{
    background: var(--glass) !important;
    backdrop-filter: blur(20px) !important;
    -webkit-backdrop-filter: blur(20px) !important;
    border-left: 1px solid var(--glass-border) !important;
}}

.sidebarModule {{
    background: rgba(0, 0, 0, 0.3) !important;
    backdrop-filter: blur(10px) !important;
    border: 1px solid var(--glass-border) !important;
    border-radius: 12px !important;
    margin: 8px !important;
}}

.sidebarModule:hover {{
    background: rgba(255, 85, 0, 0.15) !important;
    border-color: rgba(255, 85, 0, 0.4) !important;
}}

.sound__coverArt, .sc-artwork, .image__lightOutline, .image__full {{
    border-radius: 8px !important;
    box-shadow: 0 4px 20px rgba(0,0,0,0.6) !important;
}}

.sound__coverArt:hover {{
    transform: scale(1.03) !important;
}}

.playButton, .sc-button-play {{
    background: linear-gradient(135deg, var(--accent), #ff6b2b) !important;
    border: none !important;
}}

.playButton:hover {{
    transform: scale(1.1) !important;
}}

.playbackTimeline__progressWrapper {{
    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
    height: 3px !important;
    position: relative !important;
}}

.playbackTimeline, .playbackTimeline__mainWrapper {{
    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
}}

.playbackTimeline__progressBackground {{
    background: var(--accent) !important;
    border: none !important;
    box-shadow: none !important;
    border-radius: 2px !important;
}}

.playbackTimeline__progressHandle {{
    background: #fff !important;
    border: 2px solid var(--accent) !important;
    border-radius: 50% !important;
    width: 12px !important;
    height: 12px !important;
    box-shadow: none !important;
    opacity: 1 !important;
    visibility: visible !important;
    top: 50% !important;
    transform: translateY(-50%) !important;
    margin-top: 0 !important;
}}

.volume__sliderWrapper {{
    background: transparent !important;
    backdrop-filter: blur(20px) !important;
    border: none !important;
}}

.volume__sliderBackground {{
    background: rgba(255,255,255,0.3) !important;
    border-radius: 2px !important;
}}

.volume__sliderProgress {{
    background: var(--accent) !important;
}}

.volume__sliderHandle {{
    background: #fff !important;
    border: 2px solid var(--accent) !important;
    border-radius: 50% !important;
}}

.sc-button-addtoset, .sc-button-like, .sc-button-repost, .sc-button-share,
.sc-button-more, .playbackSoundBadge__actions .sc-button {{
    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
}}

.sc-button-addtoset:hover, .sc-button-like:hover, .sc-button-repost:hover {{
    background: rgba(255, 255, 255, 0.1) !important;
}}

.soundActions, .sound__soundActions {{
    display: flex !important;
    gap: 16px !important;
    align-items: center !important;
}}

.soundActions .sc-button, .sound__soundActions .sc-button {{
    margin: 0 !important;
    padding: 6px 10px !important;
    min-width: auto !important;
}}

.sc-button-group {{
    display: flex !important;
    gap: 6px !important;
}}

.soundList__item, .lazyLoadingList__list > li, .userStream__list > li {{
    background: rgba(0, 0, 0, 0.3) !important;
    backdrop-filter: blur(10px) !important;
    border: 1px solid var(--glass-border) !important;
    border-radius: 10px !important;
    margin: 4px 0 !important;
}}

.soundList__item:hover, .lazyLoadingList__list > li:hover {{
    background: rgba(255, 85, 0, 0.15) !important;
    border-color: rgba(255, 85, 0, 0.4) !important;
}}

.sc-button {{
    background: rgba(0, 0, 0, 0.4) !important;
    backdrop-filter: blur(8px) !important;
    border: 1px solid var(--glass-border) !important;
    border-radius: 6px !important;
}}

.sc-button:hover {{
    background: rgba(255, 85, 0, 0.25) !important;
    border-color: rgba(255, 85, 0, 0.5) !important;
}}

.sc-button-follow {{
    background: rgba(255, 85, 0, 0.2) !important;
    border: 1px solid rgba(255, 85, 0, 0.5) !important;
}}

.volume, .volume__button {{
    background: transparent !important;
    border: none !important;
}}

.tabs__tab.selected, .g-tabs-link.active {{
    border-bottom: 2px solid var(--accent) !important;
    color: #ff6b2b !important;
}}

.headerSearch__input {{
    background: rgba(0, 0, 0, 0.4) !important;
    border: 1px solid var(--glass-border) !important;
    border-radius: 6px !important;
}}

.headerSearch__input:focus {{
    border-color: rgba(255, 85, 0, 0.6) !important;
}}

::-webkit-scrollbar {{
    width: 8px !important;
}}

::-webkit-scrollbar-track {{
    background: rgba(0,0,0,0.2) !important;
}}

::-webkit-scrollbar-thumb {{
    background: rgba(255, 85, 0, 0.6) !important;
    border-radius: 4px !important;
}}

.modal, .dropdownContent, .dialog {{
    background: var(--glass) !important;
    backdrop-filter: blur(30px) !important;
    border: 1px solid var(--glass-border) !important;
    border-radius: 12px !important;
}}

.banner, .announcement, .upsellBanner, .promoBanner {{
    display: none !important;
}}

.iconButton, .playControls__control {{
    border: none !important;
    background: transparent !important;
}}

.iconButton:hover, .playControls__control:hover {{
    background: rgba(255, 255, 255, 0.1) !important;
}}

.waveform__layer {{
    opacity: 0.7 !important;
}}
"#, bg = BG_GIF_BASE64)
}

const INJECT_SCRIPT: &str = r#"
(function() {
    const css = CSS_PLACEHOLDER;
    const style = document.createElement('style');
    style.id = 'sc-glass';
    style.textContent = css;
    
    function inject() {
        if (!document.getElementById('sc-glass') && document.head) {
            document.head.appendChild(style.cloneNode(true));
        }
    }
    
    inject();
    document.addEventListener('DOMContentLoaded', inject);
    new MutationObserver(inject).observe(document.documentElement, { childList: true, subtree: true });
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let css = get_custom_css();
    let css_escaped = css.replace('\\', "\\\\").replace('`', "\\`").replace("${", "\\${");
    let script = INJECT_SCRIPT.replace("CSS_PLACEHOLDER", &format!("`{}`", css_escaped));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let _window = WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://soundcloud.com".parse().unwrap())
            )
            .title("SoundCloud")
            .inner_size(1280.0, 800.0)
            .min_inner_size(900.0, 600.0)
            .center()
            .initialization_script(&script)
            .build()?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
