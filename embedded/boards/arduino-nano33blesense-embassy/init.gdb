set print asm-demangle on
set print pretty on
set backtrace limit 32

file target/thumbv7em-none-eabihf/release/arduino-nano33blesense-embassy

target extended-remote openocd:3333

load

monitor reset halt
