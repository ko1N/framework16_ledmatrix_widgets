#![allow(dead_code)]
use crate::matrix;
use serialport::{SerialPortInfo, SerialPortType};
use std::{
    io::{Read, Write},
    thread,
    time::{Duration, SystemTime},
};

const BRIGHTNESS_CMD: u8 = 0x00;
const PATTERN_CMD: u8 = 0x01;
const BOOTLOADER_CMD: u8 = 0x02;
const SLEEP_CMD: u8 = 0x03;
const ANIMATE_CMD: u8 = 0x04;
const PANIC_CMD: u8 = 0x05;
const DRAW_CMD: u8 = 0x06;
const SET_COL: u8 = 0x07;
const COMMIT_COL: u8 = 0x08;

const CHECKFW_CMD: u8 = 0x20;

const CMD_START: [u8; 2] = [0x32, 0xAC];

pub struct LedMatrix {
    port: Box<dyn serialport::SerialPort>,
    pub port_info: SerialPortInfo,
}

impl LedMatrix {
    /// Find LED matrices connected to the laptop.
    ///
    /// This scans serial USB devices for the known Framework module VID/PID pair.
    pub fn detect() -> Result<Vec<LedMatrix>, String> {
        let sports = serialport::available_ports()
            .map_err(|err| format!("unable to list serial ports: {err}"))?;

        // Loop through all available serial ports, save ports that match the LED matrix product name
        let mut found_ledmat: Vec<SerialPortInfo> = vec![];
        for ref sp in sports {
            if let SerialPortType::UsbPort(info) = &sp.port_type {
                let info_c = info.clone();
                if info_c.vid == 12972 && info_c.pid == 32 {
                    found_ledmat.push(sp.clone());
                }
            }
        }

        if found_ledmat.is_empty() {
            return Ok(Vec::new());
        }

        let mut mats: Vec<LedMatrix> = Vec::new();
        for m in found_ledmat {
            mats.push(LedMatrix::new(m)?);
        }

        for i in mats.iter_mut() {
            let fw_version = i.get_fw_version()?;
            log::info!("{} - {}", i.port_info.port_name, fw_version);
        }

        Ok(mats)
    }

    /// Create and connect to one LED matrix module.
    pub fn new(portinfo: SerialPortInfo) -> Result<Self, String> {
        let port0builder = serialport::new(portinfo.port_name.to_string(), 115_200);
        let port0 = port0builder
            .open()
            .map_err(|err| format!("failed to open serial port {}: {err}", portinfo.port_name))?;

        Ok(Self {
            port: port0,
            port_info: portinfo,
        })
    }

    /// Send one command packet to the LED matrix module.
    pub fn sendcommand(&mut self, cmd: u8, params: Option<&[u8]>) -> Result<(), String> {
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(CMD_START.as_slice());
        buffer.push(cmd);

        if let Some(p) = params {
            buffer.extend_from_slice(p);
        }

        let _ = self
            .port
            .write(buffer.as_slice())
            .map_err(|err| format!("port write failed: {err}"))?;
        self.port
            .flush()
            .map_err(|err| format!("port flush failed: {err}"))?;

        Ok(())
    }

    /// Read up to `numbytes` from serial, waiting up to `timeout` for data.
    pub fn serialread(&mut self, numbytes: usize, timeout: Duration) -> Result<Vec<u8>, String> {
        let start_t = SystemTime::now();

        // Wait for bytes to be available
        while self
            .port
            .bytes_to_read()
            .map_err(|err| format!("failed to query serial buffer size: {err}"))?
            < 1
        {
            if start_t
                .elapsed()
                .map_err(|err| format!("system clock error during serial read: {err}"))?
                > timeout
            {
                return Err("serial read timed out".to_string());
            }
            thread::sleep(Duration::from_millis(10));
        }

        let mut buffer: Vec<u8> = vec![0; numbytes];

        while self
            .port
            .bytes_to_read()
            .map_err(|err| format!("failed to query serial buffer size: {err}"))?
            > 0
        {
            let bytes_read = self
                .port
                .read(buffer.as_mut_slice())
                .map_err(|err| format!("serial read failed: {err}"))?;
            if bytes_read == 0 {
                break;
            }
        }

        Ok(buffer)
    }

