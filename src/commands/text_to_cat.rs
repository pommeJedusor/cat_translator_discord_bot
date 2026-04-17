use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue};
use serenity::builder::CreateCommand;

use crate::cat_translator::text_to_cat;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let text_input = interaction.data.options().iter().filter(|x| x.name == "text").map(|x| match x.value{
        ResolvedValue::String(text) => Some(text),
        _ => None,
    }).next().unwrap_or(None);

    let is_ephemeral = interaction.data.options().iter().filter(|x| x.name == "is_ephemeral").map(|x| match x.value{
        ResolvedValue::Boolean(is_ephemeral) => Some(is_ephemeral),
        _ => None,
    }).next().unwrap_or(None).unwrap_or(false);


    let response_text = if let Some(text) = text_input
    {
         text_to_cat(text).unwrap_or_else(|x| format!("`{}`", x))
    } else {
        "`field text is required`".to_string()
    };

    let _ = interaction.create_response(
        ctx,
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
            response_text,
        ).ephemeral(is_ephemeral))).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("text_to_cat")
        .description("translate the text into cat noises")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "text", "text to turn into cat noises")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "is_ephemeral", "whether you're the only one to see the response or not")
                .required(false),
        )
}
