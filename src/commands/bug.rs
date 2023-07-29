use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("bug")
        .description("Annoys somebody and tells them to be on time")
        .create_option(|o| {
            o.name("somebody")
                .description("That somebody")
                .kind(CommandOptionType::User)
        })
        .create_option(|o| {
            o.name("time")
                .description("What time they were supposed to show up")
                .kind(CommandOptionType::String)
        })
}