use std::{borrow::Cow, sync::Arc};

/// [`RadioList`] widget style
#[derive(Debug, Clone)]
pub struct RadioListStyle {
    /// Icon of selected item
    pub active_icon: Cow<'static, str>,
    /// Icon of unselected item
    pub inactive_icon: Cow<'static, str>,
}

impl Default for RadioListStyle {
    fn default() -> Self {
        Self { active_icon: Cow::Borrowed("🟢"), inactive_icon: Cow::Borrowed("") }
    }
}

impl RadioListStyle {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn builder() -> RadioListStyleBuilder {
        RadioListStyleBuilder::new()
    }
}

#[derive(Debug)]
pub struct RadioListStyleBuilder {
    pub active_icon: Cow<'static, str>,
    pub inactive_icon: Cow<'static, str>,
}

impl Default for RadioListStyleBuilder {
    fn default() -> Self {
        Self { active_icon: Cow::Borrowed("🟢"), inactive_icon: Cow::Borrowed("") }
    }
}

impl RadioListStyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Arc<RadioListStyle> {
        Arc::new(RadioListStyle {
            active_icon: self.active_icon,
            inactive_icon: self.inactive_icon,
        })
    }

    pub fn active_icon(mut self, value: Cow<'static, str>) -> Self {
        self.active_icon = value;
        self
    }

    pub fn inactive_icon(mut self, value: Cow<'static, str>) -> Self {
        self.inactive_icon = value;
        self
    }
}
