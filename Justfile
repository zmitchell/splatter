_default:
    @just --list --unsorted

# Build splatter
build:
    @cargo build

# Build splatter and all of the examples
build-all:
    @cargo build --all-targets

# Run the test suite
test regex="":
    @cargo test {{regex}}

