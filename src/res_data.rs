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

pub fn generate_entries_map(path: PathBuf) -> HashMap<String, Vec<FileEntry>> {
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

pub fn filter_entries_keys(keys: &Vec<String>, pattern: &str) -> Vec<String> {
    let search_re = match RegexBuilder::new(&format!(r"{}", pattern))
        .case_insensitive(true)
        .build() {
            Ok(re) => re,
            Err(_) => Regex::new("").unwrap(),
    };

    keys.iter()
        .filter(|e| search_re.is_match(e))
        .map(|s| s.to_string())
        .collect()
}

