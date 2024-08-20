use walkdir::WalkDir;
use std::{collections::HashMap, path::PathBuf};

// TODO try to use the crossterm crate to implement a TUI
use eframe::egui::{self};
use egui_extras::{Column, TableBuilder};

use regex::{Regex, RegexBuilder};

#[derive(Hash, Debug)]
struct FileEntry {
    path: String,
    name: String,
    extension: String,
}

impl FileEntry {
    fn new(path: String, name: String, ext: String) -> FileEntry {
        FileEntry { path, name, extension: ext }
    }
}

fn generate_entries_map(path: PathBuf) -> HashMap<String, Vec<FileEntry>> {
    let mut map: HashMap<String, Vec<FileEntry>> = HashMap::new();

    let walker = WalkDir::new(path)
        .max_depth(3)  
        .into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let md = entry.metadata().unwrap();

        if md.is_dir() {
            continue;
        }

        let name = entry.path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        let path = entry.path()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let ext = match entry.path()
            .extension() {
            Some(e) => e.to_str().unwrap().to_string(),
            None => "".to_string(),
        };

        match map.contains_key(&name.to_string()) {
            true => {
                let vec = map.get_mut(&name.to_string()).unwrap(); 
                let entry = FileEntry::new(path, name.to_string(), ext);

                vec.push(entry);
            },
            false => {
                let mut vec = Vec::new();
                let entry = FileEntry::new(path, name.to_string(), ext);
                vec.push(entry);

                map.insert(name.to_string(), vec);
            }
        }

    }

    map

}

struct MyApp {
    path: PathBuf,
    entries_map: HashMap<String, Vec<FileEntry>>,
    search_string: String,
}

impl MyApp {
    fn new(path: PathBuf) -> Self {

        Self {
            path: path.clone(),
            entries_map: generate_entries_map(path),
            search_string: "".to_string(),
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            path: dirs::home_dir().unwrap(), 
            entries_map: HashMap::new(),
            search_string: "".to_string(),
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

            //let search_re = Regex::new(&format!(r"{}.*", &self.search_string)).unwrap();
            let search_re = match RegexBuilder::new(&format!(r"{}.*", &self.search_string))
                .case_insensitive(true)
                .build() {
                Ok(re) => re,
                Err(_) => Regex::new("").unwrap(),
            };

            let mut keys: Vec<&String> = match self.search_string.is_empty() {
                true => {self.entries_map.keys().collect()},
                false => {
                    self.entries_map.keys()
                        .filter(|key| {search_re.is_match(key)})
                        .collect()
                },
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
                    
                    keys.sort();
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

fn main() -> eframe::Result {

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
