// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    backends::{HostInfoBackend, RawHostInfoBackend},
    error::HostInfoError,
};

pub struct UnavailableHostInfoBackend;

impl HostInfoBackend for UnavailableHostInfoBackend {}

impl RawHostInfoBackend for UnavailableHostInfoBackend {}
