arg tag=latest
from rust:${tag} as builder


workdir /app
copy ./Cargo.toml ./Cargo.lock ./
run cargo fetch

copy . .

run cargo build --release

run ls /app/target/release
cmd ["/app/target/release/xlsx-speedy"]
