use std::{borrow::Cow, sync::Arc};

/// Style that is shared among all widgets
#[derive(Debug, Clone)]
pub struct CommonStyle {
    pub empty_cell_icon: Cow<'static, str>,
}

impl Default for CommonStyle {
    fn default() -> Self {
        Self { empty_cell_icon: Cow::Borrowed("✖️") }
    }
}

impl CommonStyle {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn builder() -> CommonStyleBuilder {
        CommonStyleBuilder::new()
    }
}

#[derive(Debug)]
pub struct CommonStyleBuilder {
    pub empty_cell_icon: Cow<'static, str>,
}

impl Default for CommonStyleBuilder {
    fn default() -> Self {
        Self { empty_cell_icon: Cow::Borrowed("✖️") }
    }
}

impl CommonStyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Arc<CommonStyle> {
        Arc::new(CommonStyle { empty_cell_icon: self.empty_cell_icon })
    }

    pub fn empty_cell_icon(mut self, value: Cow<'static, str>) -> Self {
        self.empty_cell_icon = value;
        self
    }
}
