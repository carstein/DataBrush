use crate::colors;
use crate::errors;

use serde::{Serialize, Deserialize};


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
  pub len: usize,
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
          len: chunk_len, 
          highlights: vec!(),
      })
  }

  pub fn set_highlight(&mut self, mut highlight: Highlight) -> Result<(), errors::DataBrushErrors> {
      // Select random color
      highlight.color = colors::COLOR_MAP[self.highlights.len() % 6];

      // Check for overlapping highlights
      let lower_bound = highlight.offset;
      let upper_bound = lower_bound + highlight.length;

      for hl in &self.highlights {
          if lower_bound >= hl.offset && lower_bound < (hl.offset + hl.length) {
              return Err(errors::DataBrushErrors::HighlightOverlap);
          }

          if upper_bound > hl.offset && upper_bound <= (hl.offset + hl.length) {
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

  pub fn from_json(template: Vec<u8>, data_set:Vec<u8>) -> Result<Dataset, errors::DataBrushErrors> {
      // read and parse template
    if template.is_empty() {
        return Err(errors::DataBrushErrors::EmptyTemplate)
    }

    let mut dataset: Dataset = serde_json::from_slice(&template).unwrap();

    // validate created dataset
    dataset.validate()?;
    dataset.assign_colors();

    // Add dataset
    if data_set.is_empty() {
        return Err(errors::DataBrushErrors::EmptyDataset)
    }

    if data_set.len() > MAX_FILE_LEN {
        return Err(errors::DataBrushErrors::FileTooLarge) 
    }

    Ok(dataset)
  }

  // TODO(carstein): Finish implementation
  fn validate(&self) -> Result<(), errors::DataBrushErrors> {

    Ok(())
  }

  // TODO(carstein): finish implementation
  fn assign_colors(&mut self) {
    for chunk in &self.chunks {
        for _hl in &chunk.highlights {
            // iterate over highlights and set colors
        }
    }
  }

  pub fn add_chunk(&mut self, mut new_chunk: Chunk) -> Result<(), errors::DataBrushErrors> {
      // Calculate offset
      let mut chunk_offset = 0;

      if let Some(chunk) = self.chunks.last() {
          chunk_offset = chunk.offset + chunk.len;
      }

      if chunk_offset + new_chunk.len > self.data.len() {
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
      let chunk_len = last.len + last.offset;

      self.chunks.push(Chunk {
          name: last_chunk_name, 
          offset: chunk_len,
          len: self.data.len() - chunk_len,
          highlights: vec!(),
      });
  }
}