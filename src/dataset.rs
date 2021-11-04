use crate::colors;
use crate::errors;

use serde::{Serialize, Deserialize};
use serde_json;


pub const MAX_FILE_LEN: usize = 10 * 1024 * 1024; // 10Mb

#[derive(Serialize, Deserialize)]
pub struct Highlight {
    pub name: String,
    pub offset: usize,
    pub length: usize,
    #[serde(skip)]
    pub color: colors::Colors,
}

impl Highlight {
    pub fn new(highlight_name: String,  
        offset:usize, 
        length: usize) -> Result<Self, errors::DataBrushErrors> {

        if length == 0 {
            return Err(errors::DataBrushErrors::HighlightSizeZero(highlight_name));
        }

        Ok( Highlight {
            name: highlight_name,
            offset,
            length,
            color: colors::COLOR_MAP[0],
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
  pub name: String,
  pub offset: usize,
  pub length: usize,
  pub highlights: Vec<Highlight>,
}

impl Chunk {
  pub fn new(chunk_name: String, chunk_len: usize) -> Result<Self, errors::DataBrushErrors> {
      if chunk_len == 0 {
          return Err(errors::DataBrushErrors::ChunkSizeZero(chunk_name));
      }
      
      Ok(Chunk { 
          name: chunk_name, 
          offset: 0, 
          length: chunk_len, 
          highlights: vec!(),
      })
  }

  pub fn set_highlight(&mut self, mut highlight: Highlight) -> Result<(), errors::DataBrushErrors> {
      // Select random color
      highlight.color = colors::COLOR_MAP[self.highlights.len() % 6];

      // Check for overlapping highlights
      let lower_band = highlight.offset;
      let upper_band = lower_band + highlight.length;

      for hl in &self.highlights {
          if lower_band >= hl.offset && lower_band < (hl.offset + hl.length) {
              return Err(errors::DataBrushErrors::HighlightOverlap);
          }

          if upper_band > hl.offset && upper_band <= (hl.offset + hl.length) {
              return Err(errors::DataBrushErrors::HighlightOverlap);
          }
      }

      self.highlights.push(highlight);
      Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct Dataset {
  pub name: String,
  #[serde(skip)]
  pub data:  Vec<u8>,
  pub chunks: Vec<Chunk>,
}

impl Dataset {
  pub fn new(set_name: String, data_set: Vec<u8>) -> Result<Dataset, errors::DataBrushErrors> {
      if data_set.is_empty() {
          return Err(errors::DataBrushErrors::EmptyDataset)
      }

      if data_set.len() > MAX_FILE_LEN {
          return Err(errors::DataBrushErrors::FileTooLarge) 
      }

      Ok(Dataset {
          name: set_name,
          data: data_set,
          chunks: vec!(),
      })
  }

  pub fn from_json(template: Vec<u8>, data_set: Vec<u8>) -> Result<Dataset, errors::DataBrushErrors> {
      // read and parse template
    if template.is_empty() {
        return Err(errors::DataBrushErrors::EmptyTemplate)
    }

    // Add dataset
    if data_set.is_empty() {
        return Err(errors::DataBrushErrors::EmptyDataset)
    }

    if data_set.len() > MAX_FILE_LEN {
        return Err(errors::DataBrushErrors::FileTooLarge) 
    }


    let mut dataset: Dataset = serde_json::from_slice(&template).unwrap();
    // attach data
    dataset.data = data_set;

    // validate created dataset
    dataset.validate()?;

    for chunk in dataset.chunks.iter_mut() {
        for (index, hl) in chunk.highlights.iter_mut().enumerate() {
            hl.color = colors::COLOR_MAP[index % 6];
        }
    }

    Ok(dataset)
  }

  fn validate(&self) -> Result<(), errors::DataBrushErrors> {

    // Validating overlapping chunks
    let mut lower_band = 0;
    let upper_band = self.data.len();

    for chunk in &self.chunks {
        if chunk.length == 0 {
            return Err(errors::DataBrushErrors::ChunkSizeZero(chunk.name.clone()));
        }

        if chunk.offset < lower_band || chunk.offset + chunk.length > upper_band {
            println!("data[{}:{}] -> {}-{}", lower_band, upper_band, chunk.offset, chunk.offset + chunk.length);
            return Err(errors::DataBrushErrors::ChunkOverflow)
        }

        // Check for overlapping highlights
        let mut hl_lower_band = chunk.offset;
        let hl_upper_band = hl_lower_band + chunk.length;

        for hl in &chunk.highlights {
            if hl.length == 0 {
                return Err(errors::DataBrushErrors::HighlightSizeZero(hl.name.clone()))
            }

            if hl.offset < hl_lower_band || hl.offset + hl.length > hl_upper_band {
                println!("chunk[{}:{}] -> {}-{}", hl_lower_band, hl_upper_band, hl.offset, hl.offset + hl.length);
                return Err(errors::DataBrushErrors::HighlightOverlap);
            }
            hl_lower_band = hl.offset + hl.length;
        }
        lower_band = chunk.offset + chunk.length;
    }

    Ok(())
  }

  pub fn add_chunk(&mut self, mut new_chunk: Chunk) -> Result<(), errors::DataBrushErrors> {
      // Calculate offset
      let mut chunk_offset = 0;

      if let Some(chunk) = self.chunks.last() {
          chunk_offset = chunk.offset + chunk.length;
      }

      if chunk_offset + new_chunk.length > self.data.len() {
          return Err(errors::DataBrushErrors::ChunkOverflow);
      }

      new_chunk.offset = chunk_offset;
      new_chunk.highlights.sort_by(|a, b| a.offset.cmp(&b.offset));

      self.chunks.push(new_chunk);

      Ok(())
  }

  fn _set_chunk(&mut self, new_chunk: Chunk) {
      // TODO(carstein: Add check for overlapping chunks
      
      self.chunks.push(new_chunk)
  }

  fn _last_chunk(&mut self, last_chunk_name: String) {
      let last = self.chunks.last().unwrap();
      let chunk_len = last.length + last.offset;

      self.chunks.push(Chunk {
          name: last_chunk_name, 
          offset: chunk_len,
          length: self.data.len() - chunk_len,
          highlights: vec!(),
      });
  }
}