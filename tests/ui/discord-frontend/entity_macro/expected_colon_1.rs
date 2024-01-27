// ==BEGIN TESTSUITE DECL==
// testsuite-type: ui
// testsuite-result: compile-fail
// ==END TESTSUITE DECL==

/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

extern crate core;
extern crate hartex_discord_entitycache_macros;

use hartex_discord_entitycache_macros::entity;

#[entity(
    from = "twilight_model::channel::Channel",
    assume = [],
    id = [],
    exclude = [],
    extra = ["extra_id"],
    overrides = [],
    relates = []
)]
pub struct ExpectedColon1;

fn main() {}
