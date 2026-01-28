use crate::cli::ListAndResetArgs;

pub fn list(args: &ListAndResetArgs) -> Result<(), String> {
    let print_str = match args {
        ListAndResetArgs::ToDelete => "to_delete",
        ListAndResetArgs::ToKeep => "to_keep",
        ListAndResetArgs::Sets => "sets",
    };

    println!("Hello from List {}", print_str);

    Ok(())
}
