/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Some custom events that can be sent and received within `HarTex` Discord bot.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{Stream, StreamExt};
use hartex_model::payload::CommandExecuted;

/// A wrapper around `UnboundedReceiver<HarTexEvent>`; it receives event from
/// the stream.
#[allow(clippy::module_name_repetitions)]
pub struct Events {
    receiver: UnboundedReceiver<HarTexEvent>,
}

impl Events {
    /// Creates a new `Events` with the given `UnboundedReceiver`
    #[must_use]
    pub fn new(receiver: UnboundedReceiver<HarTexEvent>) -> Self {
        Self { receiver }
    }
}

impl Stream for Events {
    type Item = HarTexEvent;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(context)
    }
}

/// The various custom-defined events that is used within `HarTex`.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub enum HarTexEvent {
    /// A command is executed.
    CommandExecuted(Box<CommandExecuted>),
}
