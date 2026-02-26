use egui_commonmark::{
    CommonMarkCache, CommonMarkViewer, CustomFormatCallback, CustomFormatGroup, CustomFormatMatcher,
};

struct App {
    cache: CommonMarkCache,
    format_matcher: CustomFormatMatcher,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let callbacks: [&CustomFormatCallback; 2] = [
                &|ui, match_str| {
                    if ui.button(format!("📋 {match_str}")).clicked() {
                        ui.ctx().copy_text(match_str.to_string());
                    }
                },
                &|ui, match_str| {
                    if ui.link(match_str).clicked() {
                        println!("Clicked issue: {match_str}");
                    }
                },
            ];

            let group = CustomFormatGroup {
                matcher: &self.format_matcher,
                callbacks: &callbacks,
            };

            CommonMarkViewer::new()
                .custom_formats(group)
                .show(ui, &mut self.cache, EXAMPLE_TEXT);
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Regex Formatting Viewer",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| {
            Ok(Box::new(App {
                cache: CommonMarkCache::default(),

                format_matcher: CustomFormatMatcher::new(&[
                    r"\b\d{1,3}(?:\.\d{1,3}){3}\b", // Index 0: IP address
                    r"#\d+",                        // Index 1: Issue number
                ])
                .unwrap(),
            }))
        }),
    )
}

const EXAMPLE_TEXT: &str = r"
# Server Logs

Database at 192.168.1.15 is dropping packets. This seems related to #4012. 
Please ensure you do not block 172.16.254.1 as per issue #3990.
";
