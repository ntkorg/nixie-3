if cargo nx build --package nixie-test; then
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-test -d > disassembly.txt
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-test -d --demangle > disassembly.demangled.txt
  ~/Downloads/ryujinx-1.1.1217-linux_x64/publish/Ryujinx target/aarch64-nintendo-switch-freestanding/debug/nixie-test.nsp
fi
