// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use displaydoc::Display;

/// A list of error outcomes for exceeding location bounds
#[derive(Display, Debug, Copy, Clone, PartialEq)]
pub enum LocationOutOfBoundsError {
    /// Latitude value was out of bounds
    #[displaydoc("Latitude {0} outside bounds of -90 to 90")]
    Latitude(f64),

    /// Longitude value was out of bounds
    #[displaydoc("Longitude {0} outside bounds of -180 to 180")]
    Longitude(f64),

    /// Offset value was out of bounds
    #[displaydoc("Offset {0} outside bounds of {1} to {2}")]
    Offset(f64, f64, f64),
}
