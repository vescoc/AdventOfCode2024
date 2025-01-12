set print asm-demangle on
set print pretty on
set backtrace limit 32

file target/thumbv7em-none-eabihf/release/arduino-nano33blesense

target extended-remote openocd:3333

load

monitor reset halt

monitor rtt server start 8765 0
monitor rtt setup 0x20000000 30 "SEGGER RTT"
monitor rtt start
