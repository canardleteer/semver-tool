//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2025 canardleteer
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
//!
//! This is a bunch of last mile display + serialization logic.
//!
//! NOTE(canardleteer): I just lifted these regexes from the [proptest-semver]
//!                     crate, because I don't want to pull in the full
//!                     dependencies.
//!
//! NOTE(canardleteer): Moving to all [rand] `0.9` would be nice, just borrowing
//!                     from the regex_generate example here.

extern crate rand_old;
extern crate regex_generate;
use rand_old::{thread_rng, Rng};
use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use semver::{BuildMetadata, Prerelease};

/// Regex for Semantic Version 2.0.0, directly from the spec, with 2 changes:
///
/// * ASCII Only Restriction
/// * No prepended `^` or trailing `$`, since [proptest!] uses this with the
///   [regex_generate](https://github.com/CryptArchy/regex_generate) crate.
pub const SEMVER_REGEX: &str = r"(?-u:(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)";

/// Regex to build a Pre-Release string, always, without the `-`.
pub const ALWAYS_PRERELEASE_REGEX: &str = r"(?-u:(?:((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*)))";

/// Regex to build a Build Metadata string, always, without the prefix `+`.
pub const ALWAYS_BUILD_METADATA_REGEX: &str = r"(?-u:(?:([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*)))";

/// Generate [Vec<String>] filled with valid Semantic Versions.
pub(crate) fn generate_any_valid_semver(count: usize) -> Vec<String> {
    let mut semver = Generator::new(SEMVER_REGEX, thread_rng(), DEFAULT_MAX_REPEAT).unwrap();

    let mut v = Vec::with_capacity(count);
    for _i in 0..count {
        let mut buffer = vec![];
        semver.generate(&mut buffer).unwrap();
        v.push(String::from_utf8(buffer).unwrap())
    }
    v
}

/// Generate [Vec<String>] filled with valid Semantic Versions bound by [u64::MAX]
/// promises for MAJOR, MINOR and PATCH.
///
/// This could probably be done better.
pub(crate) fn generate_u64_safe_semver(count: usize) -> Vec<String> {
    let mut pre_release_gen =
        Generator::new(ALWAYS_PRERELEASE_REGEX, thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut build_metadata_gen = Generator::new(
        ALWAYS_BUILD_METADATA_REGEX,
        thread_rng(),
        DEFAULT_MAX_REPEAT,
    )
    .unwrap();

    let mut v = Vec::with_capacity(count);
    for _i in 0..count {
        let pre: Option<Prerelease> = match thread_rng().gen_bool(0.5) {
            true => {
                let mut buffer = vec![];
                pre_release_gen.generate(&mut buffer).unwrap();
                Some(Prerelease::new(&String::from_utf8(buffer).unwrap()).unwrap())
            }
            false => None,
        };

        let build: Option<BuildMetadata> = match thread_rng().gen_bool(0.5) {
            true => {
                let mut buffer = vec![];
                build_metadata_gen.generate(&mut buffer).unwrap();
                Some(BuildMetadata::new(&String::from_utf8(buffer).unwrap()).unwrap())
            }
            false => None,
        };

        let mut s = format!(
            "{}.{}.{}",
            thread_rng().gen::<u64>(),
            thread_rng().gen::<u64>(),
            thread_rng().gen::<u64>()
        );
        if let Some(pre) = pre {
            s = format!("{s}-{}", pre);
        }
        if let Some(build) = build {
            s = format!("{s}+{}", build);
        }

        v.push(s)
    }
    v
}
