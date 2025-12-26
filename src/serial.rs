// use crate::sys::SysSTATE;
use crate::CHANPRINT;
// use crate::CHSTATE;
use crate::SHARED;
use core::{fmt::Write, str, sync::atomic::Ordering};
use defmt::{info, unwrap};
use embassy_executor::Spawner;

use embassy_stm32::usart::UartRx;
use embassy_stm32::usart::UartTx;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, mode::Async, peripherals, usart};
use heapless::String;

pub fn uart_init(uart: Uart<'static, Async>, spawner: Spawner) {
    let (tx, rx) = uart.split();
    spawner.spawn(uart_rx_task(rx)).unwrap();
    info!("UART Init!");
    spawner.spawn(uart_tx_task(tx)).unwrap();
}

pub fn print(msg: &str) {
    let mut prnt: String<10> = String::new();
    let _ = prnt.write_str(msg);

    let _ = CHANPRINT.try_send(prnt);
}
pub fn println(msg: &str) {
    let mut prnt: String<10> = String::new();
    let _ = prnt.write_fmt(format_args!("\r\n{}", msg));

    let _ = CHANPRINT.try_send(prnt);
}

#[embassy_executor::task]
pub async fn uart_rx_task(mut rx: UartRx<'static, Async>) {
    // let shared_var = SHARED.load(Ordering::Relaxed);
    info!("UART RX TASK Running!");
    let mut buf = [0; 8];
    let mut datain: String<8> = String::new();
    loop {
        // info!("waiting for RX!");
        let n = unwrap!(rx.read_until_idle(&mut buf).await);
        info!("RX received");
        for i in 0..n {
            let _ = datain.push(buf[i] as char);
        }

        let mut str_match = false;
        match datain.as_str() {
            "a" => {
                print("\r\nyes1");
                // unwrap!(tx.write("\r\nyes1".as_bytes()).await);

                str_match = true;
            }
            "bc" => {
                print("\r\n23");

                str_match = true;
                //  SHARED.store(shared_var.wrapping_add(1), Ordering::Relaxed);
            }
            "r" => {
                // let _ = CHSTATE.try_send(SysSTATE::MOVE);
                str_match = true;
            }
            "c" => str_match = true,
            _ => {
                if datain.contains("\r") {
                    // unwrap!(tx.write("\r\n".as_bytes()).await);
                    str_match = true;
                }
            }
        }
        if str_match {
            datain.clear()
        }
        buf.fill(0);
    }
}
#[embassy_executor::task]
pub async fn uart_tx_task(mut tx: UartTx<'static, Async>) {
    info!("UART TX TASK Running!");
    loop {
        let msg = CHANPRINT.receive().await;
        unwrap!(tx.write(msg.as_bytes()).await);
        // for n in 0u32.. {
        //     let mut s: String<128> = String::new();
        //     core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();
        //     tx.write(s.as_bytes()).await.ok();
        // }
    }
    info!("UART TX TASK Ended!");
}

// for n in 0u32.. {
//     let mut s: String<128> = String::new();
//     core::write!(&mut s, "Hello DMA World {}!\r\n", n).unwrap();

//     info!("Writing...");
//     usart.write(s.as_bytes()).await.ok();

//     info!("wrote DMA");
// }
