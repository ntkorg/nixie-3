if ($Script:args -eq "release") {
  $outputbin = ".\target\aarch64-nintendo-switch-freestanding\release\nixie-test.nsp"
  
  Remove-Item $outputbin
  
  & cargo nx build --package nixie-test --release | Tee-Object -Variable output
} else {
  $outputbin = ".\target\aarch64-nintendo-switch-freestanding\debug\nixie-test.nsp"
  
  Remove-Item $outputbin
  
  & cargo nx build --package nixie-test | Tee-Object -Variable output
}
  
if ([System.IO.File]::Exists($outputbin)) {
  dotnet run --project C:\Users\Rose\Documents\ntkorg\ryujinx\src\Ryujinx\Ryujinx.csproj $outputbin
}