use eframe::egui;
use egui_commonmark::*;
use regex::{Regex, RegexSet};

struct App {
    cache: CommonMarkCache,
    format_set: RegexSet,
    ip_regex: Regex,
    issue_regex: Regex,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Construct the rules. Order MATTERS here.
                // Index 0 must be the IP regex, Index 1 must be the Issue regex.
                let formats = [
                    CustomFormat {
                        regex: &self.ip_regex,
                        callback: &|ui, match_str| {
                            if ui.button(format!("📋 {match_str}")).clicked() {
                                ui.ctx().copy_text(match_str.to_string());
                            }
                        },
                    },
                    CustomFormat {
                        regex: &self.issue_regex,
                        callback: &|ui, match_str| {
                            if ui.link(match_str).clicked() {
                                println!("Navigating to {match_str}...");
                            }
                        },
                    },
                ];

                let group = CustomFormatGroup {
                    set: &self.format_set,
                    rules: &formats,
                };

                CommonMarkViewer::new().custom_formats(group).show(
                    ui,
                    &mut self.cache,
                    EXAMPLE_TEXT,
                );
            });
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "RegexSet Formatting Viewer",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| {
            // 1. Define patterns
            let patterns = [
                r"\b\d{1,3}(?:\.\d{1,3}){3}\b", // Index 0: IP address
                r"#\d+",                        // Index 1: Issue number
            ];

            Ok(Box::new(App {
                cache: CommonMarkCache::default(),
                // 2. Compile Set
                format_set: RegexSet::new(patterns).unwrap(),
                // 3. Compile individual Regexes mapping to the exact same indices
                ip_regex: Regex::new(patterns[0]).unwrap(),
                issue_regex: Regex::new(patterns[1]).unwrap(),
            }))
        }),
    )
}

const EXAMPLE_TEXT: &str = r"
# Server Logs

Database at 192.168.1.15 is dropping packets. This seems related to #4012. 
Please ensure you do not block 172.16.254.1 as per issue #3990.
";
