use core::fmt;

#[derive(Debug, Clone)]
pub enum SetStyle {
    First,
    Last,
    FirstAndLast,
    FirstN(usize),
    LastN(usize),
    FirstNandLastM(usize, usize),
    EveryN(usize),
    EvenlySpacedN(usize),
}

pub enum AppendOrOverride {
    Append,
    Override,
}

pub enum ChoiceInGettingStyle {
    Append,
    Reset,
    Copy,
    Set,
}

impl fmt::Display for SetStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SetStyle::First => "First",
            SetStyle::Last => "Last",
            SetStyle::FirstAndLast => "First And Last",
            SetStyle::FirstN(_) => "First N",
            SetStyle::LastN(_) => "Last N",
            SetStyle::FirstNandLastM(_, _) => "First N and Last M",
            SetStyle::EveryN(_) => "Every N",
            SetStyle::EvenlySpacedN(_) => "N Evenly Spaced",
        };
        write!(f, "{s}")
    }
}

pub struct ManageSetsType {
    pub full_set: Vec<String>,
    pub label: String,
    pub chosen_styles: Vec<SetStyle>,
}

impl ManageSetsType {
    pub fn styles_to_string(&self) -> String {
        let mut string = String::from("[");
        for style in &self.chosen_styles {
            string = format!("{} -> {:?}", string, style)
        }
        string = format!("{} ]", string);

        string
    }
    pub fn label_truncated(&self, length_to_strip: usize) -> String {
        let length = self.label.len();

        let mut clone_of_label = self.label.clone();

        clone_of_label.drain(length_to_strip..length).collect()
    }
}
