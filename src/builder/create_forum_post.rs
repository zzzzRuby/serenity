#[cfg(feature = "http")]
use super::Builder;
use super::CreateMessage;
#[cfg(feature = "http")]
use crate::http::CacheHttp;
#[cfg(feature = "http")]
use crate::internal::prelude::*;
use crate::model::prelude::*;

/// [Discord docs](https://discord.com/developers/docs/resources/channel#start-thread-in-forum-channel).
#[derive(Clone, Debug, Serialize)]
#[must_use]
pub struct CreateForumPost<'a> {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
    message: CreateMessage,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    applied_tags: Vec<ForumTagId>,

    #[serde(skip)]
    audit_log_reason: Option<&'a str>,
}

impl<'a> CreateForumPost<'a> {
    /// Creates a builder with the given name and message content, leaving all other fields empty.
    pub fn new(name: impl Into<String>, message: CreateMessage) -> Self {
        Self {
            name: name.into(),
            message,
            auto_archive_duration: None,
            rate_limit_per_user: None,
            applied_tags: Vec::new(),
            audit_log_reason: None,
        }
    }

    /// The name of the forum post. Replaces the current value as set in [`Self::new`].
    ///
    /// **Note**: Must be between 2 and 100 characters long.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// The contents of the first message in the forum post.
    ///
    /// See [`CreateMessage`] for restrictions around message size.
    pub fn message(mut self, message: CreateMessage) -> Self {
        self.message = message;
        self
    }

    /// Duration in minutes to automatically archive the forum post after recent activity.
    pub fn auto_archive_duration(mut self, duration: AutoArchiveDuration) -> Self {
        self.auto_archive_duration = Some(duration);
        self
    }

    /// How many seconds must a user wait before sending another message.
    ///
    /// Bots, or users with the [`MANAGE_MESSAGES`] and/or [`MANAGE_CHANNELS`] permissions are
    /// exempt from this restriction.
    ///
    /// **Note**: Must be between 0 and 21600 seconds (360 minutes or 6 hours).
    ///
    /// [`MANAGE_MESSAGES`]: crate::model::permissions::Permissions::MANAGE_MESSAGES
    /// [`MANAGE_CHANNELS`]: crate::model::permissions::Permissions::MANAGE_CHANNELS
    #[doc(alias = "slowmode")]
    pub fn rate_limit_per_user(mut self, seconds: u16) -> Self {
        self.rate_limit_per_user = Some(seconds);
        self
    }

    pub fn add_applied_tag(mut self, applied_tag: ForumTagId) -> Self {
        self.applied_tags.push(applied_tag);
        self
    }

    pub fn set_applied_tags(
        mut self,
        applied_tags: impl IntoIterator<Item = impl Into<ForumTagId>>,
    ) -> Self {
        self.applied_tags = applied_tags.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the request's audit log reason.
    pub fn audit_log_reason(mut self, reason: &'a str) -> Self {
        self.audit_log_reason = Some(reason);
        self
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl<'a> Builder for CreateForumPost<'a> {
    type Context<'ctx> = ChannelId;
    type Built = GuildChannel;

    /// Creates a forum post in the given channel.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if invalid data is given.
    async fn execute(
        self,
        cache_http: impl CacheHttp,
        ctx: Self::Context<'_>,
    ) -> Result<Self::Built> {
        cache_http.http().create_forum_post(ctx, &self, self.audit_log_reason).await
    }
}
