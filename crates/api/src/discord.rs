//! Discord bot integration for role management.
//!
//! Requires environment variables:
//! - DISCORD_BOT_TOKEN: Bot token with "Manage Roles" permission
//! - DISCORD_GUILD_ID: The Renzora Discord server ID
//! - DISCORD_ROLE_PRO: Role ID for Pro subscribers
//! - DISCORD_ROLE_INDIE: Role ID for Indie subscribers
//! - DISCORD_ROLE_STUDIO: Role ID for Studio subscribers

use sqlx::PgPool;
use uuid::Uuid;

/// Discord role IDs loaded from env.
struct DiscordConfig {
    bot_token: String,
    guild_id: String,
    role_pro: String,
    role_indie: String,
    role_studio: String,
}

impl DiscordConfig {
    fn load() -> Option<Self> {
        Some(Self {
            bot_token: std::env::var("DISCORD_BOT_TOKEN").ok()?,
            guild_id: std::env::var("DISCORD_GUILD_ID").ok()?,
            role_pro: std::env::var("DISCORD_ROLE_PRO").ok()?,
            role_indie: std::env::var("DISCORD_ROLE_INDIE").ok()?,
            role_studio: std::env::var("DISCORD_ROLE_STUDIO").ok()?,
        })
    }

    fn role_id_for_plan(&self, plan_id: &str) -> Option<&str> {
        match plan_id {
            "pro" => Some(&self.role_pro),
            "indie" => Some(&self.role_indie),
            "studio" => Some(&self.role_studio),
            _ => None,
        }
    }

    fn all_role_ids(&self) -> Vec<&str> {
        vec![&self.role_pro, &self.role_indie, &self.role_studio]
    }
}

/// Add the subscription role to a user's Discord account.
/// Removes any other subscription roles first (user can only have one tier).
async fn set_discord_role(config: &DiscordConfig, discord_user_id: &str, plan_id: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let base = format!(
        "https://discord.com/api/v10/guilds/{}/members/{}/roles",
        config.guild_id, discord_user_id
    );

    // Remove all subscription roles first
    for role_id in config.all_role_ids() {
        let _ = client
            .delete(&format!("{}/{}", base, role_id))
            .header("Authorization", format!("Bot {}", config.bot_token))
            .send()
            .await;
    }

    // Add the new role
    if let Some(role_id) = config.role_id_for_plan(plan_id) {
        let resp = client
            .put(&format!("{}/{}", base, role_id))
            .header("Authorization", format!("Bot {}", config.bot_token))
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|e| format!("Discord API error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Discord role add failed: {} {}", status, body));
        }
    }

    Ok(())
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

/// Called when a user subscribes or changes plan.
/// Looks up their Discord ID and assigns the appropriate role.
pub async fn on_subscription_change(db: &PgPool, user_id: Uuid, plan_id: &str) {
    let Some(config) = DiscordConfig::load() else { return };

    // Get user's Discord ID
    let discord_id: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT discord_id FROM users WHERE id = $1"
    ).bind(user_id).fetch_optional(db).await.ok().flatten();

    let Some((Some(discord_user_id),)) = discord_id else { return };

    if let Err(e) = set_discord_role(&config, &discord_user_id, plan_id).await {
        tracing::warn!("Failed to set Discord role for user {}: {}", user_id, e);
    }
}

/// Called when a subscription is canceled or expires.
/// Removes all subscription roles from the user's Discord account.
pub async fn on_subscription_end(db: &PgPool, user_id: Uuid) {
    let Some(config) = DiscordConfig::load() else { return };

    let discord_id: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT discord_id FROM users WHERE id = $1"
    ).bind(user_id).fetch_optional(db).await.ok().flatten();

    let Some((Some(discord_user_id),)) = discord_id else { return };

    if let Err(e) = remove_all_roles(&config, &discord_user_id).await {
        tracing::warn!("Failed to remove Discord roles for user {}: {}", user_id, e);
    }
}

/// Called when a user links their Discord account.
/// If they have an active subscription, assign the appropriate role.
pub async fn on_discord_link(db: &PgPool, user_id: Uuid, discord_user_id: &str) {
    let Some(config) = DiscordConfig::load() else { return };

    // Check if user has an active subscription
    let sub: Option<(String,)> = sqlx::query_as(
        "SELECT plan_id FROM subscriptions WHERE user_id = $1 AND status = 'active' AND current_period_end > NOW()"
    ).bind(user_id).fetch_optional(db).await.ok().flatten();

    if let Some((plan_id,)) = sub {
        if let Err(e) = set_discord_role(&config, discord_user_id, &plan_id).await {
            tracing::warn!("Failed to set Discord role on link for user {}: {}", user_id, e);
        }
    }
}

/// Called when a user unlinks their Discord account.
/// Remove all subscription roles.
pub async fn on_discord_unlink(discord_user_id: &str) {
    let Some(config) = DiscordConfig::load() else { return };

    if let Err(e) = remove_all_roles(&config, discord_user_id).await {
        tracing::warn!("Failed to remove Discord roles on unlink: {}", e);
    }
}
