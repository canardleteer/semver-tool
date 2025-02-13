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
//! These are generally the "Results" we're looking for, as types.
use std::{cmp::Ordering, fmt};

use indexmap::IndexMap;
use rand::prelude::*;
use semver::{BuildMetadata, Version, VersionReq};
use serde::Serialize;

// use super::misc::*;

/// The result of a simple filter test.
#[derive(Serialize, PartialEq)]
pub(crate) struct FilterTestResult {
    pass: bool,
}

impl FilterTestResult {
    pub(crate) fn filter_test(filter: &VersionReq, semantic_version: &Version) -> FilterTestResult {
        filter.matches(semantic_version).into()
    }
}

impl From<bool> for FilterTestResult {
    fn from(value: bool) -> Self {
        Self { pass: value }
    }
}

impl fmt::Display for FilterTestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pass: {}", self.pass)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) enum SegmentType {
    Numeric,
    Ascii,
}

impl fmt::Display for SegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegmentType::Numeric => write!(f, "Numeric"),
            SegmentType::Ascii => write!(f, "Ascii"),
        }
    }
}

/// Describes a dot separated segment of either a Pre-Release, or Build Metadata string.
///
/// Kind describes how the value is meant to be interpreted for precedence.
#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct PreMetaSegment {
    kind: SegmentType,
    value: String,
}

impl fmt::Display for PreMetaSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.value, self.kind)
    }
}

impl From<&str> for PreMetaSegment {
    fn from(value: &str) -> Self {
        // WARNING(canardletter): Note, the spec does not enforce any numeric limit here.
        let digit_count = value
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect::<Vec<char>>()
            .len();

        PreMetaSegment {
            kind: if digit_count == value.len() {
                SegmentType::Numeric
            } else {
                SegmentType::Ascii
            },
            value: value.to_string(),
        }
    }
}

/// Descriptive information about a Version.
#[derive(Serialize, PartialEq)]
pub(crate) struct VersionExplaination {
    major: u64,
    minor: u64,
    patch: u64,
    prerelease_string: String,
    #[serde(rename(serialize = "prerelease"))]
    prerelease: Vec<PreMetaSegment>,
    build_metadata_string: String,
    #[serde(rename(serialize = "build-metadata"))]
    build_metadata: Vec<PreMetaSegment>,
}

impl From<&Version> for VersionExplaination {
    fn from(value: &Version) -> Self {
        Self {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
            prerelease: value
                .pre
                .as_str()
                .split('.')
                .map(PreMetaSegment::from)
                .collect(),
            prerelease_string: value.pre.to_string(),
            build_metadata: value
                .build
                .as_str()
                .split('.')
                .map(PreMetaSegment::from)
                .collect(),
            build_metadata_string: value.build.to_string(),
        }
    }
}

impl fmt::Display for VersionExplaination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Major: {}", self.major)?;
        writeln!(f, "Minor: {}", self.minor)?;
        writeln!(f, "Patch: {}", self.patch)?;
        writeln!(f, "PreRelease: {}", self.prerelease_string)?;
        for i in self.prerelease.iter() {
            writeln!(f, "- {i}")?;
        }
        writeln!(f, "Build Metadata: {}", self.build_metadata_string)?;
        for i in self.build_metadata.iter() {
            writeln!(f, "- {i}")?;
        }
        Ok(())
    }
}

/// A simple list of Versions.
#[derive(Serialize, PartialEq)]
pub(crate) struct FlatVersionsList {
    versions: Vec<Version>,
}

impl From<&mut OrderedVersionMap> for FlatVersionsList {
    fn from(value: &mut OrderedVersionMap) -> Self {
        let mut flat: Vec<Version> = Vec::new();

        value.inner.iter_mut().for_each(|vv| flat.append(vv.1));
        Self { versions: flat }
    }
}

impl fmt::Display for FlatVersionsList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.versions.iter() {
            writeln!(f, "{v}")?
        }
        Ok(())
    }
}

/// A usefully ordered list of versions.
#[derive(Serialize)]
pub(crate) struct OrderedVersionMap {
    #[serde(rename(serialize = "versions"))]
    inner: IndexMap<Version, Vec<Version>>,
}

