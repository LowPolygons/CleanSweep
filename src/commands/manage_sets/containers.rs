use core::fmt;

#[derive(Debug, Clone)]
pub enum SetStyle {
    First,
    Last,
    FirstAndLast,
    FirstN(usize),
    LastN(usize),
    FirstNandLastM(usize, usize),
    EveryNIndexed(usize, ZeroOrOne),
    EvenlySpacedN(usize),
}
#[derive(Debug, Clone)]
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
    pub fn vec_style_to_string(styles: &Vec<SetStyle>) -> String {
        let mut per_set_list: String = String::from("(");
        styles.iter().for_each(|item| {
            per_set_list = format!("{} {:?} + ", per_set_list, item);
        });

        per_set_list = format!("{} )", per_set_list);

        per_set_list
    }
    pub fn styles_to_string(&self) -> String {
        let mut string = String::from("[");
        for styles in &self.chosen_styles {
            string = format!(
                "{} -> {:?}",
                string,
                ManageSetsType::vec_style_to_string(styles)
            )
        }
        string = format!("{}]", string);

        string
    }
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
    Preview,
}
pub enum NewStyleBehaviour {
    Append,
    Reset,
    Copy,
    Set,
}
