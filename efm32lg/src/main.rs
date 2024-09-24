#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use efm32lg230_pac as pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Take ownership of device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    // Enable the clock for GPIO and USART0 (not UART0)
    let cmu = &dp.CMU;
    cmu.hfperclken0.modify(|_, w| w.usart0().set_bit());  // Enable clock for USART0
    cmu.hfperclken0.modify(|_, w| w.gpio().set_bit());    // Enable clock for GPIO

    // Configure GPIO pins PA0 (TX) and PA1 (RX) for USART0
    let gpio = &dp.GPIO;
    gpio.pa_model.modify(|_, w| w.mode0().pushpull());  // PA0 as TX
    gpio.pa_model.modify(|_, w| w.mode1().input());     // PA1 as RX

    // Set up USART0 for UART-like communication at 115200 baud
    let usart0 = &dp.USART0;
    usart0.clkdiv.write(|w| unsafe { w.div().bits(0xD) });  // Set clock divisor for ~115200 baud
    usart0.route.write(|w| {
        w.txpen().set_bit();  // Enable TX pin
        w.rxpen().clear_bit(); // Disable RX pin (since we're only sending)
        unsafe { w.location().bits(0) }  // Use default location for TX/RX in an unsafe block
    });

    // Enable the USART transmitter
    usart0.cmd.write(|w| w.txen().set_bit());

    // Send "Hello, World!" byte by byte
    let hello = b"Hello, World!\r\n";
    for &byte in hello {
        while usart0.status.read().txbl().bit_is_clear() {}  // Wait until the TX buffer is empty
        usart0.txdata.write(|w| unsafe { w.txdata().bits(byte) }); // Send byte in an unsafe block
    }

    // Loop forever to prevent the program from exiting
    loop {}
}