    ///
    /// Get the current firmware version of the LED matrix module.
    ///
    pub fn get_fw_version(&mut self) -> Result<String, String> {
        self.sendcommand(CHECKFW_CMD, None)?;
        let bytes = self.serialread(32, Duration::from_secs(5))?;
        if bytes.len() < 3 {
            return Ok(String::new());
        }

        let major = bytes[0];
        let minor = (bytes[1] & 0xF0) >> 4;
        let patch = bytes[1] & 0x0F;
        let pre_release = bytes[2] == 1;

        let version = format!("{}.{}.{} Pre Release: {}", major, minor, patch, pre_release);
        Ok(version)
    }

    ///
    /// Tell the module to wake up
    ///
    pub fn wake(&mut self) -> Result<(), String> {
        self.sendcommand(SLEEP_CMD, Some(&[0]))?;
        Ok(())
    }

    ///
    /// Tell the module to go to sleep
    ///
    pub fn sleep(&mut self) -> Result<(), String> {
        self.sendcommand(SLEEP_CMD, Some(&[1]))?;
        Ok(())
    }

    ///
    /// Draw a matrix using only ON/OFF commands. Each bit sent in the parameters
    /// is a LED, so a matrix needs to be encoded from a 9x34 array to a 39 byte array.
    /// There is no brightness control with this method.
    ///
    /// This allows for faster framerates than draw_matrix (with brightnesses) since its
    /// ~0.4% of the data (1/255)
    ///
    pub fn draw_bool_matrix(
        &mut self,
        mat: [[bool; matrix::MATRIX_WIDTH]; matrix::MATRIX_HEIGHT],
    ) -> Result<(), String> {
        let buffer = matrix::encode(mat);
        self.sendcommand(DRAW_CMD, Some(buffer.as_slice()))?;
        Ok(())
    }

    ///
    /// Sets the brightness of every LED in the module (0=OFF, 255=FULL)
    ///
    pub fn set_full_brightness(&mut self, val: u8) -> Result<(), String> {
        self.sendcommand(BRIGHTNESS_CMD, Some(&[val]))?;
        Ok(())
    }

    ///
    /// Write a single column of LEDs - indexed from left to right, 0-8.
    /// This has brightness control, where 0=OFF and 255=FULL brightness.
    /// Columns are not changed until the commit_col function is run (Allows you to
    /// write all the columns THEN display them at once)
    ///
    pub fn set_col(&mut self, col: u8, arr: [u8; 34]) -> Result<(), String> {
        let mut vec = vec![];
        vec.push(col);
        vec.extend_from_slice(arr.as_slice());
        self.sendcommand(SET_COL, Some(vec.as_slice()))?;
        Ok(())
    }

    ///
    /// Tell the module to display all the LEDs written to with set_col
    ///
    pub fn commit_col(&mut self) -> Result<(), String> {
        self.sendcommand(COMMIT_COL, Some(&[]))?;
        Ok(())
    }

    ///
    /// Display an entire matrix with individual LED brightness values. Slow updating,
    /// but allows for more complex UIs
    ///
    pub fn draw_matrix(
        &mut self,
        mat: [[u8; matrix::MATRIX_WIDTH]; matrix::MATRIX_HEIGHT],
    ) -> Result<(), String> {
        // Transpose array
        let tpose = matrix::transpose(mat);

        for (col, column_data) in tpose.iter().enumerate() {
            self.set_col(col as u8, *column_data)?;
        }

        self.commit_col()?;

        Ok(())
    }
}
