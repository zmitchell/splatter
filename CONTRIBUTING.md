*You can learn about contributing to splatter in [the
guide](https://guide.splatter.cc/contributing.html).*

## Updating package versions
We use `cargo-workspace-version` to:
- Find out if all of the `splatter` crates have the same version number
- Set the version number for all of the `splatter` crates
- Ensure that all of the `splatter` crates depend on the same/latest version of the other `splatter` crates

To check whether all of the version numbers match:
```
$ cargo workspace-version check <version>
```

To set a new version:
```
$ cargo workspace-version update <version>
```
