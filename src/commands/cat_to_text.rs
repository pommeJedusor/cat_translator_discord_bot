use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue};
use serenity::builder::CreateCommand;

use crate::cat_translator::{cat_noises_to_text};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let text_input = interaction.data.options().iter().filter(|x| x.name == "cat-noises").map(|x| match x.value{
        ResolvedValue::String(text) => Some(text),
        _ => None,
    }).next().unwrap_or(None);

    let is_ephemeral = interaction.data.options().iter().filter(|x| x.name == "is_ephemeral").map(|x| match x.value{
        ResolvedValue::Boolean(is_ephemeral) => Some(is_ephemeral),
        _ => None,
    }).next().unwrap_or(None).unwrap_or(false);


    let response_text = if let Some(text) = text_input
    {
         cat_noises_to_text(text)
    } else {
        "`field cat-noises is required`".to_string()
    };

    let _ = interaction.create_response(
        ctx,
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
            response_text,
        ).ephemeral(is_ephemeral))).await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cat_to_text")
        .description("translate the cat noises into text")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "cat-noises", "cat noises to turn back into text")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "is_ephemeral", "whether you're the only one to see the response or not")
                .required(false),
        )
}
