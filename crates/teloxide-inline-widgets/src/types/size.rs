use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Size {
    /// The number of rows in the inline keyboard markup of the widget
    pub rows: u8,
    /// The number of columns in the inline keyboard markup of the widget
    pub columns: u8,
}

impl Size {
    pub fn new(rows: u8, columns: u8) -> Self {
        Self { rows, columns }
    }
}
