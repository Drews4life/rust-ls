To run this project all you need is either UNIX system or Docker.

If you are trying to run program on the unix system:
1. install rustup https://www.rust-lang.org/learn/get-started
2. run `cargo run {path}`

if you are trying to run program using Docker:
1. Build the image `docker build -t {image-name} {context}`
2. Run the image `docker run ${image-name}`
You can specify execution path in `CMD` comman in Docker file. By default the path will be `.`