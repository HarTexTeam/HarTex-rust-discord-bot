//! # The `error` Module
//!
//! This module defines several types for error handling in the HarTex Discord bot.

use std::string::FromUtf8Error;

use base64::DecodeError;

use ctrlc::Error as CtrlcError;

use toml::de::Error as TomlDeserializationError;

use crate::discord::{
    embed_builder::EmbedError,
    gateway::cluster::ClusterStartError,
    http::{
        error::Error as HttpError,
        request::channel::message::create_message::CreateMessageError
    },
    model::gateway::payload::update_presence::UpdatePresenceError
};

/// # Enum `HarTexError`
///
/// An enumeration representing the various error types used within HarTex.
#[derive(Debug)]
pub enum HarTexError {
    /// # Enum Variant HarTexError::ClusterStartError
    ///
    /// A wrapper around `twilight_gateway::cluster::ClusterStartError`.
    ///
    /// ## Fields
    /// - `error`, type `ClusterStartError`: the cluster start error returned when building the
    ///                                      cluster.
    ClusterStartError {
        error: ClusterStartError
    },

    /// # Enum Variant `HarTexError::CreateMessageError`
    ///
    /// A wrapper around `twilight_http::request::channel::message::create_message::CreateMessageError`.
    ///
    /// ## Fields
    /// - `error`, type `CreateMessageError`: the error returned when attempting to send a message,
    CreateMessageError {
        error: CreateMessageError
    },

    /// # Enum Variant `HarTexError::Base64DecodeError`
    ///
    /// A wrapper around `base64::DecodeError`
    ///
    /// ## Fields
    /// - `error`, type `DecodeError`: the error returned when attempting to decode base64.
    Base64DecodeError {
        error: DecodeError
    },

    /// # Enum Variant `HarTexError::CtrlcError`
    ///
    /// A wrapper around `ctrlc::Error`.
    ///
    /// ## Fields
    /// - `error`, type `Error`: the ctrlc error returned when setting the ctrl-c handler.
    CtrlcError {
        error: CtrlcError
    },

    /// # Enum Variant `HarTexError::EmbedError`
    ///
    /// A wrapper around `twilight_embed_builder::EmbedError`.
    ///
    /// ## Fields
    /// - `error`, type `EmbedError`: the embed error returned when building a Discord embed.
    EmbedError {
        error: EmbedError
    },

    /// # Enum Variant `TomlDeserializationError`
    ///
    /// A wrapper around `toml::de::Error`
    ///
    /// ## Fields
    ///
    /// - `error`, type `Error`: the TOML deserialization error returned when attempting to
    ///                          deserializing TOML.
    TomlDeserializationError {
        error: TomlDeserializationError
    },

    /// # Enum Variant `HHarTexError::TwilightHttpError`
    ///
    /// A wrapper around `twilight_http::error::Error`.
    ///
    /// ## Fields
    /// - `error`, type `Error`: the error returned when executing an HTTP request.
    TwilightHttpError {
        error: HttpError
    },

    /// # Enum Variant `HarTexError::UpdatePresenceError`
    ///
    /// A wrapper around `twilight_model::gateway::paylod::update_presence::UpdatePresenceError`.
    ///
    /// ## Fields
    /// - `error`, type `UpdatePresenceError`: the error presence update error returned when
    ///                                        attempting to update the bot's presence.
    UpdatePresenceError {
        error: UpdatePresenceError
    },
    
    /// # Enum Variant `HarTexError::Utf8ValidationError`
    /// 
    /// A wrapper around `std::string::FromUtf8Error`.
    /// 
    /// ## Fields
    /// - `error`, type `FromUtf8Error`: the error returned when attempting to construct a string
    ///                                  with a `Vec<u8>` with the UTF-8 encoding.
    Utf8ValidationError {
        error: FromUtf8Error
    },

    /// # Enum Variant `HarTexError::Custom`
    ///
    /// Represents a custom error that cannot be represented with any other variants of this
    /// enumeration.
    ///
    /// ## Fields
    /// - `message`, type `&str`: the error message.
    Custom {
        message: String
    }
}

impl From<ClusterStartError> for HarTexError {
    fn from(error: ClusterStartError) -> Self {
        Self::ClusterStartError {
            error
        }
    }
}

impl From<CreateMessageError> for HarTexError {
    fn from(error: CreateMessageError) -> Self {
        Self::CreateMessageError {
            error
        }
    }
}

impl From<CtrlcError> for HarTexError {
    fn from(error: CtrlcError) -> Self {
        Self::CtrlcError {
            error
        }
    }
}

impl From<DecodeError> for HarTexError {
    fn from(error: DecodeError) -> Self {
        Self::Base64DecodeError {
            error
        }
    }
}

impl From<EmbedError> for HarTexError {
    fn from(error: EmbedError) -> Self {
        Self::EmbedError {
            error
        }
    }
}

impl From<FromUtf8Error> for HarTexError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8ValidationError {
            error
        }
    }
}

impl From<HttpError> for HarTexError {
    fn from(error: HttpError) -> Self {
        Self::TwilightHttpError {
            error
        }
    }
}

impl From<TomlDeserializationError> for HarTexError {
    fn from(error: TomlDeserializationError) -> Self {
        Self::TomlDeserializationError {
            error
        }
    }
}

impl From<UpdatePresenceError> for HarTexError {
    fn from(error: UpdatePresenceError) -> Self {
        Self::UpdatePresenceError {
            error
        }
    }
}

/// # Type Alias `HarTexResult<T>`
///
/// A type alias for `Result<T, HarTexError>`, used for error-handling.
pub type HarTexResult<T> = Result<T, HarTexError>;
