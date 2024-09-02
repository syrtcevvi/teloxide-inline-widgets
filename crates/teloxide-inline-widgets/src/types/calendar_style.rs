use std::{borrow::Cow, sync::Arc};

/// [`Calendar`] widget style
#[derive(Debug, Clone)]
pub struct CalendarStyle {
    /// Icon for `choose previous month` button
    pub previous_month_icon: Cow<'static, str>,
    /// Icon for `choose next month` button
    pub next_month_icon: Cow<'static, str>,
    /// Icon for `choose previous year` button
    pub previous_year_icon: Cow<'static, str>,
    /// Icon for `choose next year` button
    pub next_year_icon: Cow<'static, str>,
    /// Names of the days of the week
    pub days_of_the_week: [Cow<'static, str>; 7],
    /// Names of months
    pub months: [Cow<'static, str>; 12],
}

impl Default for CalendarStyle {
    fn default() -> Self {
        Self {
            previous_month_icon: Cow::Borrowed("◀️"),
            next_month_icon: Cow::Borrowed("▶️"),
            previous_year_icon: Cow::Borrowed("◀️"),
            next_year_icon: Cow::Borrowed("▶️"),
            days_of_the_week: [
                Cow::Borrowed("Mon"),
                Cow::Borrowed("Tue"),
                Cow::Borrowed("Wed"),
                Cow::Borrowed("Thu"),
                Cow::Borrowed("Fri"),
                Cow::Borrowed("Sat"),
                Cow::Borrowed("Sun"),
            ],
            months: [
                Cow::Borrowed("January"),
                Cow::Borrowed("February"),
                Cow::Borrowed("March"),
                Cow::Borrowed("April"),
                Cow::Borrowed("May"),
                Cow::Borrowed("June"),
                Cow::Borrowed("July"),
                Cow::Borrowed("August"),
                Cow::Borrowed("September"),
                Cow::Borrowed("October"),
                Cow::Borrowed("November"),
                Cow::Borrowed("December"),
            ],
        }
    }
}

impl CalendarStyle {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn builder() -> CalendarStyleBuilder {
        CalendarStyleBuilder::new()
    }
}

#[derive(Debug)]
pub struct CalendarStyleBuilder {
    /// Icon for `choose previous month` button
    pub previous_month_icon: Cow<'static, str>,
    /// Icon for `choose next month` button
    pub next_month_icon: Cow<'static, str>,
    /// Icon for `choose previous year` button
    pub previous_year_icon: Cow<'static, str>,
    /// Icon for `choose next year` button
    pub next_year_icon: Cow<'static, str>,
    /// Names of the days of the week
    pub days_of_the_week: [Cow<'static, str>; 7],
    /// Names of months
    pub months: [Cow<'static, str>; 12],
}

impl Default for CalendarStyleBuilder {
    fn default() -> Self {
        Self {
            previous_month_icon: Cow::Borrowed("◀️"),
            next_month_icon: Cow::Borrowed("▶️"),
            previous_year_icon: Cow::Borrowed("◀️"),
            next_year_icon: Cow::Borrowed("▶️"),
            days_of_the_week: [
                Cow::Borrowed("Mon"),
                Cow::Borrowed("Tue"),
                Cow::Borrowed("Wed"),
                Cow::Borrowed("Thu"),
                Cow::Borrowed("Fri"),
                Cow::Borrowed("Sat"),
                Cow::Borrowed("Sun"),
            ],
            months: [
                Cow::Borrowed("January"),
                Cow::Borrowed("February"),
                Cow::Borrowed("March"),
                Cow::Borrowed("April"),
                Cow::Borrowed("May"),
                Cow::Borrowed("June"),
                Cow::Borrowed("July"),
                Cow::Borrowed("August"),
                Cow::Borrowed("September"),
                Cow::Borrowed("October"),
                Cow::Borrowed("November"),
                Cow::Borrowed("December"),
            ],
        }
    }
}

impl CalendarStyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Arc<CalendarStyle> {
        Arc::new(CalendarStyle {
            previous_month_icon: self.previous_month_icon,
            next_month_icon: self.next_month_icon,
            previous_year_icon: self.previous_year_icon,
            next_year_icon: self.next_year_icon,
            days_of_the_week: self.days_of_the_week,
            months: self.months,
        })
    }

    pub fn previous_month_icon(mut self, value: Cow<'static, str>) -> Self {
        self.previous_month_icon = value;
        self
    }

    pub fn next_month_icon(mut self, value: Cow<'static, str>) -> Self {
        self.next_month_icon = value;
        self
    }

    pub fn previous_year_icon(mut self, value: Cow<'static, str>) -> Self {
        self.previous_year_icon = value;
        self
    }

    pub fn next_year_icon(mut self, value: Cow<'static, str>) -> Self {
        self.next_year_icon = value;
        self
    }

    pub fn days_of_the_week(mut self, value: [Cow<'static, str>; 7]) -> Self {
        self.days_of_the_week = value;
        self
    }

    pub fn months(mut self, value: [Cow<'static, str>; 12]) -> Self {
        self.months = value;
        self
    }
}
