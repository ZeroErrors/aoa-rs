//! Provides the ability to use [Android Open Accessory Protocol 2.0](https://source.android.com/devices/accessories/aoa2)
//!
//! AOAv2 audio support has been deprecated as of Android 8.0 so should generally be avoided if possible.
//!
//! Information on alternatives: https://source.android.com/devices/audio/usb
use std::time::Duration;

use rusb::{request_type, DeviceHandle, Direction, Recipient, RequestType, UsbContext};

use crate::AccessoryError;

pub const USB_AUDIO_PRODUCT_ID: u16 = 0x2D02;
pub const USB_AUDIO_ADB_PRODUCT_ID: u16 = 0x2D03;
pub const USB_ACCESSORY_AUDIO_PRODUCT_ID: u16 = 0x2D04;
pub const USB_ACCESSORY_AUDIO_ADB_PRODUCT_ID: u16 = 0x2D05;

pub const ACCESSORY_SET_AUDIO_MODE: u8 = 0x3A;

#[repr(u16)]
#[derive(Debug)]
pub enum AudioMode {
    NoAudio = 0u16,
    // 2 channel, 16-bit PCM at 44100 KHz
    DualChannel16bitPCM44100khz = 1,
}

pub trait AOA2Handle {
    /// Sends the `SET_AUDIO_MODE` control request.
    ///
    /// See: https://source.android.com/devices/accessories/aoa2#audio-support
    fn send_audio_mode(
        &mut self,
        audio_mode: AudioMode,
        timeout: Duration,
    ) -> Result<(), AccessoryError>;

    // TODO: Add HID support? https://source.android.com/devices/accessories/aoa2#hid-support
}

impl<T: UsbContext> AOA2Handle for DeviceHandle<T> {
    fn send_audio_mode(
        &mut self,
        audio_mode: AudioMode,
        timeout: Duration,
    ) -> Result<(), AccessoryError> {
        self.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
            ACCESSORY_SET_AUDIO_MODE,
            audio_mode as u16,
            0,
            &[],
            timeout,
        )?;

        Ok(())
    }
}
