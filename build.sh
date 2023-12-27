cargo build
rm -rf ./target/aarch64-nintendo-switch-freestanding/debug/exefs
mkdir -p ./target/aarch64-nintendo-switch-freestanding/debug/exefs
elf2nso ./target/aarch64-nintendo-switch-freestanding/debug/nixie-core ./target/aarch64-nintendo-switch-freestanding/debug/exefs/main
cp main.npdm ./target/aarch64-nintendo-switch-freestanding/debug/exefs
build_pfs0 ./target/aarch64-nintendo-switch-freestanding/debug/exefs ./target/aarch64-nintendo-switch-freestanding/debug/exefs.nsp
curl -T ./target/aarch64-nintendo-switch-freestanding/debug/exefs.nsp ftp://anon:non@192.168.1.136:5000/atmosphere/contents/01002B30028F6000/exefs.nsp