use std::path::PathBuf;

use inkanim::InkAnimAnimationLibraryResource;

mod inkanim;

fn main() {
    let content = std::fs::read_to_string(PathBuf::from(
        "./rootconnecttosandrarescue.json".to_string(),
    ))
    .expect("file");

    let resource = serde_json::from_str::<InkAnimAnimationLibraryResource>(&content).unwrap();
    // println!("{resource:#?}");
    let matches = resource.sequences[0]
        .data
        .get_path_indexes_matching(vec![1, 3, 0, 0, 9]);
    // 1 3 0 0 is Booting_Screen
    // 1 3 0 0 12 is rectangles_Beauty
    println!("{matches:#?}");
    println!(
        "found {} target(s) at this path or nested below",
        matches.len()
    );
}
