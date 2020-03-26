/*
 * Copyright 2020 sukawasatoru
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

use crate::model::version::Version;

pub fn convert_to_version_code(version: &Version, flush_code: u16) -> u64 {
    // flush:                                                 1111111111111111
    // patch:                                 11111111111111110000000000000000
    // minor:                 111111111111111100000000000000000000000000000000
    // major: 1111111111111111000000000000000000000000000000000000000000000000
    //        FF              FF              FF              FF
    let version = version;
    ((version.major as u64) << 48)
        + ((version.minor as u64) << 32)
        + ((version.patch as u64) << 16)
        + flush_code as u64
}

pub fn convert_to_version(version_code: u64) -> (Version, u16) {
    let mut version_code = version_code;
    let mask = 0b1111_1111_1111_1111;

    let flush_code = (version_code & mask) as u16;
    version_code >>= 16;
    let patch = (version_code & mask) as u16;
    version_code >>= 16;
    let minor = (version_code & mask) as u16;
    version_code >>= 16;
    let major = version_code as u16;

    ([major, minor, patch].into(), flush_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_version_code_max() {
        assert_eq!(
            convert_to_version_code(
                &[std::u16::MAX, std::u16::MAX, std::u16::MAX].into(),
                std::u16::MAX,
            ),
            std::u64::MAX
        );
    }

    #[test]
    fn test_convert_to_version_max() {
        assert_eq!(
            convert_to_version(std::u64::MAX),
            (
                [std::u16::MAX, std::u16::MAX, std::u16::MAX].into(),
                std::u16::MAX
            )
        )
    }
}
