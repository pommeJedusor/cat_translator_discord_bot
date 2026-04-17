use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::cat_translator::text_to_cat;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(str), ..
    }) = options.first()
    {
        text_to_cat(*str).unwrap_or_else(|x| format!("`{}`", x))
    } else {
        "field text is required".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("text_to_cat")
        .description("translate the text into cat noises")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "text", "text to turn into cat noises")
                .required(true),
        )
}
