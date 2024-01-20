#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32g0;

#[entry]
fn main() -> ! {
    loop {}
}
