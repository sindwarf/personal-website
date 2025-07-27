cd ../client
npm run build
cp -r dist/spa ~/release/client/spa
cd ../server
cargo clean
cargo build -r
cp target/release/server ~/release
