use dialoguer::{Input, theme::ColorfulTheme};

pub fn get_number_input(label: &str, first_time_calling: bool) -> Result<usize, String> {
    let colour = ColorfulTheme::default();

    let theme = if first_time_calling {
        Input::with_theme(&colour).with_prompt(label)
    } else {
        Input::with_theme(&colour)
    };

    let number: usize = theme
        .validate_with(|input: &String| -> Result<(), &str> {
            input
                .parse::<usize>()
                .map(|_| ()) // validate_with needs to return nothing
                .map_err(|_| "Please enter a valid number")
        })
        .interact_text()
        .map_err(|e| format!("Failed to validate numerical input, {:?}", e))?
        .parse()
        .map_err(|e| format!("Error formatting the parsed number, {:?}", e))?;

    Ok(number)
}

pub fn get_number_input_in_range(label: &str, lower: usize, upper: usize) -> Result<usize, String> {
    let mut number: usize = get_number_input(label, true).map_err(|e| format!("{e}"))?;

    while number < lower || number > upper {
        println!(
            "Please enter a number in the correct range ({}-{})",
            lower, upper
        );
        number = get_number_input(label, false).map_err(|e| format!("{e}"))?;
    }

    Ok(number)
}

pub fn get_string_input_matching_provided_string(
    label: &str,
    match_against: &str,
) -> Result<bool, String> {
    let colour = ColorfulTheme::default();

    let input: String = Input::with_theme(&colour)
        .with_prompt(label)
        .interact_text()
        .map_err(|_| format!("Failed to get text input"))?;

    Ok(input == match_against)
}
