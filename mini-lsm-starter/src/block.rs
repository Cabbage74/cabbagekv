// Copyright (c) 2022-2025 Alex Chi Z
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

use std::mem::offset_of;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the course
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(self.data.len() + self.offsets.len() * 2 + 2);
        buf.put_slice(&self.data);
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        buf.put_u16(self.offsets.len() as u16);

        buf.freeze()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        let num_elements_pos = data.len() - 2;
        let num_elements = (&data[num_elements_pos..]).get_u16() as usize;

        let offsets_pos = num_elements_pos - num_elements * 2;
        let data_section = data[0..offsets_pos].to_vec();

        let mut offsets = Vec::with_capacity(num_elements);
        let mut offsets_data = &data[offsets_pos..num_elements_pos];
        for _ in 0..num_elements {
            offsets.push(offsets_data.get_u16());
        }

        Self {
            data: data_section,
            offsets,
        }
    }
}
