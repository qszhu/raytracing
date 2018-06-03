### run
```bash
$ cargo build --release
$ target/release/rti1w out.png
```

### running time
width | height | samples/pixel | bounces | time | workers
--- | --- | --- | --- | --- | ---
200 | 100 | 100 | 10 | 10s | 1
200 | 100 | 100 | 10 | 2s | 8
1200 | 800 | 10 | 10 | 2m20s | 8
