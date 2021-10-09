use std::sync::Arc;

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        gateway::Cluster,
        http::Client,
        model::application::interaction::Interaction
    },
    error::HarTexResult,
    logging::tracing::{
        self,
        Instrument
    }
};

use hartex_cmdsys::{
    command::Command,
    context::{
        CommandContext,
        CommandContextInner
    }
};

use hartex_plugins::{
    globadmin_only::refroles::Refroles,
    global::{
        about::About,
        ping::Ping,
        source::Source,
        team::Team,
    },
    information::{
        guildinfo::Guildinfo,
        userinfo::Userinfo
    }
};

/// # Asynchronous Function `handle_interaction`
///
/// Handles the incoming interaction asynchronously.
///
/// ## Parameters
/// - `interaction`, type `Interaction`: the interaction
/// - `cache`, type `InMemoryCache`: the in-memory cache
/// - `http`, type `Client`: the Twilight HTTP client
/// - `cluster`: the gateway cluster
pub async fn handle_interaction(
    interaction: Interaction,
    cache: InMemoryCache,
    http: Client,
    cluster: Cluster
) -> HarTexResult<()> {
    let span = tracing::trace_span!("interaction handler");

    match {
        match interaction.clone() {
            Interaction::ApplicationCommand(command) => {
                match &*command.data.name {
                    // Global Administrator Only Plugin
                    "refroles" => {
                        Refroles.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }

                    // Global Plugin
                    "about" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `about`; invoking command handler");
                        });

                        let span = tracing::trace_span!("interaction command handler: about command");

                        About.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        )
                            .instrument(span)
                            .await
                    }
                    "ping" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `ping`; invoking command handler");
                        });

                        Ping.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "source" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `source`; invoking command handler");
                        });

                        Source.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "team" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `team`; invoking command handler");
                        });

                        Team.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }

                    // Information Plugin
                    "guildinfo" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `guildinfo`; invoking command handler");
                        });

                        Guildinfo.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "userinfo" => {
                        span.in_scope(|| {
                            tracing::trace!("interaction command identified - `userinfo`; invoking command handler");
                        });

                        Userinfo.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    } {
        Ok(_) => (),
        Err(error) => {
            tracing::error!(
                "failed to handle interaction due to an error: {error:?}"
            );
        }
    }

    Ok(())
}