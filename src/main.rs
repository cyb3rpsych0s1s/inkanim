use std::path::PathBuf;

use inkanim::InkAnimAnimationLibraryResource;

mod inkanim;

fn main() {
    let content = std::fs::read_to_string(PathBuf::from(
        "./rootconnecttosandrarescue.json".to_string(),
    ))
    .expect("file");

    let resource = serde_json::from_str::<InkAnimAnimationLibraryResource>(&content).unwrap();
    println!("{resource:#?}");
}
