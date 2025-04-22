use std::fmt;
use fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result};
use env_logger::fmt::style::Style;
use env_logger::Builder;
use log::LevelFilter;

use crate::GLOBAL;

static TARGET_FORMATTER: PaddingFormatter = PaddingFormatter::new();
static MEM_FORMATTER: PaddingFormatter = PaddingFormatter::new();

pub fn try_init(level_filter: LevelFilter) -> Result<()> {
    let mut builder = Builder::new();

    builder.filter_level(level_filter);

    builder.format(|f, record| {
        use std::io::Write;

        let target = record.target();
        let target_padded = TARGET_FORMATTER.pad(target);

        let mem = GLOBAL.get_bytesize();
        let mem_padded = MEM_FORMATTER.pad(mem);

        let args = record.args();

        let level = record.level();

        let time = f.timestamp_millis();

        let level_style = f.default_level_style(level);
        let target_style = Style::new().bold();

        writeln!(f, "[{time}][{level_style}{level}{level_style:#}][{target_style}{target_padded}{target_style:#}][MEM:{mem_padded}] {args}")
    });

    builder.try_init().context("could not initialize logger")?;
    Ok(())
}

struct Padded<'a, T> {
    value: T,
    formatter: &'a PaddingFormatter,
}

impl<'a, T: fmt::Display> fmt::Display for Padded<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_width = self.formatter.max_width.load(Ordering::Relaxed);

        let mut tw = TrackingWriter { writer: f, written_chars: 0 };
        write!(tw, "{: <max_width$}", self.value)?;
        let width = tw.written_chars;

        if width > max_width {
            self.formatter.max_width.store(width, Ordering::Relaxed);
        }
        Ok(())
    }
}

struct PaddingFormatter {
    max_width: AtomicUsize,
}

impl PaddingFormatter {
    const fn new() -> Self {
        Self {
            max_width: AtomicUsize::new(0)
        }
    }

    fn pad<T: fmt::Display>(&self, value: T) -> Padded<T> {
        Padded { value, formatter: self }
    }
}

struct TrackingWriter<W> {
    writer: W,
    written_chars: usize,
}

impl<W: fmt::Write> fmt::Write for TrackingWriter<W> {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.writer.write_char(c)?;
        self.written_chars += 1;
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)?;
        self.written_chars += s.chars().count();
        Ok(())
    }
}
