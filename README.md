# DSS Shiplet Homework
Requires `rustc` version 1.47 or higher. Currently compiles on both macOS and Windows.

## To Build
```bash
$ cargo run
```

or 

```bash
$ cargo build --release
$ ./target/release/dss_shiplet // dss_shiplet.exe on Windows
```

or, to run the provided binaries

```bash
$ ./dss_shiplet
```
```powershell
PS C:\..\dss_shiplet> ./dss_shiplet.exe
```

On Windows, there may be temporary flash of an empty white screen while the grid's images download.

## Navigating the UI
The app currently supports Arrow Up, Arrow Down, Arrow Right, and Arrow Left navigation.

## Commit History
The default branch `main` is a squashed presentation of the work history, segmented by the top-level feature implementations.

Checkout `progress-history` to see the commits as they occurred.
