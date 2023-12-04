// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate jsonschema;
extern crate log;
extern crate num_derive;
extern crate num_traits;
extern crate once_cell;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate spdx;
extern crate strum;
extern crate strum_macros;
extern crate thiserror;
extern crate toml;

mod conversion;
mod formats;
mod license;
mod logger;
mod macros;
mod oxrl;
mod validation;

use git_version::git_version;

pub const VERSION: &str = git_version!(cargo_prefix = "", fallback = "unknown");
