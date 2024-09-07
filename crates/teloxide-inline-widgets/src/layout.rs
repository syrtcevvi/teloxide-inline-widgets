use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use crate::types::Size;

/// Allows to combine widgets either `horizontally` or `vertically`
///
/// The actual size of the whole [`Layout`] is determined by the size of
/// provided [`InlineKeyboardMarkup`]s
pub struct Layout {
    pub markups: Vec<(InlineKeyboardMarkup, Size)>,
    pub orientation: LayoutOrientation,
}

impl Layout {
    /// Creates a new layout with widgets' inline keyboard markups
    pub fn new(markups: Vec<(InlineKeyboardMarkup, Size)>, orientation: LayoutOrientation) -> Self {
        Self { markups, orientation }
    }

    /// Returns the size of the [`Layout`]
    pub fn size(&self) -> Size {
        use LayoutOrientation::*;

        let (rows, columns) = self.markups.iter().fold((0, 0), |required_size, (_markup, size)| {
            let Size { rows, columns } = size;
            match self.orientation {
                Horizontal => (required_size.0.max(*rows), required_size.1 + columns),
                Vertical => (required_size.0 + rows, required_size.1.max(*columns)),
            }
        });
        Size { rows, columns }
    }

    /// Creates an empty inline keyboard markup with specified number of rows
    /// and columns
    fn empty_inline_keyboard_markup(Size { rows, columns }: Size) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(
            std::iter::repeat(
                // FIXME: allow customize noop buttons
                std::iter::repeat(InlineKeyboardButton::callback("✖️", "noop"))
                    .take(columns as usize)
                    .collect(),
            )
            .take(rows as usize)
            .collect::<Vec<Vec<_>>>(),
        )
    }
}

/// Represents the orientation of a layout
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
}

impl From<Layout> for InlineKeyboardMarkup {
    fn from(layout: Layout) -> Self {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> =
            Layout::empty_inline_keyboard_markup(layout.size()).inline_keyboard;

        let (mut curr_row, mut curr_column) = (0_u8, 0_u8);
        for (markup, size) in layout.markups {
            for (row_i, row) in markup.inline_keyboard.into_iter().enumerate() {
                for (col_i, button) in row.into_iter().enumerate() {
                    keyboard[curr_row as usize + row_i][curr_column as usize + col_i] = button;
                }
            }
            match layout.orientation {
                LayoutOrientation::Horizontal => curr_column += size.columns,
                LayoutOrientation::Vertical => curr_row += size.rows,
            }
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl From<Layout> for ReplyMarkup {
    fn from(value: Layout) -> Self {
        ReplyMarkup::InlineKeyboard(value.into())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case((
        vec![Size::new(2,2), Size::new(2, 2)],
        LayoutOrientation::Horizontal
    ), Size::new(2, 4))]
    #[case((
        vec![Size::new(1,3), Size::new(4, 2)],
        LayoutOrientation::Horizontal,
    ), Size::new(4, 5))]
    #[case((
        vec![Size::new(2, 2), Size::new(2, 2)],
        LayoutOrientation::Vertical
    ), Size::new(4, 2))]
    // TODO more tests
    fn layout(#[case] init: (Vec<Size>, LayoutOrientation), #[case] expected_size: Size) {
        let markups = init
            .0
            .into_iter()
            .map(|size| (Layout::empty_inline_keyboard_markup(size), size))
            .collect::<Vec<_>>();

        let layout = Layout::new(markups, init.1);

        assert_eq!(expected_size, layout.size());
    }
}
