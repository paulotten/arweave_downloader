use anyhow::{bail, Result};
use futures::future::join_all;

use arweave_downloader::{args, reader, writer};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::parse();
    if args.connections < 1 {
        bail!("connections must be set to at least 1");
    }

    let client = reader::get_client().await?;
    let offsets = reader::get_tx_offsets(&client, &args.transaction).await?;

    let mut file = writer::create_file(&args.output).await?;

    let mut i = 0;
    while i < offsets.len() {
        // read
        let mut readers = vec![];
        for _ in 0..args.connections {
            if i >= offsets.len() {
                break;
            }

            readers.push(reader::read_chunk(client.clone(), offsets[i]));

            i += 1;
        }

        // write
        for chunk in join_all(readers).await {
            writer::write_chunk(&mut file, &chunk?).await?;
        }
    }

    Ok(())
}
