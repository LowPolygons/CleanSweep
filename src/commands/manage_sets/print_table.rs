use std::collections::HashMap;

pub struct PrintableTable {
    columns: Vec<Column>,
    num_rows: usize,
}

pub struct Column {
    pub width: usize,
    pub title: String,
    // Strings should be truncated/grown when printed to 'width'
    pub lines: Vec<String>,
}

impl PrintableTable {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            num_rows: 0,
        }
    }
    // pub fn get_titles(&self) -> Vec<String> {
    //     self.columns
    //         .iter()
    //         .fold(Vec::<String>::new(), |mut titles, column| {
    //             titles.push(column.title.clone());
    //
    //             titles
    //         })
    // }
    pub fn new_column(&mut self, column: Column) -> bool {
        if self.columns.len() != 0 {
            // Data has already been inserted and therfore the tale is in use
            if self.columns[0].lines.len() != 0 {
                return false;
            }
        }

        self.columns.push(column);

        true
    }
    pub fn insert_row(&mut self, lines: Vec<String>) -> bool {
        if lines.len() != self.columns.len() {
            return false;
        }

        for (index, value) in lines.into_iter().enumerate() {
            self.columns[index].lines.push(value.clone());
        }

        self.num_rows = self.num_rows + 1;

        true
    }

    pub fn get_printable_strings(&self) -> Vec<String> {
        let mut length_of_table: usize = self.columns.iter().map(|column| column.width + 1).sum();
        // Plus 1 for the last bar
        length_of_table = length_of_table + 1;

        let mut lines: Vec<String> = Vec::new();

        lines.push(char_to_string_of_len('-', length_of_table));

        // Titles
        let mut titles: String = String::from("|");

        self.columns.iter().for_each(|column| {
            titles = format!(
                "{}{}|",
                titles,
                truncate_or_stretch_string_to_length(column.title.clone(), column.width)
            )
        });

        lines.push(titles);

        lines.push(char_to_string_of_len('-', length_of_table));

        // Now each line
        for index in (0..self.num_rows).into_iter() {
            let mut current_row = String::from("|");

            self.columns.iter().for_each(|column| {
                current_row = format!(
                    "{}{}|",
                    current_row,
                    truncate_or_stretch_string_to_length(column.lines[index].clone(), column.width)
                )
            });

            lines.push(current_row)
        }

        lines.push(char_to_string_of_len('-', length_of_table));

        lines
    }
}
pub fn char_to_string_of_len(char: char, length: usize) -> String {
    let mut string_result = String::new();

    for _ in (0..length).into_iter() {
        string_result = format!("{}{}", string_result, char);
    }

    string_result
}

pub fn truncate_or_stretch_string_to_length(string: String, length: usize) -> String {
    let mut result = string;

    if result.len() == length {
        return result;
    }

    if result.len() > length {
        result = result
            .drain((result.len() - length)..result.len())
            .collect();

        return result;
    } else {
        result = format!(
            "{}{} ",
            char_to_string_of_len(' ', length - result.len() - 1),
            result
        );
        return result;
    }
}
