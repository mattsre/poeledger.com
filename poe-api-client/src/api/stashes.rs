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
        let path = Path::new("test/stash.json");
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(file) => file,
        };

        let _: PublicStashesResponse = serde_json::from_reader(file).unwrap();

        let path = Path::new("test/stash-3.json");
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(file) => file,
        };

        let _: PublicStashesResponse = serde_json::from_reader(file).unwrap();
    }
}
