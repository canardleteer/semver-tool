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
//! This source file doesn't contain much more than just the basics of
//! CLI documentation, and routing to the appropriate place.

//! NOTE(canardleteer): We allow bare_urls, because CLI documentation is
//!                     more important than rust-doc here.
#![allow(rustdoc::bare_urls)]

use clap::{Parser, Subcommand};
use semver::{Version, VersionReq};
use std::error::Error;
use std::io;

mod misc;
mod results;

use misc::*;
use results::*;

/// Simple program to work with Semantic Versioning 2.0.0
///
/// https://github.com/canardleteer/semver-tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,

    #[clap(long, short = 'o', value_enum, default_value_t=OutputFormat::Yaml)]
    out: OutputFormat,
}

/// All commands available
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Explain A Version as parsed by the spec
    ///
    /// Breaks apart the Semantic Version, into it's individual components.
    ///
    /// All values are returned as strings, because the unsigned integer
    /// types are not necessarilly bound by a numeric type that is parseable
    /// by common libraries.
    ///
    /// It is worth noting, Semver 2.0.0 §11.4.1 & §11.4.2 pre-release &
    /// metadata dot separated values, cannot be negative numbers, since
    /// they cannot be represented with hypens.
    ///
    /// Reference: https://semver.org/#spec-item-11
    ///
    Explain { semantic_version: Version },
    /// Compare 2 versions.
    ///
    /// Results are provided in the form
    /// "A is {Greater,Equals,Less} {to,than} B", with both Semantic results
    /// (meaninful results under Semantic Versioning), as well as Lexical
    /// results (meaninless, but handy for sorting).
    Compare {
        /// The base version used for comparison.
        a: Version,
        /// The version we are comparing against.
        b: Version,
    },
    /// Sort a list of versions, with either Semantic or Lexical ordering.
    ///
    /// Results are grouped by default, under the meaningful components of Semantic
    /// Versioning (without build metadata), then enumerated under that component.
    Sort {
        #[clap(long, short = 'f', default_value = None)]
        /// Only emit versions that match a filter.
        ///
        /// These filter rules are described by the semver crate `VersionReq``
        /// documentation, and more generally in the cargo book.
        ///
        /// In particular, note the warnings around pre-releases in the
        /// VersionReq documentation.
        ///
        /// References:
        /// - https://docs.rs/semver/1.0.25/semver/struct.VersionReq.html
        /// - https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
        filter: Option<VersionReq>,

        #[clap(long, action)]
        /// Lexical Sorting (aka Total Order).
        ///
        /// WARNING: This may lead to bad choices surrounding semantic
        /// versioning,
        ///
        /// This is bound to be controversial, but worth understanding.
        ///
        /// Semver 2.0.0 §10 states that:
        /// "Build metadata MUST be ignored when determining version
        /// precedence."
        ///
        /// This has been set to the default behavior of emulating undefined
        /// behavior, because it MUST be ignored. It is quite common, for
        /// people to accidentally choose the sorting order of their favorite
        /// or most familiar tool, and not the specification itself. This
        /// enforces by default, the ignoring of the version precedence.
        ///
        /// Additionally, we must interpret the following statement as
        /// undefined ordering for the case where Build Metadata may be `None`
        /// or `Some`:
        ///
        /// "Thus two versions that differ only in the build metadata, have
        /// the same precedence."
        ///
        /// References:
        /// - https://semver.org/#spec-item-10
        lexical_sorting: bool,

        #[clap(long, short = 'r', action)]
        /// Reverses ordering.
        ///
        /// Note, "reversing" always effects the comparable versions being
        /// ordered, but is ignored when NOT lexically sorted, for the list of
        /// sematically identical versions (aka, different metadata). Since by
        /// default they are randomly sorted, there is no point.
        reverse: bool,

        #[clap(long, action)]
        /// Flatten the map, and provide a list of versions.
        ///
        /// WARNING: This may lead to bad choices surrounding semantic
        /// versioning.
        flatten: bool,

        /// If no versions are present, then the tool will read from stdin, one
        /// version per line.
        versions: Option<Vec<Version>>,
    },
    /// Test a version against a filter
    FilterTest {
        /// Filter to test against version.
        ///
        /// These filter rules are described by the semver crate `VersionReq``
        /// documentation, and more generally in the cargo book.
        ///
        /// In particular, note the warnings around pre-releases in the
        /// VersionReq documentation.
        ///
        /// References:
        /// - https://docs.rs/semver/1.0.25/semver/struct.VersionReq.html
        /// - https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
        filter: VersionReq,

        /// Version to test
        semantic_version: Version,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let application_output: ApplicationOutput = match args.cmd {
        Commands::Explain { semantic_version } => explain(&semantic_version).into(),
        Commands::Compare { a, b } => compare(&a, &b).into(),
        Commands::Sort {
            versions,
            filter,
            lexical_sorting,
            reverse,
            flatten,
        } => {
            let mut parsed_versions = Vec::new();

            // Read from stdin, or pass forward the pre-parsed list from the arguments
            match versions {
                Some(versions) => parsed_versions = versions,
                None => {
                    let lines = io::stdin().lines();
                    for (line_no, line) in lines.enumerate() {
                        match line {
                            Ok(line) => {
                                let line = line.trim();
                                parsed_versions.push(Version::parse(line)
                                .map_err(|e| {
                                    eprintln!("unable to parse an enumerated version: line {line_no}: {line}: {e}");
                                    e
                                })?);
                                Ok(())
                            }
                            Err(e) => {
                                eprintln!("unable to read from stdin: {e}");
                                Err(ApplicationError::InvalidArgument {
                                    expected: "to be able to read from stdin".to_string(),
                                    found: e.to_string(),
                                })
                            }
                        }?
                    }
                }
            }

            let mut ordered_version_list =
                sort(&mut parsed_versions, &filter, lexical_sorting, reverse);

            match flatten {
                true => FlatVersionsList::from(&mut ordered_version_list).into(),
                false => ordered_version_list.into(),
            }
        }
        Commands::FilterTest {
            filter,
            semantic_version,
        } => filter_test(&filter, &semantic_version).into(),
    };

    match args.out {
        OutputFormat::Text => print!("{application_output}"),
        OutputFormat::Yaml => {
            println!("---");
            let yaml = serde_yaml::to_string(&application_output)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            print!("{yaml}");
        }
        OutputFormat::Json => {
            let json = serde_json::to_string(&application_output)
                .map_err(|e| ApplicationError::OutputFormatError { err: e.to_string() })?;
            println!("{json}");
        }
    }

    // TODO(canarleteer): We should include status codes upon exit based on flags.
    Ok(())
}

fn sort(
    versions: &mut Vec<Version>,
    filter: &Option<VersionReq>,
    lexical_sorting: bool,
    reverse: bool,
) -> OrderedVersionMap {
    OrderedVersionMap::new(versions, filter, lexical_sorting, reverse)
}

/// Returns the semantic and lexical equivalence of 2 versions.
fn compare(a: &Version, b: &Version) -> ComparisonStatement {
    ComparisonStatement::new(a, b)
}

fn explain(v: &Version) -> VersionExplaination {
    VersionExplaination::from(v)
}
fn filter_test(filter: &VersionReq, semantic_version: &Version) -> FilterTestResult {
    FilterTestResult::filter_test(filter, semantic_version)
}
