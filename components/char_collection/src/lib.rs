// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#[macro_use]
mod macros;
mod char_collection;
mod conversions;
mod operators;
pub use char_collection::CharCollection;
pub use char_collection::MultiCharRange;
pub use conversions::*;
pub use operators::*;