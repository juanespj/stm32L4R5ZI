#![no_std]
#![no_main]

use core::fmt::Write;
mod motion;
mod serial;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{DataBits, Parity, Uart};
use embassy_stm32::{bind_interrupts, gpio, peripherals, usart};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
mod sys;
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::{spi, Config};
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::image::{ImageRaw, ImageRawLE};
use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use embedded_hal_bus::spi::ExclusiveDevice;
use serial::{print, println};
use tinybmp::Bmp;
use {defmt_rtt as _, panic_probe as _};

use st7735_embassy::{self, buffer_size, ST7735};

use heapless::String;
use {defmt_rtt as _, panic_probe as _};

//
bind_interrupts!(struct Irqs {
    // LPUART1 => usart::InterruptHandler<peripherals::LPUART1>;
    LPUART1 => usart::InterruptHandler<peripherals::LPUART1>;
});
static SHARED: AtomicU32 = AtomicU32::new(0);
static MOTORA: AtomicU32 = AtomicU32::new(0);
// static CHSTATE: Channel<ThreadModeRawMutex, sys::SysSTATE, 1> = Channel::new();
static CHANPRINT: Channel<ThreadModeRawMutex, String<10>, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut config = usart::Config::default();
    config.baudrate = 115200;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = usart::StopBits::STOP1;

    let usart = Uart::new(
        p.LPUART1, p.PG8, p.PG7, Irqs, p.DMA1_CH3, p.DMA1_CH4, config,
    )
    .unwrap();
    //TX PA2 PB11 PA101 PG7
    //RX PA3 PB10 PC0 PG8
    serial::uart_init(usart, spawner);

    // cs_pin: chip select pin
    // let cs = Output::new(p.PD14.degrade(), Level::Low, Speed::High);
    // // rst:  display reset pin, managed at driver level
    // let rst = Output::new(p.PD15.degrade(), Level::High, Speed::High);
    // // dc: data/command selection pin, managed at driver level
    // let dc = Output::new(p.PF12.degrade(), Level::High, Speed::High);
    // let mut backlight = Output::new(p.PB9, Level::High, Speed::High);

    // let mut spi_config = spi::Config::default();
    // spi_config.frequency = mhz(1);

    // //let spi = spi::Spi::new_blocking(p.SPI4, p.PE12, p.PE14, p.PE5, spi_config);
    // let spi = spi::Spi::new(
    //     p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA1_CH1, p.DMA1_CH2, spi_config,
    // );
    // let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    // let mut display = ST7735::<_, _, _, 161, 130, { buffer_size(161, 130) }>::new(
    //     spi_dev,
    //     dc,
    //     rst,
    //     Default::default(),
    // );
    // display.init(&mut Delay).await.unwrap();
    // display.clear(Rgb565::BLACK).unwrap();

    // // draw ferris
    // let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("ferris.raw"), 86);
    // let image: Image<_> = Image::new(&image_raw, Point::new(34, 8));
    // image.draw(&mut display).unwrap();
    // let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("ferris.bmp")).unwrap();
    // let image = Image::new(&raw_image, Point::new(34, 24));
    // image.draw(&mut display).unwrap();

    // display.flush().await.unwrap();

    // LED is set to max, but can be modulated with pwm to change backlight brightness

    // let step = p.PF15;
    // let dir = p.PE11;

    // spawner.spawn(sys::sys_task(dir, step)).unwrap();

    // #define enPin PF12    // 8
    // #define stepXPin PF15 // X.STEP
    // #define dirXPin PE11  // X.DIR
    // #define stepYPin PE13 // Y.STEP
    // #define dirYPin PE9   // Y.DIR
    // #define stepZPin PF14 // Z.STEP
    // #define dirZPin PF13  // Z.DIR

    // Timer::after_millis(100).await;
    // backlight.set_high();
    println("Hello World");
    println("Hello World");
    // Timer::after(Duration::from_millis(700)).await;
    // backlight.set_low();
    // Timer::after(Duration::from_millis(300)).await;
    loop {
        println("Hello World");
    }
}

// use core::fmt::{Debug, Write};
// use embedded_hal::blocking::delay::DelayUs;
// use embedded_hal::digital::v2::{InputPin, OutputPin};
// use one_wire_bus::OneWire;

// fn find_devices<P, E>(delay: &mut impl DelayUs<u16>, tx: &mut impl Write, one_wire_pin: P)
// where
//     P: OutputPin<Error = E> + InputPin<Error = E>,
//     E: Debug,
// {
//     let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
//     for device_address in one_wire_bus.devices(false, delay) {
//         // The search could fail at any time, so check each result. The iterator automatically
//         // ends after an error.
//         let device_address = device_address.unwrap();

//         // The family code can be used to identify the type of device
//         // If supported, another crate can be used to interact with that device at the given address
//         writeln!(
//             tx,
//             "Found device at address {:?} with family code: {:#x?}",
//             device_address,
//             device_address.family_code()
//         )
//         .unwrap();
//     }
// }
