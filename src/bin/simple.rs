use anyhow::Result;

use arweave_downloader::{args, reader, writer};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::parse();

    let client = reader::get_client().await?;
    let offsets = reader::get_tx_offsets(&client, &args.transaction).await?;

    let mut file = writer::create_file(&args.output).await?;

    for offset in offsets {
        let chunk = reader::read_chunk(client.clone(), offset).await?;
        writer::write_chunk(&mut file, &chunk).await?;
    }

    Ok(())
}
