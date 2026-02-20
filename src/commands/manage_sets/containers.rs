use core::fmt;

#[derive(Debug, Clone)]
pub enum SetStyle {
    First,
    Last,
    FirstAndLast,
    EveryN(usize),
    EvenlySpacedN(usize),
}

impl fmt::Display for SetStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SetStyle::First => "First",
            SetStyle::Last => "Last",
            SetStyle::FirstAndLast => "First And Last",
            SetStyle::EveryN(_) => "Every N",
            SetStyle::EvenlySpacedN(_) => "N Evenly Spaced",
        };
        write!(f, "{s}")
    }
}

pub struct ManageSetsType {
    pub full_set: Vec<String>,
    pub label: String,
    pub chosen_style: Option<SetStyle>,
}

impl ManageSetsType {
    pub fn style_to_string(&self) -> String {
        return match &self.chosen_style {
            Some(v) => format!("{}", v),
            None => format!("None Chosen"),
        };
    }
    pub fn label_truncated(&self, length_to_strip: usize) -> String {
        let length = self.label.len();

        let mut clone_of_label = self.label.clone();

        clone_of_label.drain(length_to_strip..length).collect()
    }
}
