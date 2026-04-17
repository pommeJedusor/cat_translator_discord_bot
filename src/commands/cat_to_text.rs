use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::cat_translator::{cat_noises_to_text};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(str), ..
    }) = options.first()
    {
        cat_noises_to_text(*str)
    } else {
        "field cat-noises is required".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cat_to_text")
        .description("translate the cat noises into text")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "cat-noises", "cat noises to turn back into text")
                .required(true),
        )
}