impl OrderedVersionMap {
    pub(crate) fn new(
        versions: &mut Vec<Version>,
        filter: &Option<VersionReq>,
        lexical_sorting: bool,
        reverse: bool,
    ) -> Self {
        if let Some(filter) = filter {
            versions.retain(|v| filter.matches(v));
        }

        // Generally sort the input for keys into the IndexMap.
        versions.sort();

        // Reverse the ordering, if appropriate.
        if reverse {
            versions.reverse()
        }

        // Create our return structure.
        let mut ordered_version_map: IndexMap<Version, Vec<Version>> = IndexMap::new();

        // Capture all keys and complete Versions.
        for version in versions {
            let key = version_without_build_metadata(version);
            match ordered_version_map.get_mut(&key) {
                Some(v) => v.push(version.clone()),
                None => {
                    let new_value = vec![version.clone()];
                    let map_response = ordered_version_map.insert(key, new_value);
                    if map_response.is_some() {
                        panic!("should not have gotten a map response for an empty key")
                    }
                }
            }
        }

        // For each key, sort each list of versions in an appropriate order.
        for (_, v) in ordered_version_map.iter_mut() {
            if lexical_sorting {
                v.sort();
                if reverse {
                    v.reverse();
                }
            } else {
                v.shuffle(&mut rand::rng());
                // reverse is silently ignored in this case.
            }
        }

        Self {
            inner: ordered_version_map,
        }
    }
}

impl fmt::Display for OrderedVersionMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME: need a better walk here
        for key in self.inner.keys() {
            writeln!(f, "{key}:")?;
            if let Some(vals) = self.inner.get(key) {
                for val in vals {
                    writeln!(f, "\t- {val}")?;
                }
            }
        }
        Ok(())
    }
}

/// A statement about the comparison about 2 versions
#[derive(Serialize, PartialEq)]
pub(crate) struct ComparisonStatement {
    semantic_ordering: SerializableOrdering,
    lexical_ordering: SerializableOrdering,
}

impl ComparisonStatement {
    pub(crate) fn new(a: &Version, b: &Version) -> Self {
        let a_no_build = version_without_build_metadata(a);
        let b_no_build = version_without_build_metadata(b);

        Self {
            semantic_ordering: a_no_build.cmp(&b_no_build).into(),
            lexical_ordering: a.cmp(b).into(),
        }
    }
}

impl fmt::Display for ComparisonStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Semantically: {:?}\nLexically: {:?}\n",
            self.semantic_ordering, self.lexical_ordering
        )
    }
}

/// Just a small reimplementation of std::Ordering with Serialization.
#[derive(Debug, Serialize, PartialEq)]
pub(crate) enum SerializableOrdering {
    Less,
    Greater,
    Equal,
}

impl From<Ordering> for SerializableOrdering {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => SerializableOrdering::Less,
            Ordering::Equal => SerializableOrdering::Equal,
            Ordering::Greater => SerializableOrdering::Greater,
        }
    }
}

