use colored::*;

use std::cmp;
use std::fmt::Write;

use crate::colors;
use crate::dataset;


#[derive(Copy, Clone)]
struct Brush {
    data: u8,
    color: colors::Colors,
}

struct Line {
    address: usize,
    fields: [Option<Brush>; 16],
}

impl Line {
    fn new() -> Line {
        Line {
            address: 0,
            fields: [None; 16],
        }
    }
}

pub struct Painter {
    offset: usize,

}

impl Default for Painter {
  fn default() -> Self {
    Self::new()
  }
}

impl Painter {
    pub fn new() -> Painter {
        Painter {
            offset: 0,
        }
    }

    pub fn prepare(mut self, set: &dataset::Dataset) -> Result<String, std::fmt::Error> {
        // calculate optimal size for output
        let mut output = String::with_capacity(set.data.len() * 6);

        // print out name of the set
        writeln!(output, "======== {}", set.name)?;

        // Start writing chunks
        for chunk in &set.chunks {
            // Set current highlight
            let mut hl_iterator = chunk.highlights.iter();
            let mut current_highlight = hl_iterator.next();

            // Prepare chunk name in reverse order
            let mut chunk_name_segments = vec!();
            let lines_num = (chunk.len / 16) + 1;
            match lines_num {
                1 => {
                    chunk_name_segments.push(format!("] {}", chunk.name))
                }
                2 => {
                    chunk_name_segments.push(String::from("┛"));
                    chunk_name_segments.push(format!("┓ {}", chunk.name));
                }
                3 => {
                    chunk_name_segments.push(String::from("┛"));
                    chunk_name_segments.push(format!("┃ {}", chunk.name));
                    chunk_name_segments.push(String::from("┓"));
                }
                _ => {
                    chunk_name_segments.push(String::from("┛"));
                    for _ in 0..(lines_num - 3) {
                        chunk_name_segments.push(String::from("┃"));
                    }
                    chunk_name_segments.push(format!("┃ {}", chunk.name));
                    chunk_name_segments.push(String::from("┓"));
                    println!("chunk name segments: {:?}", chunk_name_segments);
                }
            }

            // Line loop - figure out break condition
            while self.offset < (chunk.offset + chunk.len)  {
                let mut current_line = Line::new();
                current_line.address = self.offset & (usize::MAX << 4);

                // populate fields
                // determine if we need to print whole line,
                // just a fragment (end of the chunk)
                // or maybe start from the middle (star of new chunk)
                let upper_bound = 
                    cmp::min(chunk.offset + chunk.len - self.offset, 
                            16 - (self.offset % 16));
                
                let mut hl_length = 0;
                let mut hl_color = colors::Colors::Normal;
                for index in self.offset..(self.offset + upper_bound) {
                    if hl_length == 0 {
                        hl_color = colors::Colors::Normal;
                        match current_highlight {
                            Some(curr) => {
                                if curr.offset == index {
                                    hl_length = curr.length - 1;
                                    hl_color = *curr.color;
                                    current_highlight = hl_iterator.next();
                                }
                            },
                            None => {
                                hl_length = usize::MAX;
                            }
                        }
                    } else {
                        hl_length -= 1;
                    }

                    current_line.fields[index % 16] = Some(Brush {
                        data: set.data[index],
                        color: hl_color,
                    });
                    self.offset += 1;
                }

                // print line
                write!(output, "{:08x}┃ ", current_line.address )?;

                // first 8 bytes
                for index in 0..8 {
                    match &current_line.fields[index] {
                        Some(x) => write!(output, "{} ", 
                            paint(&format!("{:02X}", x.data), x.color))?,
                        None => write!(output, "   ")?,
                    }
                }
                // space
                write!(output, "  ")?;
                
                // last 8 bytes
                for index in 8..16 {
                    match &current_line.fields[index] {
                        Some(x) => write!(output, "{} ", 
                            paint(&format!("{:02X}", x.data), x.color))?,
                        None => write!(output, "   ")?,
                    }
                }

                //chunk name
                write!(output, " {}", 
                    chunk_name_segments.pop().unwrap_or_else(|| "".to_string()))?;

                // trailing new line
                writeln!(output)?;

            }
            // print out data highlights
            for hl in &chunk.highlights {
                writeln!(output, "-- {}", paint(&hl.name, *hl.color))?;
            }

            //Empty line after chunk
            writeln!(output)?;
        }

        Ok(output)
    }
}

fn paint(data: &str, color: colors::Colors) -> colored::ColoredString {
    match color {
        colors::Colors::Red =>  data.red(),
        colors::Colors::Blue => data.blue(),
        colors::Colors::Green => data.green(),
        colors::Colors::Cyan => data.cyan(),
        colors::Colors::Magenta => data.magenta(),
        colors::Colors::Yellow => data.yellow(),
        colors::Colors::Normal => data.normal(),
    }
}