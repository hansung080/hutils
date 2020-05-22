// Copyright (c) The Hutils Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

/// `ClosureCacher` caches the result of invoking the closure.
/// 
/// ### Examples
/// ```
/// use hutils::ClosureCacher;
/// 
/// let mut cacher = ClosureCacher::new(|x| x);
/// 
/// assert_eq!(1, *cacher.result(&1));
/// ```
#[allow(unused)]
pub struct ClosureCacher<'a, F, K, V>
where
    F: Fn(&K) -> &V,
    K: Hash + Eq,
{
    closure: F,
    results: HashMap<&'a K, &'a V>,
}

impl<'a, F, K, V> ClosureCacher<'a, F, K, V>
where
    F: Fn(&K) -> &V,
    K: Hash + Eq,
{
    /// `new` creates a new `ClosureCacher`.
    #[allow(unused)]
    pub fn new(closure: F) -> ClosureCacher<'a, F, K, V> {
        ClosureCacher {
            closure,
            results: HashMap::new(),
        }
    }

    /// `result` returns the cached result if it exists.
    /// Otherwise, it invokes the closure and returns the result.
    #[allow(unused)]
    pub fn result(&mut self, arg: &'a K) -> &'a V {
        match self.results.get(arg) {
            Some(result) => result,
            None => {
                let result = (self.closure)(arg);
                self.results.insert(arg, result);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_cacher_i32() {
        let mut cacher = ClosureCacher::new(|x| x);

        assert_eq!(1, *cacher.result(&1));
        assert_eq!(2, *cacher.result(&2));
    }

    #[test]
    fn closure_cacher_string() {
        let mut cacher = ClosureCacher::new(|x| x);
        let foo = String::from("foo");
        let bar = String::from("bar");

        assert_eq!(String::from("foo"), *cacher.result(&foo));
        assert_eq!(String::from("bar"), *cacher.result(&bar));
    }
}