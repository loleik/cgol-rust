# cgol-rust
Terminal based demo version of Conway's Game of Life, written in rust. This will be used as a base for a WASM implementation, which is intended to be put on my website.

## Usage
Run using `cargo run -- pattern <PATTERN> <HEIGHT> <WIDTH>` or build and run `./cgol-rust run <PATTERN> <HEIGHT> <WIDTH>`. Or alternatively use Docker:
+ `docker build -t cgol-rust .`
+ `docker run cgol-rust <PATTERN> <HEIGHT> <WIDTH>`

Arguments are as follows:
+ `PATTERN` a list of co-ordinates in the form `[(x1,y1),(x2,y2),...(xn,yn)]`.
+ `HEIGHT`, `WIDTH` the integer height and width of the viewport.
