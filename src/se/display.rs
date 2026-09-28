use crate::{
    driver,
    se::{self},
};
use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::*},
    pixelcolor::BinaryColor,
    text::Text,
};
use heapless::String;

pub fn create_header<'a>() -> Text<'a, MonoTextStyle<'a, BinaryColor>> {
    // Setup text style for display
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    let text = "SkyoLabs";

    // 9 from font width
    let width = text.len() as i32 * 9;
    let x = (se::core::SSD1306_SCREEN_SIZE.width() as i32 - width) / 2;

    Text::new(text, Point::new(x, 10), text_style)
}

fn draw_glyph<'a>(
    display: &mut driver::ssd1306::Ssd1306<'a>,
    level: se::core::Level,
    x: usize,
    y: usize,
) {
    let glyph = match level {
        // se::core::Level::Safe => &se::glyph::SAFE,
        se::core::Level::Safe => return,
        se::core::Level::Warning => &se::glyph::WARNING,
        se::core::Level::Danger => &se::glyph::DANGER,
    };

    let size = 10;
    display.draw_glyph(glyph, size, x, y - 16 / 2);
}

pub fn display_info<'a>(
    display: &mut driver::ssd1306::Ssd1306<'a>,
    header: Text<'a, MonoTextStyle<'a, BinaryColor>>,
    temp: f32,
    hum: f32,
    gas: f32,
    temp_level: se::core::Level,
    hum_level: se::core::Level,
    gas_level: se::core::Level,
) {
    display.clear();

    // Draw the header
    display.draw_glyph(&se::glyph::WIFI, 10, 0, 0);
    header.draw(display);

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X12)
        .text_color(BinaryColor::On)
        .build();

    let glyph_x = (128 - 16) as usize;

    // Draw the temperature
    let mut text: String<32> = String::new();
    write!(&mut text, "Temp : {:.1} C", temp).unwrap();
    Text::new(&text, Point::new(0, 25), text_style).draw(display);

    // Draw temp status
    draw_glyph(display, temp_level, glyph_x, 25);

    // Draw the humidity
    let mut text: String<32> = String::new();
    write!(&mut text, "Hum  : {:.1} %", hum).unwrap();
    Text::new(&text, Point::new(0, 35), text_style).draw(display);

    // Draw the humidity status
    draw_glyph(display, hum_level, glyph_x, 35);

    // Draw the gas
    let mut text: String<32> = String::new();
    write!(&mut text, "Gas  : {}", gas).unwrap();
    Text::new(&text, Point::new(0, 45), text_style).draw(display);

    // Draw the gas status
    draw_glyph(display, gas_level, glyph_x, 45);

    // Draw the status
    // let mut text: String<32> = String::new();
    // match level {
    //     Level::Safe => write!(&mut text, "Status: AMAN").unwrap(),
    //     Level::Warning => write!(&mut text, "Status: WASPADA").unwrap(),
    //     Level::Danger => write!(&mut text, "Status: BAHAYA").unwrap(),
    // };
    // Text::new(&text, Point::new(0, 55), text_style).draw(display);

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .build();

    let text = "By Kelompok 2 XII-4";
    let width = text.len() as i32 * 5;
    let x = (se::core::SSD1306_SCREEN_SIZE.width() as i32 - width) / 2;
    Text::new(text, Point::new(x as i32, 60), text_style).draw(display);

    let _ = display.flush();
}
