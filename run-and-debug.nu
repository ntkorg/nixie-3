cargo nx build --package nixie-test
if $env.LAST_EXIT_CODE == 0 {
  # llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d | save -f disassembly.txt
  # llvm-objdump target/aarch64-nintendo-switch-freestanding/debug/nixie-core -d --demangle | save -f disassembly.demangled.txt
  dotnet run --project ..\Ryujinx\src\Ryujinx\Ryujinx.csproj -- target/aarch64-nintendo-switch-freestanding/debug/nixie-test.nsp
}
