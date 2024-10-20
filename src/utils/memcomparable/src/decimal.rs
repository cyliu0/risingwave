// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

/// An extended decimal number with `NaN`, `-Inf` and `Inf`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decimal {
    /// Not a Number.
    NaN,
    /// Negative infinity.
    NegInf,
    /// Normalized value.
    Normalized(rust_decimal::Decimal),
    /// Infinity.
    Inf,
}

impl Decimal {
    /// A constant representing 0.
    pub const ZERO: Self = Decimal::Normalized(rust_decimal::Decimal::ZERO);
}

impl From<rust_decimal::Decimal> for Decimal {
    fn from(decimal: rust_decimal::Decimal) -> Self {
        Decimal::Normalized(decimal)
    }
}

impl FromStr for Decimal {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nan" => Ok(Decimal::NaN),
            "-inf" => Ok(Decimal::NegInf),
            "inf" => Ok(Decimal::Inf),
            _ => Ok(Decimal::Normalized(s.parse()?)),
        }
    }
}
