use crate::serial;
use crate::serial::println;
// use crate::CHANPRINT;
// use crate::CHSTATE;
// use crate::MOTORA;
use core::pin::Pin;
// use core::{
//     fmt::{Error, Write},
//     str,
//     sync::atomic::{AtomicU32, Ordering},
// };
use defmt::{info, unwrap};
use embassy_executor::Spawner;
// use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
// use embassy_nrf::Peripherals;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
// use stepper::{
//     fugit::NanosDurationU32 as Nanoseconds, motion_control, ramp_maker, Direction, Stepper,
// };

#[derive(Debug, Clone, PartialEq)]
pub enum SysSTATE {
    INIT,
    IDLE,
    READ,
    MOVE,
    STOP,
}
pub struct SysCfg {
    pub state: SysSTATE,
    pub state_last: SysSTATE,
    pub new_state: bool,
    // pub btn3: Input<'static, AnyPin>,
    // pub btn4: Input<'static, AnyPin>,
}
impl Default for SysCfg {
    fn default() -> SysCfg {
        SysCfg {
            state: SysSTATE::INIT,
            state_last: SysSTATE::INIT,
            new_state: false,
        }
    }
}

// #[embassy_executor::task]
// pub async fn sys_task(dir: Pin, step: Pin) {
//     let mut timer = Timer::<1_000_000>::new();

//     type Num = fixed::FixedI64<typenum::U32>;
//     let target_accel = Num::from_num(0.001); // steps / tick^2; 1000 steps / s^2
//     let max_speed = Num::from_num(0.001); // steps / tick; 1000 steps / s

//     let profile = ramp_maker::Trapezoidal::new(target_accel);

//     let mut xaxis = Stepper::from_driver(MyDriver::new())
//         // Enable direction control
//         .enable_direction_control(dir, Direction::Forward, &mut timer)?
//         // Enable step control
//         .enable_step_control(step)
//         // Enable motion control using the software fallback
//         .enable_motion_control((timer, profile, DelayToTicks));
//     let mut sys = SysCfg::default();
//     serial::println("Init");
//     loop {
//         if sys.state != sys.state_last {
//             sys.new_state = true;
//         }
//         sys.state_last = sys.state.clone();
//         match sys.state {
//             SysSTATE::INIT => {
//                 println("INIT");
//                 sys.state = SysSTATE::IDLE;
//             }
//             SysSTATE::IDLE => todo!(),
//             SysSTATE::READ => todo!(),
//             SysSTATE::MOVE => {
//                 let target_step = 2000;
//                 xaxis.move_to_position(max_speed, target_step).wait()?;
//                 sys.state = SysSTATE::IDLE;
//             }
//             SysSTATE::STOP => todo!(),
//         };

//         // match sys.state {
//         //     SysSTATE::INIT => {
//         //         println("INIT");
//         //         sys.state = SysSTATE::IDLE;
//         //     }
//         //     SysSTATE::IDLE => {
//         //         if sys.new_state {
//         //             println("IDLE");
//         //             sys.new_state = false;
//         //         }
//         //         if let Ok(state) = CHSTATE.try_receive() {
//         //             println("msg in");
//         //             sys.state = state;
//         //         };
//         //     }
//         //     SysSTATE::READ => todo!(),
//         //     SysSTATE::MOVE => {
//         //         if sys.new_state {
//         //             MOTORA.store(40, Ordering::Relaxed);
//         //             sys.new_state = false;
//         //         }
//         //         println("MOVING");
//         //         Timer::after_millis(100).await;
//         //         println("DONE");
//         //         sys.state = SysSTATE::IDLE
//         //     }
//         //     SysSTATE::STOP => todo!(),
//         // }

//         Timer::after_millis(100).await;
//     }
// }

// // Here's the converter that Stepper is going to use internally, to convert
// // from the computed delay value to timer ticks. Since we chose to use timer
// // ticks as the unit of time for velocity and acceleration, this conversion
// // is pretty simple (and cheap).
// use num_traits::cast::ToPrimitive;
// pub struct DelayToTicks;
// impl<const TIMER_HZ: u32> motion_control::DelayToTicks<Num, TIMER_HZ> for DelayToTicks {
//     type Error = core::convert::Infallible;

//     fn delay_to_ticks(&self, delay: Num) -> Result<fugit::TimerDurationU32<TIMER_HZ>, Self::Error> {
//         Ok(fugit::TimerDurationU32::<TIMER_HZ>::from_ticks(
//             Num::to_u32(&delay).expect("the delay to convert"),
//         ))
//     }
// }
