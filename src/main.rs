use std::path::Path;
use std::time::SystemTime;
use tokio::main;
use crate::file_helper::get_path_files;

mod file_helper;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = SystemTime::now();

    let files = get_path_files(Path::new("/home/belochka/")).await?;

    println!("{:?} count {}", time.elapsed()?, files.iter().count());
    Ok(())
}
