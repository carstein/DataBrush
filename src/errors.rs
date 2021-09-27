
#[derive(Debug)]
pub enum DataBrushErrors {
    EmptyDataset,
    FileTooLarge,
    ChunkSizeZero,
    HighlightSizeZero,
    ChunkOverflow,
    HighlightOverlap
}