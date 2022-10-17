This is a downloader for Arweave transactions. It contains two implementations.

## Sequential Reads and Writes

```
cargo run --bin simple -- --transaction <tx_id> --output <file_name>
```

The first implementation sequentially reads chunks from Arweave then writes them to the output file. Only one request at a time is made to Arweave.

## Parallel Reads, Sequential Writes

```
cargo run --bin downloader -- --transaction <tx_id> --output <file_name> [--connections <num>]
```

The second implementation features an optional `--connections` argment which defaults to 10. The program will request up to that many chunks at once from Arweave. Chunks are then ordered and sequentially written to the output file.
