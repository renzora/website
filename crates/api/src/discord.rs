//! Discord bot integration for role management.
//!
//! Requires environment variables:
//! - DISCORD_BOT_TOKEN: Bot token with "Manage Roles" permission
//! - DISCORD_GUILD_ID: The Renzora Discord server ID
//! - DISCORD_ROLE_SUPPORTER: Role ID for Supporter subscribers


/// Discord role IDs loaded from env.
struct DiscordConfig {
    bot_token: String,
    guild_id: String,
    role_supporter: String,
}

impl DiscordConfig {
    fn load() -> Option<Self> {
        Some(Self {
            bot_token: std::env::var("DISCORD_BOT_TOKEN").ok()?,
            guild_id: std::env::var("DISCORD_GUILD_ID").ok()?,
            role_supporter: std::env::var("DISCORD_ROLE_SUPPORTER").ok()?,
        })
    }

    fn all_role_ids(&self) -> Vec<&str> {
        vec![&self.role_supporter]
    }
}

/// Remove all subscription roles from a Discord user.
async fn remove_all_roles(config: &DiscordConfig, discord_user_id: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let base = format!(
        "https://discord.com/api/v10/guilds/{}/members/{}/roles",
        config.guild_id, discord_user_id
    );

    for role_id in config.all_role_ids() {
        let _ = client
            .delete(&format!("{}/{}", base, role_id))
            .header("Authorization", format!("Bot {}", config.bot_token))
            .send()
            .await;
    }

    Ok(())
}

/// Called when a user unlinks their Discord account. Clears any tier role the
/// account still carries from the retired subscription system.
pub async fn on_discord_unlink(discord_user_id: &str) {
    let Some(config) = DiscordConfig::load() else { return };

    if let Err(e) = remove_all_roles(&config, discord_user_id).await {
        tracing::warn!("Failed to remove Discord roles on unlink: {}", e);
    }
}
