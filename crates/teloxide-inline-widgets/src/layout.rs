use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

/// Allows to combine widgets either `horizontally` or `vertically`
///
/// The actual size of the whole [`Layout`] is determined by the size of
/// provided [`InlineKeyboardMarkup`]s
pub struct Layout {
    pub markups: Vec<InlineKeyboardMarkup>,
    pub orientation: LayoutOrientation,
}

impl Layout {
    /// Creates a new layout with widgets' inline keyboard markups
    pub fn new(markups: Vec<InlineKeyboardMarkup>, orientation: LayoutOrientation) -> Self {
        Self { markups, orientation }
    }

    /// Returns the size of the [`Layout`]
    pub fn size(&self) -> (u8, u8) {
        use LayoutOrientation::*;

        self.markups.iter().fold((0, 0), |required_size, markup| {
            let (rows, columns) = Self::markup_size(markup);
            match self.orientation {
                Horizontal => (required_size.0.max(rows), required_size.1 + columns),
                Vertical => (required_size.0 + rows, required_size.1.max(columns)),
            }
        })
    }

    fn markup_size(markup: &InlineKeyboardMarkup) -> (u8, u8) {
        // Not as accurate as I wanted, but..it works?
        (markup.inline_keyboard.len() as u8, markup.inline_keyboard[0].len() as u8)
    }

    /// Creates an empty inline keyboard markup with specified number of rows
    /// and columns
    fn empty_inline_keyboard_markup((rows, columns): (u8, u8)) -> InlineKeyboardMarkup {
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
        let (rows, columns) = layout.size();

        let mut keyboard: Vec<Vec<InlineKeyboardButton>> =
            Layout::empty_inline_keyboard_markup((rows, columns)).inline_keyboard;

        let (mut curr_i, mut curr_j) = (0_u8, 0_u8);
        for markup in layout.markups {
            let size = Layout::markup_size(&markup);

            for (row_i, row) in markup.inline_keyboard.into_iter().enumerate() {
                for (col_i, button) in row.into_iter().enumerate() {
                    keyboard[curr_i as usize + row_i][curr_j as usize + col_i] = button;
                }
            }
            match layout.orientation {
                LayoutOrientation::Horizontal => curr_j += size.1,
                LayoutOrientation::Vertical => curr_i += size.0,
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
        vec![(2, 2), (2, 2)],
        LayoutOrientation::Horizontal
    ), (2, 4))]
    #[case((
        vec![(1, 3), (4, 2)],
        LayoutOrientation::Horizontal,
    ), (4, 5))]
    #[case((
        vec![(2, 2), (2, 2)],
        LayoutOrientation::Vertical
    ), (4, 2))]
    // TODO more tests
    fn layout(#[case] init: (Vec<(u8, u8)>, LayoutOrientation), #[case] expected_size: (u8, u8)) {
        let markups = init
            .0
            .into_iter()
            .map(|size| Layout::empty_inline_keyboard_markup(size))
            .collect::<Vec<_>>();

        let layout = Layout::new(markups, init.1);

        assert_eq!(expected_size, layout.size());
    }
}
