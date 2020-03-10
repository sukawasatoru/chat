/*
 * Copyright 2019, 2020 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::str::FromStr;

use crate::prelude::*;

#[derive(Debug, Eq, Ord, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub fn parse<T: AsRef<str>>(version: T) -> Fallible<Self> {
        Ok(version.as_ref().parse()?)
    }
}

impl AsRef<Version> for Version {
    fn as_ref(&self) -> &Version {
        self
    }
}

impl FromStr for Version {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('.').collect();
        Ok(Self {
            major: v.get(0).ok_or_err()?.parse()?,
            minor: v.get(1).ok_or_err()?.parse()?,
            patch: v.get(2).ok_or_err()?.parse()?,
        })
    }
}

impl Into<Version> for [u16; 3] {
    fn into(self) -> Version {
        Version {
            major: self[0],
            minor: self[1],
            patch: self[2],
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd<Version> for Version {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        let major = self.major.cmp(&other.major);
        if major != std::cmp::Ordering::Equal {
            return Some(major);
        }

        let minor = self.minor.cmp(&other.minor);
        if minor != std::cmp::Ordering::Equal {
            return Some(minor);
        }

        let patch = self.patch.cmp(&other.patch);
        if patch != std::cmp::Ordering::Equal {
            return Some(patch);
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_str(StrVisitor)?;
        match Version::parse(value) {
            Ok(data) => Ok(data),
            Err(e) => Err(serde::de::Error::custom(&format!("{:?}", e))),
        }
    }
}

pub struct StrVisitor;

impl<'de> serde::de::Visitor<'de> for StrVisitor {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a borrowed string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_u32_into() {
        assert_eq!(
            Into::<Version>::into([1, 2, 3]),
            Version {
                major: 1,
                minor: 2,
                patch: 3,
            },
        )
    }

    #[test]
    fn test_from_str() -> Fallible<()> {
        assert_eq!(
            "1.2.3".parse::<Version>()?,
            Version {
                major: 1,
                minor: 2,
                patch: 3,
            }
        );
        Ok(())
    }

    #[test]
    fn test_display() {
        assert_eq!(
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
            .to_string(),
            "1.2.3"
        )
    }

    #[test]
    fn test_compare() -> Fallible<()> {
        assert!("1.2.3".parse::<Version>()? == "1.2.3".parse()?);
        assert!("1.2.3".parse::<Version>()? < "1.2.4".parse()?);
        assert!("1.2.3".parse::<Version>()? < "1.3.2".parse()?);
        assert!("1.2.3".parse::<Version>()? < "2.1.2".parse()?);
        Ok(())
    }
}
