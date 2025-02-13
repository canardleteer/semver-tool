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
use clap::ValueEnum;
use core::fmt;
use serde::Serialize;
use thiserror::Error;

use crate::results;

#[derive(Error, Debug)]
pub(crate) enum ApplicationError {
    /// We got invalid input.
    #[error("Invalid input (expected {expected:?}, got {found:?}")]
    InvalidArgument { expected: String, found: String },

    /// We were unable to prepare the output as requested.
    #[error("Failed to prepare output in this format {err:?}")]
    OutputFormatError { err: String },
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum OutputFormat {
    Text,
    Yaml,
    Json,
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum ApplicationOutput {
    /// Asseration by this program
    ComparisonStatement(results::ComparisonStatement),
    /// Ordered Map representation of versions
    OrderedVersionMap(results::OrderedVersionMap),
    /// Breakdown of version
    VersionExplaination(results::VersionExplaination),
    /// Flat list of versions
    FlatVersionsList(results::FlatVersionsList),
    /// Results from a test
    FilterTestResult(results::FilterTestResult),
}

impl From<results::ComparisonStatement> for ApplicationOutput {
    fn from(value: results::ComparisonStatement) -> Self {
        ApplicationOutput::ComparisonStatement(value)
    }
}

impl From<results::OrderedVersionMap> for ApplicationOutput {
    fn from(value: results::OrderedVersionMap) -> Self {
        ApplicationOutput::OrderedVersionMap(value)
    }
}
impl From<results::VersionExplaination> for ApplicationOutput {
    fn from(value: results::VersionExplaination) -> Self {
        ApplicationOutput::VersionExplaination(value)
    }
}

impl From<results::FlatVersionsList> for ApplicationOutput {
    fn from(value: results::FlatVersionsList) -> Self {
        ApplicationOutput::FlatVersionsList(value)
    }
}

impl From<results::FilterTestResult> for ApplicationOutput {
    fn from(value: results::FilterTestResult) -> Self {
        ApplicationOutput::FilterTestResult(value)
    }
}

/// Display for Application Output
///
/// While this exists, I'm tempted to just make the default YAML.
impl fmt::Display for ApplicationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationOutput::ComparisonStatement(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::OrderedVersionMap(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::VersionExplaination(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::FlatVersionsList(v) => {
                write!(f, "{}", v)
            }
            ApplicationOutput::FilterTestResult(v) => {
                write!(f, "{}", v)
            }
        }
    }
}
