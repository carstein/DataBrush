use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataBrushErrors {
    #[error("dataset is empty")]
    EmptyDataset,
    #[error("json template is empty")]
    EmptyTemplate,
    #[error("file too large")]
    FileTooLarge,
    #[error("chunk <{0}> size is zero")]
    ChunkSizeZero(String),
    #[error("highlight <{0}> size is zero")]
    HighlightSizeZero(String),
    #[error("chunk larger than datasize")]
    ChunkOverflow,
    #[error("highlighted regions overlap")]
    HighlightOverlap
}