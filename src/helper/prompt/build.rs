use dialoguer::{theme::ColorfulTheme, Select};

pub fn select(question: &String, options: &std::vec::Vec<std::string::String>) -> String {
    let selected_option = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    return format!("{}", options[selected_option]);
}