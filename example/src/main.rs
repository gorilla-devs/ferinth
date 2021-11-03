use ferinth::Ferinth;
use std::{fs::File, io::Write};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("API error occured")]
    FerinthError(#[from] ferinth::Error),
    #[error("IO error occured")]
    IOError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = Ferinth::new("example"); // Initialise API

    // Using Project ID (never changes)
    let mod_ = api.get_mod("AANobbMI").await?;
    // OR you can use mod slugs (these can change)
    // let mod_ = api.get_mod("sodium").await?;

    // Get the latest version's ID
    let latest_version = &mod_.versions[0];
    // Get that version
    let latest_version = api.get_version(&latest_version).await?;
    // Get that version's first file
    let version_file = &latest_version.files[0];

    // Download version_file
    let contents = api.download_version_file(version_file).await?;
    // Open the file to write to
    let mut mod_file = File::open("Sodium.jar")?;
    // Write the contents to mod_file
    mod_file.write_all(&contents)?;

    Ok(())
}
