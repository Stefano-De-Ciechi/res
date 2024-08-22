use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;
use std::{collections::HashMap, path::PathBuf};

#[derive(Hash, Debug)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub extension: String,
}

impl FileEntry {
    fn new(path: String, name: String, ext: String) -> FileEntry {
        FileEntry { path, name, extension: ext }
    }
}

pub fn generate_entries_map(path: PathBuf, max_depth: usize) -> HashMap<String, Vec<FileEntry>> {
    let mut map: HashMap<String, Vec<FileEntry>> = HashMap::new();

    let walker = WalkDir::new(path)
        .max_depth(max_depth)  
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

pub struct ResApp {
    pub path: PathBuf,
    pub entries_map: HashMap<String, Vec<FileEntry>>,
    pub search_string: String,
    pub keys: Vec<String>,
    pub filtered_keys: Vec<String>,
    pub max_depth: usize,
}

impl ResApp {
    pub fn new(path: PathBuf, max_depth: usize) -> Self {
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
    pub fn update(&mut self, path: PathBuf, max_depth: usize) {
        self.entries_map = generate_entries_map(path, max_depth);
        self.keys = self.entries_map.keys()
            .map(|e| e.to_string())
            .collect();

        self.keys.sort();

        // if the search string isn't empty use it to filter the newly-generated entries
        match self.search_string.is_empty() {
            true => self.filtered_keys = Vec::new(),
            //true => self.filtered_keys = self.keys.clone(),
            false => self.filter_by_name(&self.search_string.clone()),
        }

    }

    // TODO eventually expand to fuzzy filter by extension or relative path
    // actually, it is not possible to do it efficiently enought right now; an alternative would be to have
    // a separate hashmap where the keys are the file extensions (it would be necessary to populate
    // that map too)
    pub fn filter_by_name(&mut self, pattern: &str) {
        let search_re = match RegexBuilder::new(&format!(r"{}", pattern))
            .case_insensitive(true)
            .build() {
                Ok(re) => re,
                Err(_) => Regex::new("").unwrap(),
        };

        self.filtered_keys = self.keys.iter()
            .filter(|e| search_re.is_match(e))
            .map(|s| s.to_string())
            .collect()
    }

}

