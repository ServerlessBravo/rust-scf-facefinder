#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#rustup target add x86_64-unknown-linux-musl
#sudo apt update
#sudo apt install build-essential
#sudo apt install pkgconf
#sudo apt install openssl libssl-dev -y

# install wasm-pack
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

rm -rf pkg
rm -rf pkg.zip

wasm-pack build --target nodejs --release
zip -r pkg.zip index.js pkg  -x "*.git*"
