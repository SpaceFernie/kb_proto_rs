#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32g0::stm32g0b1;

#[entry]
fn main() -> ! {
    let peripherals = stm32g0b1::Peripherals::take().unwrap();

    // Enable SWD
    peripherals.RCC.apbenr1.write(|w| w.dbgen().set_bit());

    // Enable GPIOA
    peripherals.RCC.iopenr.write(|w| w.gpioaen().set_bit());

    // LED LD4 is on pin PA5 for the Nucleo-G0B1RE
    // Button B1 is on pin PC13 for the Nucleo-G0B1RE
    peripherals.GPIOA.moder.write(|w| w.moder4().bits(0b01)); // Set PA5 to output
    peripherals.GPIOA.otyper.write(|w| w.ot4().set_bit()); // Set PA5 to P-P output
    peripherals.GPIOA.ospeedr.write(|w| w.ospeedr4().bits(0b00)); // Set PA5 drive strength to minimum

    loop {
        // Delay approx 1s for blinky
        let counter_threshold: u32 = 4_000_000;

        // Blink
        let mut counter: u32 = 0;
        while counter_threshold < counter {
            counter += 1;
        }

        peripherals.GPIOA.odr.write(|w| w.odr4().set_bit());

        counter = 0;
        while counter_threshold < counter {
            counter += 1;
        }

        peripherals.GPIOA.odr.write(|w| w.odr4().clear_bit());
    }
}
