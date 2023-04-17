// Copyright (c) The hUtils Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use std::hash::Hash;

/// `FnCacher` caches the result of a high-cost function using the design patterns of memoization and lazy evaluation.
/// 
/// ### Examples
/// ```
/// use hutils::FnCacher;
///
/// let mut square = FnCacher::new(|x| x * x);
///
/// assert_eq!(&9, square.call(&3));
/// ```
pub struct FnCacher<'a, F, T, R>
where
    F: Fn(&T) -> R,
    T: Eq + Hash,
{
    function: F,
    results: HashMap<&'a T, R>,
}

impl<'a, F, T, R> FnCacher<'a, F, T, R>
where
    F: Fn(&T) -> R,
    T: Eq + Hash,
{
    /// `new` constructs a `FnCacher` with `function`.
    pub fn new(function: F) -> Self {
        Self {
            function,
            results: HashMap::new(),
        }
    }

    /// `call` returns the cached result if it exists in the cache.
    /// Otherwise, it calls a function with `arg` and returns the result.
    pub fn call(&mut self, arg: &'a T) -> &R {
        if self.results.get(arg).is_none() {
            let result = (self.function)(arg);
            self.results.insert(arg, result);
        }
        &self.results[arg]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_cacher_i32() {
        let mut cacher = FnCacher::new(|&x| x);
        let cases = vec![(1, 1), (2, 2), (3, 3)];
        cases.iter().for_each(|case| assert_eq!(&case.1, cacher.call(&case.0)));

        let mut cacher = FnCacher::new(|x| x * x);
        let cases = vec![(1, 1), (2, 4), (3, 9)];
        cases.iter().for_each(|case| assert_eq!(&case.1, cacher.call(&case.0)));
    }

    #[test]
    fn fn_cacher_string() {
        let mut cacher = FnCacher::new(|x: &String| x.clone());
        let cases = vec![
            ("a".to_string(), "a".to_string()),
            ("b".to_string(), "b".to_string()),
            ("c".to_string(), "c".to_string()),
        ];
        cases.iter().for_each(|case| assert_eq!(&case.1, cacher.call(&case.0)));

        let mut cacher = FnCacher::new(|x: &String| x.len());
        let cases = vec![
            ("a".to_string(), 1),
            ("bb".to_string(), 2),
            ("ccc".to_string(), 3),
        ];
        cases.iter().for_each(|case| assert_eq!(&case.1, cacher.call(&case.0)));
    }
}