// TODO try to use the crossterm crate to implement a TUI
use crate::res_data::{filter_entries_keys, generate_entries_map, FileEntry};
use eframe::egui::{self};
use egui_extras::{Column, TableBuilder};
use std::{collections::HashMap, path::PathBuf};

struct MyApp {
    path: PathBuf,
    entries_map: HashMap<String, Vec<FileEntry>>,
    search_string: String,
    keys: Vec<String>,
    filtered_keys: Vec<String>,
    max_depth: usize,
}

impl MyApp {
    fn new(path: PathBuf, max_depth: usize) -> Self {
        let entries = generate_entries_map(path.clone(), max_depth);
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
            max_depth,
        }
    }

    // regenerate entries, keys and filtered keys when either the path or the max_depth are modified
    fn update(&mut self, path: PathBuf, max_depth: usize) {
        self.entries_map = generate_entries_map(path, max_depth);
        self.keys = self.entries_map.keys()
            .map(|e| e.to_string())
            .collect();

        self.keys.sort();

        // if the search string isn't empty use it to filter the newly-generated entries
        match self.search_string.is_empty() {
            true => self.filtered_keys = Vec::new(),
            false => self.filtered_keys = filter_entries_keys(&self.keys, &self.search_string),
        }

    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);

            ui.heading(format!("{}", self.path.to_str().unwrap()));

            ui.horizontal(|ui| {
                ui.label("max_depth: ");

                let _slider = ui.add(egui::Slider::new(&mut self.max_depth, 1..=10));

                if _slider.changed() {
                    self.update(self.path.clone(), self.max_depth);
                }

            });

            ui.horizontal(|ui| {
                ui.label("search (with regex too): ");

                let _search = ui.add(egui::TextEdit::singleline(&mut self.search_string));

                if _search.changed() {
                    self.filtered_keys = filter_entries_keys(&self.keys, &self.search_string);
                }

            });

            /*let _search = ui.add(egui::TextEdit::singleline(&mut self.search_string));

            if _search.changed() {
                self.filtered_keys = filter_entries_keys(&self.keys, &self.search_string);
            }*/

            let entries: Vec<&FileEntry> = match self.search_string.is_empty() {
                true => self.keys.iter().flat_map(|key| self.entries_map[key].iter()).collect(),
                false => self.filtered_keys.iter().flat_map(|key| self.entries_map[key].iter()).collect(),
            };

            ui.label(format!("entries: {}", entries.len()));

            egui::ScrollArea::horizontal().show(ui, |ui| {

                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
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
                    .body(|body| {
                        body.rows(row_height, entries.len(), |mut row| {
                            let index = row.index();
                            let entry = entries[index];

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
                    });
            });
        });

    }
}

pub fn res_ui_init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Everywhere Search",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(dirs::home_dir().unwrap(), 3)))),
    )
}
