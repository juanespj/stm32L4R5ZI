use embassy_stm32::gpio::{Input, Output};
pub struct MotionPins {
    pub all_enable_pin: Output<'static>, // D8

    pub x_endstop_pin: Input<'static>, // D9
    pub y_endstop_pin: Input<'static>, // D10
    pub z_endstop_pin: Input<'static>, // D11

    pub x_step_pin: Output<'static>, // D2
    pub y_step_pin: Output<'static>, // D3
    pub z_step_pin: Output<'static>, // D4

    pub x_dir_pin: Output<'static>, // D5
    pub y_dir_pin: Output<'static>, // D6
    pub z_dir_pin: Output<'static>, // D7
}
// impl MotionPins {
//     pub fn disable(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         self.all_enable_pin.set_high();
//     }

//     pub fn enable(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         self.all_enable_pin.set_low();
//     }

//     pub fn set_forward_direction(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         cfg_if::cfg_if! {
//             if #[cfg(not(feature="debug-signals"))] {
//                 #[cfg(feature = "with-x-axis")]
//                 if _channels.contains(printhor_hwa_common::StepperChannel::X) {
//                     self.x_dir_pin.set_high();
//                 }
//                 else {
//                     self.x_dir_pin.set_low();
//                 }
//                 #[cfg(feature = "with-y-axis")]
//                 if _channels.contains(printhor_hwa_common::StepperChannel::Y) {
//                     self.y_dir_pin.set_high();
//                 }
//                 else {
//                     self.y_dir_pin.set_low();
//                 }
//                 #[cfg(feature = "with-z-axis")]
//                 if _channels.contains(printhor_hwa_common::StepperChannel::Z) {
//                     self.z_dir_pin.set_high();
//                 }
//                 else {
//                     self.z_dir_pin.set_low();
//                 }
//                 #[cfg(feature = "with-e-axis")]
//                 if _channels.contains(printhor_hwa_common::StepperChannel::E) {
//                     self.e_dir_pin.set_high();
//                 }
//                 else {
//                     self.e_dir_pin.set_low();
//                 }
//             }
//         }
//     }

//     pub fn step_toggle(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         #[cfg(feature = "with-x-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::X) {
//             self.x_step_pin.toggle();
//         }
//         #[cfg(feature = "with-y-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Y) {
//             self.y_step_pin.toggle();
//         }
//         #[cfg(feature = "with-z-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Z) {
//             self.z_step_pin.toggle();
//         }
//         #[cfg(feature = "with-e-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::E) {
//             self.e_step_pin.toggle();
//         }
//     }

//     pub fn step_high(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         #[cfg(feature = "with-x-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::X) {
//             self.x_step_pin.set_high();
//         }
//         #[cfg(feature = "with-y-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Y) {
//             self.y_step_pin.set_high();
//         }
//         #[cfg(feature = "with-z-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Z) {
//             self.z_step_pin.set_high();
//         }
//         #[cfg(feature = "with-e-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::E) {
//             self.e_step_pin.set_high();
//         }
//     }

//     pub fn step_low(&mut self, _channels: printhor_hwa_common::StepperChannel) {
//         #[cfg(feature = "with-x-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::X) {
//             self.x_step_pin.set_low();
//         }
//         #[cfg(feature = "with-y-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Y) {
//             self.y_step_pin.set_low();
//         }
//         #[cfg(feature = "with-z-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Z) {
//             self.z_step_pin.set_low();
//         }
//         #[cfg(feature = "with-e-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::E) {
//             self.e_step_pin.set_low();
//         }
//     }

//     pub fn endstop_triggered(&mut self, _channels: printhor_hwa_common::StepperChannel) -> bool {
//         #[allow(unused_mut)]
//         let mut triggered = false;
//         #[cfg(feature = "with-x-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::X) {
//             triggered |= self.x_endstop_pin.is_high();
//         }
//         #[cfg(feature = "with-y-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Y) {
//             triggered |= self.y_endstop_pin.is_high();
//         }
//         #[cfg(feature = "with-z-axis")]
//         if _channels.contains(printhor_hwa_common::StepperChannel::Z) {
//             triggered |= self.z_endstop_pin.is_high();
//         }
//         triggered
//     }
// }
