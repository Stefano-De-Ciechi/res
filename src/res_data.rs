use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;
use std::path::PathBuf;
use std::sync::Arc;
use dashmap::{DashMap, ReadOnlyView};
use rayon::prelude::*;

#[derive(Hash, Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: String,
}

impl FileEntry {
    fn new(path: String, name: String, ext: String, size: String) -> FileEntry {
        FileEntry { path, name, extension: ext, size }
    }
}

pub fn generate_entries_map(path: PathBuf, max_depth: usize) -> ReadOnlyView<String, Vec<Arc<FileEntry>>> {
    let map: DashMap<String, Vec<Arc<FileEntry>>> = DashMap::new();

    let entries: Vec<_> = WalkDir::new(path)
        .max_depth(max_depth)  
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    // uses rayon par_item to execute in parallel
    entries.par_iter().for_each(|entry| {
        let md = entry.metadata().unwrap();

        if !md.is_dir() {

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
                None => String::new(),
            };

            let size = entry.metadata()
                .unwrap()
                .len();

            let size = human_readable_size(size);

            if map.contains_key(&name.to_string()) {
                let mut vec = map.get_mut(&name.to_string()).unwrap();
                let entry = FileEntry::new(path, name.to_string(), ext, size);

                vec.push(Arc::new(entry));
            } else {
                let mut vec = Vec::new();
                let entry = FileEntry::new(path, name.to_string(), ext, size);
                vec.push(Arc::new(entry));

                map.insert(name.to_string(), vec);
            }
        }

    });

    map.into_read_only()
}

pub struct ResApp {
    pub path: PathBuf,
    pub entries_map: ReadOnlyView<String, Vec<Arc<FileEntry>>>,
    pub search_string: String,
    pub keys: Vec<String>,
    pub filtered_keys: Vec<String>,
    pub max_depth: usize,
}

impl ResApp {
    pub fn new(path: PathBuf, max_depth: usize) -> Self {
        let entries = generate_entries_map(path.clone(), max_depth);
        let mut keys: Vec<String> = entries.keys()
            .map(ToString::to_string)
            .collect();

        keys.sort();

        let filtered_keys: Vec<String> = Vec::new();

        Self {
            path,
            entries_map: entries,
            search_string: String::new(),
            keys,
            filtered_keys,
            max_depth,
        }
    }

    // regenerate entries, keys and filtered keys when either the path or the max_depth are modified
    pub fn update(&mut self, path: PathBuf, max_depth: usize) {
        self.entries_map = generate_entries_map(path, max_depth);
        self.keys = self.entries_map.keys()
            .map(ToString::to_string)
            .collect();

        self.keys.sort();

        // if the search string isn't empty use it to filter the newly-generated entries
        if self.search_string.is_empty() {
            self.filtered_keys = Vec::new();
        } else {
            self.filter_keys_by_name(&self.search_string.clone());
        }

    }

    // TODO eventually expand to fuzzy filter by extension or relative path
    // actually, it is not possible to do it efficiently enough right now; an alternative would be to have
    // a separate hashmap where the keys are the file extensions (it would be necessary to populate
    // that map too)
    pub fn filter_keys_by_name(&mut self, pattern: &str) {
        let search_re = RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build().unwrap_or_else(|_| Regex::new("").unwrap());

        self.filtered_keys = self.keys.iter()
            .filter(|e| search_re.is_match(e))
            .map(ToString::to_string)
            .collect();
    }

}

fn human_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "kB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1000.0 && unit < UNITS.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", size as u64, UNITS[unit])
    } else {
        format!("{:.2} {}", size, UNITS[unit])
    }
}
