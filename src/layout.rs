use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use crate::traits::Component;

pub struct Layout<'a> {
    pub widgets: &'a [&'a dyn Component],
    pub orientation: LayoutOrientation,
}

impl<'a> Layout<'a> {
    pub fn new(widgets: &'a [&'a dyn Component], orientation: LayoutOrientation) -> Self {
        Self {
            widgets,
            orientation,
        }
    }

    pub fn size(&self) -> (u8, u8) {
        use LayoutOrientation::*;

        self.widgets
            .iter()
            .fold((0, 0), |required_size, widget| match self.orientation {
                Horizontal => (
                    required_size.0.max(widget.size().0),
                    required_size.1 + widget.size().1,
                ),
                Vertical => (
                    required_size.0 + widget.size().0,
                    required_size.1.max(widget.size().1),
                ),
            })
    }

    // FIXME: , empty_button_icon: &str + callback_data, noop ignore
    // fn create_empty_inline_keyboard((rows, columns): (u8, u8)) -> Vec<Vec<InlineKeyboardButton>> {
    //     std::iter::repeat(
    //         // FIXME: allow customize noop buttons
    //         std::iter::repeat(InlineKeyboardButton::callback("✖️", "noop"))
    //             .take(columns as usize)
    //             .collect(),
    //     )
    //     .take(rows as usize)
    //     .collect()
    // }
}

pub enum LayoutOrientation {
    Horizontal,
    Vertical,
}

impl<'a> From<Layout<'a>> for InlineKeyboardMarkup {
    fn from(value: Layout) -> Self {
        let (rows, columns) = value.size();

        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = std::iter::repeat(
            // FIXME: allow customize noop buttons
            std::iter::repeat(InlineKeyboardButton::callback("✖️", "noop"))
                .take(columns as usize)
                .collect(),
        )
        .take(rows as usize)
        .collect();

        let (mut curr_i, mut curr_j) = (0_u8, 0_u8);
        for widget in value.widgets {
            // let (size, keyboard) = (widget.size(),);

            for (row_i, row) in widget.keyboard().into_iter().enumerate() {
                for (col_i, button) in row.into_iter().enumerate() {
                    keyboard[curr_i as usize + row_i][curr_j as usize + col_i] = button;
                }
            }
            match value.orientation {
                LayoutOrientation::Horizontal => curr_j += widget.size().1,
                LayoutOrientation::Vertical => curr_i += widget.size().0,
            }
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl<'a> From<Layout<'a>> for ReplyMarkup {
    fn from(value: Layout) -> Self {
        ReplyMarkup::InlineKeyboard(value.into())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    struct DummyWidget {
        pub size: (u8, u8),
    }

    impl Component for DummyWidget {
        fn size(&self) -> (u8, u8) {
            self.size
        }

        fn keyboard(&self) -> Vec<Vec<InlineKeyboardButton>> {
            vec![]
        }
    }

    #[rstest]
    #[case((
        LayoutOrientation::Horizontal,
        vec![(2, 2), (2, 2)]
    ), (2, 4))]
    #[case((
        LayoutOrientation::Horizontal,
        vec![(1, 3), (4, 2)]
    ), (4, 5))]
    #[case((
        LayoutOrientation::Vertical,
        vec![(2, 2), (2, 2)]
    ), (4, 2))]
    fn orientation(
        #[case] init: (LayoutOrientation, Vec<(u8, u8)>),
        #[case] expected_size: (u8, u8),
    ) {
        let widgets: Vec<DummyWidget> = init
            .1
            .into_iter()
            .map(|size| DummyWidget { size })
            .collect();
        let widgets: &[&dyn Component] = &widgets
            .iter()
            .map(|v| v as &dyn Component)
            .collect::<Vec<_>>();

        let layout = Layout::new(widgets, init.0);

        assert_eq!(expected_size, layout.size());
    }
}
