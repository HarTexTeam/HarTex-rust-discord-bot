/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use hartex_database_queries::queries::discord_frontend::cached_emoji_select_by_id::CachedEmojiSelectById;
use hartex_database_queries::queries::discord_frontend::cached_emoji_upsert::CachedEmojiUpsert;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::emoji::EmojiEntity;

/// Repository for emoji entities.
pub struct CachedEmojiRepository;

impl Repository<EmojiEntity> for CachedEmojiRepository {
    async fn get(&self, id: <EmojiEntity as Entity>::Id) -> CacheResult<EmojiEntity> {
        let data = CachedEmojiSelectById::bind(id.to_string())
            .executor()
            .await?
            .one()
            .await?;

        Ok(EmojiEntity::from(data))
    }

    async fn upsert(&self, entity: EmojiEntity) -> CacheResult<()> {
        CachedEmojiUpsert::bind(
            entity.animated,
            entity.id.to_string(),
            entity.guild_id.to_string(),
            entity.name,
            entity.managed,
        )
        .executor()
        .await?
        .execute()
        .await?;

        Ok(())
    }
}
