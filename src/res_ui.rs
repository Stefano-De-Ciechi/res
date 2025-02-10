// TODO try to use the crossterm crate to implement a TUI
use crate::res_data::{FileEntry, ResApp};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

struct MyApp {
    res: ResApp,
    file_dialog: FileDialog,
    update_entries: bool,
}

impl MyApp {
    fn new(res: ResApp) -> Self {
        Self {
            res,
            update_entries: false,
            file_dialog: FileDialog::new()
                .initial_directory(dirs::home_dir().unwrap())
                .show_new_folder_button(false),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);

            ui.horizontal(|ui| {
                if ui.button("open").clicked() {
                    self.file_dialog.select_directory();
                    self.update_entries = true;
                }

                if self.update_entries {
                    if let Some(path) = self.file_dialog.update(ctx).selected() {
                        debug_println!("filtering on selected path: {:?}", path);

                        let mut path = path.to_str().unwrap_or("");

                        if path.starts_with("\\\\?\\") {
                            let tmp_path = path.strip_prefix("\\\\?\\");
                            path = match tmp_path {
                                Some(p) => p,
                                None => path,
                            };
                        }

                        debug_println!("cleaned up path: {}", path);

                        self.res.path = path.into();
                        self.res.update(self.res.path.clone(), self.res.max_depth);
                        self.update_entries = false;
                    }
                }
                
                ui.heading(format!("{}", self.res.path.to_str().unwrap()));
            });
            
            ui.horizontal(|ui| {
                ui.label("max_depth: ");

                let _slider = ui.add(egui::Slider::new(&mut self.res.max_depth, 1..=10));

                if _slider.changed() {
                    self.res.update(self.res.path.clone(), self.res.max_depth);
                    debug_println!("filtering on max_depth change to {}", self.res.max_depth);
                }

            });

            ui.horizontal(|ui| {
                ui.label("filter names (with regex too): ");

                let _search = ui.add(egui::TextEdit::singleline(&mut self.res.search_string));

                if _search.changed() {
                    //self.res.filtered_keys = filter_entries_keys(&self.res.keys, &self.res.search_string);
                    self.res.filter_by_name(&self.res.search_string.clone());
                    debug_println!("filtering on regex search: {}", self.res.search_string);
                }

            });

            // TODO this gets executed every frame (kinda wasteful of resources if entries do not change between two frames)
            let entries: Vec<&FileEntry> = match self.res.search_string.is_empty() {
                true => {
                    //debug_println!("inefficient code here");
                    self.res.keys.iter().flat_map(|key| self.res.entries_map[key].iter()).collect()
                },
                false => self.res.filtered_keys.iter().flat_map(|key| self.res.entries_map[key].iter()).collect(),
            };

            ui.label(format!("entries n.: {}", entries.len()));

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
                                let l = ui.label(&entry.name);
                                if l.clicked() {
                                    open_in_explorer(&entry);
                                }
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

// using the open crate
fn open_in_explorer(entry: &FileEntry) {
    let full_path = match entry.extension.as_str() {
         "" => format!("{}/{}", entry.path, entry.name),    // if the file has no extension
        _ => format!("{}/{}.{}", entry.path, entry.name, entry.extension),
    };

    println!("opening: {}", full_path);

    // uses the default OS specific opener associated with the selected file based on his format
    if let Err(err) = open::that(&full_path) {
        eprintln!("an error occured trying to open '{}' : {}", full_path, err);
    }
}

pub fn res_ui_init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let res = ResApp::new(dirs::home_dir().unwrap(), 3);
    let app = MyApp::new(res);

    eframe::run_native(
        "Rust Everywhere Search",
        options,
        Box::new(|_cc| Ok(Box::new(app)))
    )
    
}
