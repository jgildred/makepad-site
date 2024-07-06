cd frontend
cargo makepad wasm build -p makepad-site-frontend --release
rm -rf ../webroot/makepad_platform
rm -rf ../webroot/makepad_wasm_bridge
rm -rf ../webroot/makepad_widgets
mv -f target/makepad-wasm-app/release/makepad-site-frontend/makepad_platform ../webroot/
mv -f target/makepad-wasm-app/release/makepad-site-frontend/makepad_wasm_bridge ../webroot/
mv -f target/makepad-wasm-app/release/makepad-site-frontend/makepad_widgets ../webroot/
mv -f target/makepad-wasm-app/release/makepad-site-frontend/index.html ../webroot/
mv -f target/makepad-wasm-app/release/makepad-site-frontend/*.wasm ../webroot/
cd ..
cargo build --target wasm32-wasi --release