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

//! # Logging Facilities

pub use log;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Logger;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

/// Initialize logging factilities.
pub fn initialize() {
    let conf = Config::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "{h({l:>6} | {d(%Y-%m-%d %H:%M:%S %Z)(local):>30} | {M} - {f}:{L} - {m})}{n}",
                        )))
                        .build(),
                ),
            ),
        )
        .logger(Logger::builder().build("h2", LevelFilter::Error))
        .logger(Logger::builder().build("hyper", LevelFilter::Error))
        .logger(Logger::builder().build("hyper_util", LevelFilter::Error))
        .logger(Logger::builder().build("mio", LevelFilter::Error))
        .logger(Logger::builder().build("rdkafka", LevelFilter::Warn))
        .logger(Logger::builder().build("rustls", LevelFilter::Error))
        .logger(Logger::builder().build("tokio", LevelFilter::Error))
        .logger(Logger::builder().build("tokio_postgres", LevelFilter::Error))
        .logger(Logger::builder().build("tokio_tungstenite", LevelFilter::Error))
        .logger(Logger::builder().build("tokio_util", LevelFilter::Error))
        .logger(Logger::builder().build("shard", LevelFilter::Error))
        .logger(Logger::builder().build("trust_dns_proto", LevelFilter::Error))
        .logger(Logger::builder().build("trust_dns_resolver", LevelFilter::Error))
        .logger(Logger::builder().build("tungstenite", LevelFilter::Error))
        .logger(Logger::builder().build("twilight_gateway", LevelFilter::Error))
        .logger(Logger::builder().build("twilight_http", LevelFilter::Error))
        .logger(Logger::builder().build("twilight_model", LevelFilter::Error))
        .logger(Logger::builder().build("want", LevelFilter::Error))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .expect("failed to build log4rs configuration");

    log4rs::init_config(conf).expect("failed to initialize log4rs");
}
