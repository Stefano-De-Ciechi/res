#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::time::Instant;
    use crate::res_data::{generate_entries_map, FileEntry};

    #[test]
    fn test_generate_entries() {
        let now = Instant::now();

        let entries = generate_entries_map(PathBuf::from("C:\\"), 100);

        println!("Generate entries map: {:?}", now.elapsed());
        println!("Number of entries: {:?}", entries.len());

        let now = Instant::now();

        let vec = entries.keys().into_iter().flat_map(|key| entries[key].iter()).collect::<Vec<&FileEntry>>();

        println!("Entries into Vec: {:?}", now.elapsed());
        println!("Number of entries: {:?}", vec.len());
    }
}
