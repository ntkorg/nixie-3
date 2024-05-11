if (cargo nx build); then
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d > disassembly.txt
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d --demangle > disassembly.demangled.txt
  # ~/Downloads/ryujinx-1.1.1217-linux_x64/publish/Ryujinx ./target/aarch64-nintendo-switch-freestanding/debug/nixie-core.nsp
  curl -T ./target/aarch64-nintendo-switch-freestanding/debug/nixie-core.nsp ftp://rose:Rosefu11y!@192.168.1.136:5000/atmosphere/contents/0100F2C0115B6000/exefs.nsp
fi
