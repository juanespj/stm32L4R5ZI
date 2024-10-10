#![no_std]
#![no_main]

use core::fmt::Write;
mod serial;
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
// use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};
bind_interrupts!(struct Irqs {
    // LPUART1 => usart::InterruptHandler<peripherals::LPUART1>;
    LPUART1 => usart::InterruptHandler<peripherals::LPUART1>;
});
static SHARED: AtomicU32 = AtomicU32::new(0);
static MOTORA: AtomicU32 = AtomicU32::new(0);
// static CHSTATE: Channel<ThreadModeRawMutex, sys::SysSTATE, 1> = Channel::new();
static CHANPRINT: Channel<ThreadModeRawMutex, String<10>, 1> = Channel::new();
// #[cortex_m_rt::entry]
// fn main() -> ! {
//     info!("Hello World!");

//     let p = embassy_stm32::init(Default::default());

//     let config = Config::default();
//     let mut usart = Uart::new_blocking(p.LPUART1, p.PG8, p.PG7, config).unwrap();

//     unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
//     info!("wrote Hello, starting echo");

//     let mut buf = [0u8; 1];
//     loop {
//         unwrap!(usart.blocking_read(&mut buf));
//         unwrap!(usart.blocking_write(&buf));
//     }
// }

// bind_interrupts!(struct Irqs {
//     UART4 => usart::InterruptHandler<peripherals::UART4>;
// });

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let config = Config::default();
    let usart = Uart::new(
        p.LPUART1, p.PG8, p.PG7, Irqs, p.DMA1_CH3, p.DMA1_CH4, config,
    )
    .unwrap();
    serial::uart_init(usart, spawner);
    loop {
        Timer::after_millis(100).await;
    }
}
