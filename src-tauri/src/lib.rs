mod theme;
mod tray;

use tauri::WebviewWindowBuilder;
use theme::get_custom_css;

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
            let window = WebviewWindowBuilder::new(
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
            
            // Apply vibrancy effects
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None);
            }
            
            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_mica;
                let _ = apply_mica(&window, Some(true));
            }
            
            // Setup system tray
            if let Err(e) = tray::setup_tray(app.handle()) {
                eprintln!("Failed to setup tray: {}", e);
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
