file target/thumbv6m-none-eabi/release/aoc2024-rp-pico

target extended-remote openocd:3333

load

monitor reset halt

monitor rtt server start 8765 0
monitor rtt setup 0x20000000 30 "SEGGER RTT"
monitor rtt start
