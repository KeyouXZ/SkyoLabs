use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use esp_hal::{
    Blocking,
    i2c::{self, master::I2c},
};

// Command
pub mod commands {
    pub const DISPLAY_OFF: u8 = 0xAE;
    pub const DISPLAY_ON: u8 = 0xAF;

    pub const SET_DISPLAY_CLOCK_DIV: u8 = 0xD5;
    pub const SET_MULTIPLEX_RATIO: u8 = 0xA8;
    pub const SET_DISPLAY_OFFSET: u8 = 0xD3;
    pub const SET_START_LINE: u8 = 0x40;

    pub const CHARGE_PUMP: u8 = 0x8D;

    pub const MEMORY_MODE: u8 = 0x20;
    pub const COLUMN_ADDR: u8 = 0x21;
    pub const PAGE_ADDR: u8 = 0x22;

    pub const SEG_REMAP: u8 = 0xA0;
    pub const COM_SCAN_DEC: u8 = 0xC8;

    pub const SET_COM_PINS: u8 = 0xDA;

    pub const SET_CONTRAST: u8 = 0x81;

    pub const NORMAL_DISPLAY: u8 = 0xA6;
    pub const INVERT_DISPLAY: u8 = 0xA7;

    // Scroll
    pub const RIGHT_HORIZONTAL_SCROLL: u8 = 0x26;
    pub const LEFT_HORIZONTAL_SCROLL: u8 = 0x27;
    pub const ACTIVATE_SCROLL: u8 = 0x2F;
    pub const DEACTIVATE_SCROLL: u8 = 0x2E;
}

#[derive(Debug, Clone, Copy)]
pub enum Ssd1306Size {
    S128x64,
    S128x32,
    S96x16,
}

