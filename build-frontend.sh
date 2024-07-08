cd makepad-site-frontend
cargo makepad wasm build -p makepad-site-frontend --release
cd ..
rm -rf webroot
mv -f makepad-site-frontend/target/makepad-wasm-app/release/makepad-site-frontend webroot