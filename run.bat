cd C:\Users\david\Source\Repos\serialitm
start "serialitm" cmd.exe /k "cargo run com3"

cd C:\Users\david\Source\Repos\lm75a-temperature-sensor
start "openocd" cmd.exe /k "openocd"
start "demo hardware" cmd.exe /k "cargo run"
