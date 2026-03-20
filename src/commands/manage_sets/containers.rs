use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetStyle {
    First,
    Last,
    FirstAndLast,
    FirstN(usize),
    LastN(usize),
    FirstNandLastM(usize, usize),
    EveryNIndexed(usize, ZeroOrOne),
    EvenlySpacedN(usize),
    IDisDivisibleByN(usize),
    NumberDivisibleByN(f64),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZeroOrOne {
    Zero,
    One,
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
            SetStyle::EveryNIndexed(_, zero_or_one) => {
                let addition: String = match zero_or_one {
                    ZeroOrOne::Zero => String::from("0"),
                    ZeroOrOne::One => String::from("1"),
                };

                &format!("Position in set is divisible by N ({addition} Indexed)")
            }
            SetStyle::EvenlySpacedN(_) => "N Evenly Spaced",
            SetStyle::IDisDivisibleByN(_) => "ID is divisible by N",
            SetStyle::NumberDivisibleByN(_) => "Number is divisible by N",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone)]
pub struct ManageSetsType {
    pub full_set: Vec<String>,
    pub label: String,
    pub chosen_styles: Vec<Vec<SetStyle>>,
}

impl ManageSetsType {
    pub fn label_truncated(&self, length_to_strip: usize) -> String {
        let length = self.label.len();

        let mut clone_of_label = self.label.clone();

        clone_of_label.drain(length_to_strip..length).collect()
    }
}

// INFO:
// These enums are here to make return types of the helper methods more clear
pub enum AppendOrOverride {
    Append,
    Override,
}
pub enum ChoiceInGettingStyle {
    AffectStoredStyles(NewStyleBehaviour),
    NotAffectingStyles(NotAffectingStyles),
}
pub enum NotAffectingStyles {
    Back,
    FullTable,
    Preview,
}
pub enum NewStyleBehaviour {
    Append,
    Reset,
    Copy,
    Set,
}
