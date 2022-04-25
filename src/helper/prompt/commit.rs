use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn commit_prompt() -> String {
    // Q1: Type of changes
    let types = &[
        "feat: A new feature",
        "fix: A bug fix",
        "docs: Documentation only changes",
        "style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)",
        "refactor: A code change that neither fixes a bug nor adds a feature",
        "perf: A code change that improves performance",
        "test: Adding missing or correcting existing tests",
        "chore: Changes to the build process or auxiliary tools and libraries such as documentation generation",
    ];

    let selected_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Type of changes?:")
        .default(0)
        .items(&types[..])
        .interact()
        .unwrap();

    // Q2: Scope of changes
    let scope: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Scope of changes (eg. file, function, etc)?:")
        // TODO: length validation
        .interact_text()
        .unwrap();

    // Q3: Commit message
    let message: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit message?:")
        // TODO: length validation
        .interact_text()
        .unwrap();

    let type_split: Vec<&str> = types[selected_type].split(": ").collect();

    format!("{}({}): {}", type_split[0], scope, message)
}
