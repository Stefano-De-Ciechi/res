// TODO try to use the crossterm crate to implement a TUI
use eframe::egui::{self};
use egui_extras::{Column, TableBuilder};
use std::{collections::HashMap, path::PathBuf};
use crate::res_data::{filter_entries_keys, generate_entries_map, FileEntry};

struct MyApp {
    path: PathBuf,
    entries_map: HashMap<String, Vec<FileEntry>>,
    search_string: String,
    keys: Vec<String>,
    filtered_keys: Vec<String>,
}

impl MyApp {
    fn new(path: PathBuf) -> Self {

        let entries = generate_entries_map(path.clone());
        let mut keys: Vec<String> = entries.keys()
            .map(|e| e.to_string())
            .collect();

        keys.sort();

        let filtered_keys: Vec<String> = Vec::new();

        Self {
            path,
            entries_map: entries, 
            search_string: "".to_string(),
            keys,
            filtered_keys,
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            path: dirs::home_dir().unwrap(), 
            entries_map: HashMap::new(),
            search_string: "".to_string(),
            keys: Vec::new(),
            filtered_keys: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{}", self.path.to_str().unwrap()));

            let _search = ui.add(egui::TextEdit::singleline(&mut self.search_string));
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);

            if _search.changed() {
                self.filtered_keys = filter_entries_keys(&self.keys, &self.search_string);
            }
            
            let keys = match &self.search_string.is_empty() {
                true => &self.keys,
                false => &self.filtered_keys,
            };  

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto()).resizable(true)
                .column(Column::auto())
                .column(Column::auto())
                .header(row_height, |mut header| {
                    header.col(|ui| {
                        ui.strong("File Name");
                    });
                    header.col(|ui| {
                        ui.strong("Extension");
                    });
                    header.col(|ui| {
                        ui.strong("Path");
                    });
                })
                .body(|mut body| {
                    
                    //keys.sort();
                    for key in keys {
                        let entries = &self.entries_map[key];

                        for entry in entries {
                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label(&entry.name);
                                });
                                row.col(|ui| {
                                    ui.label(&entry.extension);
                                });
                                row.col(|ui| {
                                    ui.label(&entry.path);
                                });
                            });

                        }
                    }
                    
                });
        });
    }
}

pub fn res_ui_init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native("Rust Everywhere Search",
        options,
        Box::new(|_cc| {
        Ok(Box::new(
            MyApp::new(dirs::home_dir().unwrap())
        ))
    }))  
}
