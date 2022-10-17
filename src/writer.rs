use anyhow::Result;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn create_file(filename: &str) -> Result<File> {
    Ok(File::create(filename).await?)
}

pub async fn write_chunk(file: &mut File, chunk: &Vec<u8>) -> Result<()> {
    Ok(file.write_all(chunk).await?)
}
