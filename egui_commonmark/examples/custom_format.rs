//! Make sure to run this example from the repo directory and not the example
//! directory.
//!
//! Run this example with:
//! `cargo r --example custom_format --features better_syntax_highlighting`
//! Add `light` or `dark` to the end of the command to specify theme.

use eframe::egui;
use egui_commonmark::*;
use regex::Regex;

struct App {
    cache: CommonMarkCache,
    ip_regex: Regex,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Define the formatting rules for this frame
                let formats = [CustomFormat {
                    // Regex::clone() is cheap (it just bumps an Arc internally)
                    regex: self.ip_regex.clone(),
                    callback: &|ui, match_str| {
                        // Render a clickable button inline with the text
                        let btn = ui
                            .button(format!("📋 {match_str}"))
                            .on_hover_text("Click to copy IP address");

                        if btn.clicked() {
                            ui.ctx().copy_text(match_str.to_string());
                        }
                    },
                }];

                CommonMarkViewer::new().custom_formats(&formats).show(
                    ui,
                    &mut self.cache,
                    EXAMPLE_TEXT,
                );
            });
        });
    }
}

fn main() -> eframe::Result {
    let mut args = std::env::args();
    args.next();

    eframe::run_native(
        "Custom Regex Formatting Viewer",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            if let Some(theme) = args.next() {
                if theme == "light" {
                    cc.egui_ctx.set_theme(egui::Theme::Light);
                } else if theme == "dark" {
                    cc.egui_ctx.set_theme(egui::Theme::Dark);
                }
            }

            cc.egui_ctx.style_mut(|style| {
                style.url_in_tooltip = true;
            });

            Ok(Box::new(App {
                cache: CommonMarkCache::default(),
                // Compile the regex once on startup
                ip_regex: Regex::new(r"\b\d{1,3}(?:\.\d{1,3}){3}\b").unwrap(),
            }))
        }),
    )
}

const EXAMPLE_TEXT: &str = r#"
# Server Status Logs

The main database server at 192.168.1.15 is currently accepting connections. 
However, we are seeing dropped packets from the subnet router located at 10.0.0.1.

Please ensure that you do not block 172.16.254.1, as that is our internal DNS.

### Configuration

Ensure your local config does not bind to the loopback unless necessary:

```rust
// The custom formatting regex will NOT trigger inside this code block!
let localhost = "127.0.0.1";
println!("Binding to {}", localhost);
```

Check the logs for any unauthorized access attempts from 198.51.100.23!
"#;
