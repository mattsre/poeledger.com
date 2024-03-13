use serde::Deserialize;

use poe_types::stash::PublicStashChange;

#[derive(Deserialize)]
pub struct PublicStashesResponse {
    pub next_change_id: String,
    pub stashes: Vec<PublicStashChange>,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use super::PublicStashesResponse;

    #[test]
    fn deserialize_stash_response() {
        let path = Path::new("test/stash-1.json");
        let display = path.display();

        let file1 = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(f) => f,
        };

        let _: PublicStashesResponse = serde_json::from_reader(file1).unwrap();

        let path = Path::new("test/stash-2.json");
        let display = path.display();

        let file2 = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(f) => f,
        };

        let _: PublicStashesResponse = serde_json::from_reader(file2).unwrap();

        let path = Path::new("test/stash-3.json");
        let display = path.display();

        let file3 = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(f) => f,
        };

        let _: PublicStashesResponse = serde_json::from_reader(file3).unwrap();
    }
}
