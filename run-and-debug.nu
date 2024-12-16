cargo nx build
if $env.LAST_EXIT_CODE == 0 {
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d | save -f disassembly.txt
  llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d --demangle | save -f disassembly.demangled.txt
  ~/Downloads/ryujinx-r.49574a9-x64.AppImage target/aarch64-nintendo-switch-freestanding/debug/nixie-core.nsp
}
