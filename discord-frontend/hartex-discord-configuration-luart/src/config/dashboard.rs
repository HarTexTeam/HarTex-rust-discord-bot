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

use rlua::Context;
use rlua::Error;
use rlua::FromLua;
use rlua::Result;
use rlua::Value;

pub struct Dashboard {
    pub admins: Vec<String>,
    pub editors: Vec<String>,
    pub viewers: Vec<String>,
}

impl<'lua> FromLua<'lua> for Dashboard {
    fn from_lua(lua_value: Value<'lua>, lua: Context<'lua>) -> Result<Self> {
        let Value::Table(table) = lua_value else {
            return Err(Error::RuntimeError(
                String::from("mismatched value type"),
            ));
        };

        let admins: Vec<String> = table.get("admins")?;
        let editors: Vec<String> = table.get("editors")?;
        let viewers: Vec<String> = table.get("viewers")?;

        Ok(Self {
            admins,
            editors,
            viewers,
        })
    }
}
