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

use anyhow::Result;

use super::StorageIterator;

/// Merges two iterators of different types into one. If the two iterators have the same key, only
/// produce the key once and prefer the entry from A.
pub struct TwoMergeIterator<A: StorageIterator, B: StorageIterator> {
    a: A,
    b: B,
    choose_a: bool,
}

impl<
    A: 'static + StorageIterator,
    B: 'static + for<'a> StorageIterator<KeyType<'a> = A::KeyType<'a>>,
> TwoMergeIterator<A, B>
{
    pub fn create(a: A, mut b: B) -> Result<Self> {
        Self::skip_b(&a, &mut b)?;
        let choose_a = Self::choose_a(&a, &b);
        Ok(Self { a, b, choose_a })
    }

    fn skip_b(a: &A, b: &mut B) -> Result<()> {
        if a.is_valid() && b.is_valid() && a.key() == b.key() {
            b.next()?;
        }
        Ok(())
    }

    fn choose_a(a: &A, b: &B) -> bool {
        if !a.is_valid() {
            return false;
        }
        if !b.is_valid() {
            return true;
        }
        a.key() < b.key()
    }
}

impl<
    A: 'static + StorageIterator,
    B: 'static + for<'a> StorageIterator<KeyType<'a> = A::KeyType<'a>>,
> StorageIterator for TwoMergeIterator<A, B>
{
    type KeyType<'a> = A::KeyType<'a>;

    fn key(&self) -> Self::KeyType<'_> {
        if self.choose_a {
            self.a.key()
        } else {
            self.b.key()
        }
    }

    fn value(&self) -> &[u8] {
        if self.choose_a {
            self.a.value()
        } else {
            self.b.value()
        }
    }

    fn is_valid(&self) -> bool {
        if self.choose_a {
            self.a.is_valid()
        } else {
            self.b.is_valid()
        }
    }

    fn next(&mut self) -> Result<()> {
        if self.choose_a {
            self.a.next()?;
        } else {
            self.b.next()?;
        }

        Self::skip_b(&self.a, &mut self.b)?;
        self.choose_a = Self::choose_a(&self.a, &self.b);
        Ok(())
    }
}
