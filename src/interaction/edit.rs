use anyhow::Ok as _;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{
        component::{select_menu::SelectMenuOption, ActionRow, Component, SelectMenu},
        interaction::ApplicationCommand,
    },
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{interaction::Error, Context};

#[derive(CreateCommand, CommandModel)]
#[command(name = "edit", desc = "edit any message you select")]
pub struct Edit {}

impl Context {
    pub async fn handle_edit_command(
        &self,
        command: &ApplicationCommand,
    ) -> Result<InteractionResponse, Error> {
        // todo: check for manage messages permissions for the member
        let mut message_options: Vec<SelectMenuOption> = Vec::with_capacity(25);
        for id in self
            .cache
            .channel_messages(command.channel_id)
            .ok_or(Error::NoCachedMessages)?
        {
            let message = self.cache.message(id).ok()?;
            let content = message.content();
            message_options.push(SelectMenuOption {
                // todo: check for valid labels
                label: content.get(0..100).unwrap_or(content).to_owned(),
                value: id.to_string(),
                default: false,
                description: None,
                emoji: None,
            });
        }

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content("please select the message you want to edit".to_owned())
                    .components([Component::ActionRow(ActionRow {
                        components: vec![Component::SelectMenu(SelectMenu {
                            custom_id: "message".to_owned(),
                            options: message_options,
                            placeholder: Some("message to edit".to_owned()),
                            disabled: false,
                            max_values: None,
                            min_values: None,
                        })],
                    })])
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
        })
    }
}
