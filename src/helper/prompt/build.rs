use dialoguer::{theme::ColorfulTheme, Select, MultiSelect, Input};

pub fn select(question: &String, options: &std::vec::Vec<std::string::String>) -> String {
    let selected_option = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    return format!("{}", options[selected_option].trim());
}

pub fn multi_select(question: &String, options: &std::vec::Vec<std::string::String>) -> std::vec::Vec<usize> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .items(&options[..])
        .interact()
        .unwrap();
    
    return selections;
}

pub fn input(question: &String) -> String {
    let scope: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .interact_text()
        .unwrap();
    return scope
}