use std::{borrow::Cow, sync::Arc};

#[derive(Debug, Clone)]
pub struct CheckboxListStyle {
    pub active_icon: Cow<'static, str>,
    pub inactive_icon: Cow<'static, str>,
    pub empty_cell_icon: Cow<'static, str>,
}

impl Default for CheckboxListStyle {
    fn default() -> Self {
        Self {
            active_icon: Cow::Borrowed("☑"),
            inactive_icon: Cow::Borrowed("☐"),
            empty_cell_icon: Cow::Borrowed("✖️"),
        }
    }
}

impl CheckboxListStyle {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn builder() -> CheckboxListStyleBuilder {
        CheckboxListStyleBuilder::new()
    }
}

#[derive(Debug)]
pub struct CheckboxListStyleBuilder {
    pub active_icon: Cow<'static, str>,
    pub inactive_icon: Cow<'static, str>,
    pub empty_cell_icon: Cow<'static, str>,
}

impl Default for CheckboxListStyleBuilder {
    fn default() -> Self {
        Self {
            active_icon: Cow::Borrowed("☑"),
            inactive_icon: Cow::Borrowed("☐"),
            empty_cell_icon: Cow::Borrowed("✖️"),
        }
    }
}

impl CheckboxListStyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Arc<CheckboxListStyle> {
        Arc::new(CheckboxListStyle {
            active_icon: self.active_icon,
            inactive_icon: self.inactive_icon,
            empty_cell_icon: self.empty_cell_icon,
        })
    }

    pub fn active_icon(&mut self, value: Cow<'static, str>) -> &mut Self {
        self.active_icon = value;
        self
    }

    pub fn inactive_icon(&mut self, value: Cow<'static, str>) -> &mut Self {
        self.inactive_icon = value;
        self
    }

    pub fn empty_cell_icon(&mut self, value: Cow<'static, str>) -> &mut Self {
        self.empty_cell_icon = value;
        self
    }
}
