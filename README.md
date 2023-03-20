# vcocalc-rp2040-rs
Rust port of RP2040 VCO PLL settings calclator


## Run  

```bash
cargo run -- 125
# Requested: 125 MHz
# Achieved: 125 MHz
# REFDIV: 1
# FBDIV: 125 (VCO = 1500 MHz)
# PD1: 6
# PD2: 2
```

## See also

https://github.com/raspberrypi/pico-sdk/blob/master/src/rp2_common/hardware_clocks/scripts/vcocalc.py

Chaptor2-18 PLL  
https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf#%5B%7B%22num%22%3A232%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C552.89%2Cnull%5D