impl Ssd1306Size {
    pub fn width(&self) -> usize {
        match self {
            Ssd1306Size::S128x64 | Ssd1306Size::S128x32 => 128,
            Ssd1306Size::S96x16 => 96,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Ssd1306Size::S128x64 => 64,
            Ssd1306Size::S128x32 => 32,
            Ssd1306Size::S96x16 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollSpeed {
    Slow,
    Medium,
    Fast,
}

const MAX_BUFFER_SIZE: usize = 1024;

/// Represents an SSD1306 OLED display driver.
pub struct Ssd1306<'a> {
    i2c: I2c<'a, Blocking>,
    address: u8,
    size: Ssd1306Size,
    buffer: [u8; MAX_BUFFER_SIZE],
}

impl<'a> Ssd1306<'a> {
    /// Creates a new SSD1306 driver instance with the given I2C interface and display size.
    pub fn new(i2c: I2c<'a, Blocking>, size: Ssd1306Size) -> Self {
        Self {
            i2c,
            address: 0x3C,
            size,
            buffer: [0; MAX_BUFFER_SIZE],
        }
    }

    /// Creates a new SSD1306 driver instance with the given I2C interface, display size, and address.
    pub fn with_address(i2c: I2c<'a, Blocking>, size: Ssd1306Size, address: u8) -> Self {
        Self {
            i2c,
            address,
            size,
            buffer: [0; MAX_BUFFER_SIZE],
        }
    }

    fn pages(&self) -> usize {
        self.size.height() / 8
    }

    fn buffer_size(&self) -> usize {
        self.size.width() * self.size.height() / 8
    }

    /// Initializes the SSD1306 display.
    pub fn init(&mut self) -> Result<(), i2c::master::Error> {
        // Reset display
        self.command(&[commands::DISPLAY_OFF])?;

        // Set display clock div
        self.command(&[commands::SET_DISPLAY_CLOCK_DIV, 0x80])?;

        // Set multiplex ratio to match the display height
        self.command(&[
            commands::SET_MULTIPLEX_RATIO,
            (self.size.height() - 1) as u8,
        ])?;

        // Set display offset and start line
        self.command(&[commands::SET_DISPLAY_OFFSET, 0x00])?;
        self.command(&[commands::SET_START_LINE])?;
        // Charge pump
        self.command(&[commands::CHARGE_PUMP, 0x14])?;

        // Set memory mode and segment remap
        self.command(&[commands::MEMORY_MODE, 0x00])?;

        // Set segment remap and COM scan direction
        self.command(&[commands::SEG_REMAP | 0x01])?;
        self.command(&[commands::COM_SCAN_DEC])?;

        // Set COM pins and contrast
        match self.size {
            Ssd1306Size::S128x64 => {
                self.command(&[commands::SET_COM_PINS, 0x12])?;
            }

            Ssd1306Size::S128x32 => {
                self.command(&[commands::SET_COM_PINS, 0x02])?;
            }

            Ssd1306Size::S96x16 => {
                self.command(&[commands::SET_COM_PINS, 0x02])?;
            }
        }
        self.command(&[commands::SET_CONTRAST, 0x7F])?;

        // Set normal display and deactivate scroll
        self.command(&[commands::NORMAL_DISPLAY])?;
        self.command(&[commands::DEACTIVATE_SCROLL])?;

        self.command(&[commands::DISPLAY_ON])?;

        Ok(())
    }

    /// Sends a command to the SSD1306 display.
    pub fn command(&mut self, cmd: &[u8]) -> Result<(), i2c::master::Error> {
        let mut data = [0u8; 16];

        data[0] = 0x00;
        data[1..1 + cmd.len()].copy_from_slice(cmd);

        self.i2c.write(self.address, &data[..cmd.len() + 1])?;

        Ok(())
    }

    /// Sends data to the SSD1306 display.
    pub fn data(&mut self, data: &[u8]) -> Result<(), i2c::master::Error> {
        const CHUNK_SIZE: usize = 32;
        let mut buf = [0u8; CHUNK_SIZE + 1];
        buf[0] = 0x40; // Data control byte

        for chunk in data.chunks(CHUNK_SIZE) {
            buf[1..1 + chunk.len()].copy_from_slice(chunk);
            self.i2c.write(self.address, &buf[..1 + chunk.len()])?;
        }
        Ok(())
    }

    /// Flushes the buffer to the SSD1306 display.
    pub fn flush(&mut self) -> Result<(), i2c::master::Error> {
        self.command(&[commands::COLUMN_ADDR, 0, (self.size.width() - 1) as u8])?;
        self.command(&[commands::PAGE_ADDR, 0, (self.pages() - 1) as u8])?;

        let buf_size = self.buffer_size();
        const CHUNK_SIZE: usize = 32;
        let mut tx_buf = [0u8; CHUNK_SIZE + 1];
        tx_buf[0] = 0x40; // Data control byte

        let mut offset = 0;
        while offset < buf_size {
            let chunk_len = core::cmp::min(CHUNK_SIZE, buf_size - offset);

            // Copy data to the buffer and send it via I2C
            tx_buf[1..1 + chunk_len].copy_from_slice(&self.buffer[offset..offset + chunk_len]);
            self.i2c.write(self.address, &tx_buf[..1 + chunk_len])?;

            offset += chunk_len;
        }

        Ok(())
    }

    /// Clears the buffer, filling it with zeros.
    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    /// Sets an individual pixel (x, y) to on (true) or off (false).
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        if x >= self.size.width() || y >= self.size.height() {
            return; // Prevents out-of-bounds error
        }

        let byte_idx = x + (y / 8) * self.size.width();
        let bit_idx = y % 8;

        if on {
            self.buffer[byte_idx] |= 1 << bit_idx;
        } else {
            self.buffer[byte_idx] &= !(1 << bit_idx);
        }
    }

    pub fn draw_glyph<T>(&mut self, glyph: &[T], width: usize, x: usize, y: usize)
    where
        T: Copy + Into<u32>,
    {
        for (row, &byte) in glyph.iter().enumerate() {
            for col in 0..width {
                let on = byte.into() & (1 << (width - 1 - col)) != 0;

                self.set_pixel(x + col, y + row, on);
            }
        }
    }

    pub fn start_scroll(
        &mut self,
        direction: ScrollDirection,
        start_page: u8,
        end_page: u8,
    ) -> Result<(), i2c::master::Error> {
        let command = match direction {
            ScrollDirection::Left => commands::LEFT_HORIZONTAL_SCROLL,
            ScrollDirection::Right => commands::RIGHT_HORIZONTAL_SCROLL,
        };

        self.command(&[
            command, 0x00,       // Dummy byte
            start_page, // Start page
            0x00,       // Frame interval
            end_page,   // End page
            0x00,       // Dummy byte
            0xFF,       // Dummy byte
        ])?;

        self.command(&[commands::ACTIVATE_SCROLL])?;

        Ok(())
    }

    pub fn stop_scroll(&mut self) -> Result<(), i2c::master::Error> {
        self.command(&[commands::DEACTIVATE_SCROLL])?;

        Ok(())
    }
}

// Implements the OriginDimensions trait for Ssd1306, providing the size of the display.
impl<'a> OriginDimensions for Ssd1306<'a> {
    fn size(&self) -> Size {
        Size::new(self.size.width() as u32, self.size.height() as u32)
    }
}

// Implements the DrawTarget trait for Ssd1306, allowing drawing of pixels on the display.
impl<'a> DrawTarget for Ssd1306<'a> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let x = coord.x;
            let y = coord.y;

            if x >= 0 && x < (self.size.width() as i32) && y >= 0 && y < (self.size.height() as i32)
            {
                let on = color == BinaryColor::On;
                self.set_pixel(x as usize, y as usize, on);
            }
        }
        Ok(())
    }
}