pub(crate) fn version_without_build_metadata(version: &Version) -> Version {
    Version {
        major: version.major,
        minor: version.minor,
        patch: version.patch,
        pre: version.pre.clone(),
        build: BuildMetadata::EMPTY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE(canardleteer): I normally prefer property testing for things like this,
    //                     and may add some eventually.

    #[test]
    fn test_version_without_build_metadata() {
        assert_eq!(
            version_without_build_metadata(&Version::parse("0.0.0-123.123+123.123").unwrap()).build,
            BuildMetadata::EMPTY
        );
    }

    // PreMetaSegment
    #[test]
    fn test_pre_meta_segment() {
        assert_eq!(
            PreMetaSegment::from("00000aaaa00000"),
            PreMetaSegment {
                kind: SegmentType::Ascii,
                value: "00000aaaa00000".to_string()
            }
        );
        assert_eq!(
            PreMetaSegment::from("00000"),
            PreMetaSegment {
                kind: SegmentType::Numeric,
                value: "00000".to_string()
            }
        );
        assert_eq!(
            PreMetaSegment::from("00001"),
            PreMetaSegment {
                kind: SegmentType::Numeric,
                value: "00001".to_string()
            }
        );
        assert_eq!(
            PreMetaSegment::from("-00001"),
            PreMetaSegment {
                kind: SegmentType::Ascii,
                value: "-00001".to_string()
            }
        );
    }

    // SerializableOrdering
    // ComparisonStatement

    // Tests some simple static tests.
    #[test]
    fn test_ordered_version_map() {
        let mut scaffold1 = vec!["99.0.0", "100.0.0", "0.0.1"]
            .iter()
            .map(|v| Version::parse(v).unwrap())
            .collect();

        let test = OrderedVersionMap::new(&mut scaffold1, &None, false, false);
        println!("{:?}", test.inner.keys());
        assert!(test.inner.contains_key(&Version::parse("99.0.0").unwrap()));
        assert!(test.inner.contains_key(&Version::parse("100.0.0").unwrap()));
        assert!(test.inner.contains_key(&Version::parse("0.0.1").unwrap()));

        let mut scaffold2: Vec<Version> = vec![
            "0.0.0-alpha.0+metadata",
            "0.0.0-alpha.0+other.metadata",
            "0.0.0-alpha.0+other.metadata.3",
            "0.0.1",
            "0.0.2",
            "0.2.0",
            "0.2.99",
            "1.0.0-rc.1",
            "1.0.0-rc-1",
            "1.0.0-rc-2+aaaaaa",
            "1.0.0-rc-2+bbbbbb",
            "1.0.0-rc-2+cccccc",
            "1.0.0-rc-2+dddddd",
            "1.0.0-rc-2.0+dddddd",
            "1.0.0-rc-2.1+dddddd",
            "1.0.0-rc-2+dddddd.0",
            "1.0.0-rc-2+dddddd.1",
            "1.0.0-rc-2+eeeeee",
            "1.0.0+aaaaaa",
            "1.0.0",
            "99.99.0-rc1.0",
        ]
        .iter()
        .map(|v| Version::parse(v).unwrap())
        .collect();

        let test = OrderedVersionMap::new(&mut scaffold2, &None, false, false);
        let test_keys: Vec<Version> = test.inner.keys().map(|v| v.clone()).collect();
        assert!(test_keys.len() == 12);
        println!("{}", test_keys[0]);
        assert!(test_keys[0] == Version::parse("0.0.0-alpha.0").unwrap());
        assert!(test_keys[test_keys.len() - 1] == Version::parse("99.99.0-rc1.0").unwrap());

        // Reverse of above test.
        let test = OrderedVersionMap::new(&mut scaffold2, &None, false, true);
        let test_keys: Vec<Version> = test.inner.keys().map(|v| v.clone()).collect();
        assert!(test_keys.len() == 12);
        println!("{}", test_keys[0]);
        assert!(test_keys[test_keys.len() - 1] == Version::parse("0.0.0-alpha.0").unwrap());
        assert!(test_keys[0] == Version::parse("99.99.0-rc1.0").unwrap());

        // Filter, this should exclude all versions with pre-releases
        let test = OrderedVersionMap::new(
            &mut scaffold2,
            &Some(VersionReq::parse("*").unwrap()),
            false,
            false,
        );
        let test_keys: Vec<Version> = test.inner.keys().map(|v| v.clone()).collect();
        assert!(test_keys.len() == 5);
        println!("{}", test_keys[0]);
        assert!(test_keys[0] == Version::parse("0.0.1").unwrap());
        assert!(test_keys[test_keys.len() - 1] == Version::parse("1.0.0").unwrap());

        // Display Coverage
        let _ = format!("{}", test);
    }

    // FlatVersionsList
    // Static test around the basic structure.
    #[test]
    fn flat_version_list() {
        let mut scaffold: Vec<Version> = vec![
            "0.0.0-alpha.0+metadata",
            "0.0.0-alpha.0+other.metadata",
            "0.0.0-alpha.0+other.metadata.3",
            "0.0.1",
            "0.0.2",
            "0.2.0",
            "0.2.99",
            "1.0.0-rc.1",
            "1.0.0-rc-1",
            "1.0.0-rc-2+aaaaaa",
            "1.0.0-rc-2+bbbbbb",
            "1.0.0-rc-2+cccccc",
            "1.0.0-rc-2+dddddd",
            "1.0.0-rc-2.0+dddddd",
            "1.0.0-rc-2.1+dddddd",
            "1.0.0-rc-2+dddddd.0",
            "1.0.0-rc-2+dddddd.1",
            "1.0.0-rc-2+eeeeee",
            "1.0.0+aaaaaa",
            "1.0.0",
            "99.99.0-rc1.0",
        ]
        .iter()
        .map(|v| Version::parse(v).unwrap())
        .collect();

        // lexical sorting
        let mut test = OrderedVersionMap::new(&mut scaffold, &None, true, false);
        let test = FlatVersionsList::from(&mut test);
        assert!(test.versions.len() == 21);
        assert!(test.versions[0] == Version::parse("0.0.0-alpha.0+metadata").unwrap());
        assert!(test.versions[test.versions.len() - 1] == Version::parse("99.99.0-rc1.0").unwrap());

        // lexical sorting, reversed
        let mut test = OrderedVersionMap::new(&mut scaffold, &None, true, true);
        let test = FlatVersionsList::from(&mut test);
        assert!(test.versions.len() == 21);
        assert!(
            test.versions[test.versions.len() - 1]
                == Version::parse("0.0.0-alpha.0+metadata").unwrap()
        );
        assert!(test.versions[0] == Version::parse("99.99.0-rc1.0").unwrap());

        // Display Coverage
        let _ = format!("{}", test);
    }
    // VersionExplaination

    // static test for explain
    #[test]
    fn test_version_explaination() {
        let test =
            VersionExplaination::from(&Version::parse("0.0.0-0.a.b.c.4+0.-1.a.b0.3").unwrap());

        assert!(test.major == 0);
        assert!(test.minor == 0);
        assert!(test.patch == 0);

        assert!(test.prerelease.len() == 5);
        assert!(test.prerelease[1].kind == SegmentType::Ascii);
        assert!(test.prerelease[1].value == "a");
        assert!(test.prerelease[test.prerelease.len() - 1].kind == SegmentType::Numeric);
        assert!(test.prerelease[test.prerelease.len() - 1].value == "4");
        assert!(test.prerelease_string == "0.a.b.c.4");

        assert!(test.build_metadata.len() == 5);
        assert!(test.build_metadata[1].kind == SegmentType::Ascii);
        assert!(test.build_metadata[1].value == "-1");
        assert!(test.build_metadata[test.build_metadata.len() - 1].kind == SegmentType::Numeric);
        assert!(test.build_metadata[test.build_metadata.len() - 1].value == "3");
        assert!(test.build_metadata_string == "0.-1.a.b0.3");

        // Display Coverage
        let _ = format!("{}", test);
    }

    // FilterTestResult
    #[test]
    fn test_filter_test_result() {
        assert_eq!(
            FilterTestResult::filter_test(
                &VersionReq::parse(">1").unwrap(),
                &Version::parse("0.0.0").unwrap()
            )
            .pass,
            false
        );
        assert_eq!(
            FilterTestResult::filter_test(
                &VersionReq::parse(">1").unwrap(),
                &Version::parse("2.0.0").unwrap()
            )
            .pass,
            true
        );
        assert_eq!(
            FilterTestResult::filter_test(
                &VersionReq::parse(">=1").unwrap(),
                &Version::parse("1.0.0").unwrap()
            )
            .pass,
            true
        );

        // Display Coverage
        let test = FilterTestResult::filter_test(
            &VersionReq::parse(">=1").unwrap(),
            &Version::parse("1.0.0").unwrap(),
        );
        let _ = format!("{}", test);
    }

    // ComparisonStatement
    #[test]
    fn test_comparison_statement() {
        let test = ComparisonStatement::new(
            &Version::parse("0.0.0").unwrap(),
            &Version::parse("2.0.0").unwrap(),
        );
        assert_eq!(test.lexical_ordering, SerializableOrdering::Less);
        assert_eq!(test.semantic_ordering, SerializableOrdering::Less);

        let test = ComparisonStatement::new(
            &Version::parse("2.0.0+100").unwrap(),
            &Version::parse("2.0.0").unwrap(),
        );
        assert_eq!(test.lexical_ordering, SerializableOrdering::Greater);
        assert_eq!(test.semantic_ordering, SerializableOrdering::Equal);

        let test = ComparisonStatement::new(
            &Version::parse("2.0.0").unwrap(),
            &Version::parse("2.0.0-rc1").unwrap(),
        );
        assert_eq!(test.lexical_ordering, SerializableOrdering::Greater);
        assert_eq!(test.semantic_ordering, SerializableOrdering::Greater);

        // Display Coverage
        let _ = format!("{}", test);
    }
}
