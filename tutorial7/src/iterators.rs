/*
    Iterators in Rust

    Recall that every for loop is internally an iterator:
*/

use std::iter;

pub fn example_for() {
    let v = vec![1, 2, 3];
    for &x in &v {
        println!("{}", x);
    }
}

pub fn example_iter() {
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x);
    }
}

// More explicitly...
// Iterators are objects which expose a .next() method.
// This returns either Some(next_val) or None.
pub fn example_iter_explicit1() {
    let v = vec![1, 2, 3];
    let mut iterator = v.iter();
    loop {
        let opt = iterator.next();
        match opt {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}

// And we can also use .next() directly, not in a loop
#[test]
pub fn example_iter_explicit2() {
    let v = vec![1, 2, 3];
    let mut iterator = v.iter();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());

    // assert!(false);
}

/*
    Data structures, in particular Vec, have methods to iterate over them:
    - iter()
    - iter_mut()
    - into_iter()

    into_iter() is defined by the IntoIterator trait:
    this is what makes for loops work!
    When you use a for loop, Rust sees that the type you wrote
    in `for x in [[type]]` implements the IntoIterator trait,
    and then uses that trait to generate the right code.
*/

#[test]
pub fn example_iter_mut() {
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() {
        *x += 1;
    }
    assert_eq!(v, vec![2, 3, 4]);
}

pub fn example_into_iter() {
    // Conceptually same as example_for
    // .iter() is a method implemented for the Vec type
    //     (the name 'iter' is not special)
    // .into_iter() is a method which is part of the IntoIterator trait.
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x);
    }
}

// Other data structures...
// HashMap
// HashSet
// ...
// .iter()

/*
    Programming with Iterators!

    Very rich API:
    https://doc.rust-lang.org/std/iter/trait.Iterator.html

    Iterators can be used to write code in a more functional style.

    EXERCISES:
    1. Check if all elements in a vector are below a threshold
    2. Return a vector of only the elements below the threshold
    3. Copy the nth element n times
    4. Pad/truncate a vector to exactly a certain length
    5. Sum all the squares less than n
*/

pub fn all_below(v: Vec<usize>, thresh: usize) -> bool {
    // Could write a for loop...
    // Could also write it in a functional style with iterators
    v.iter().all(|&ele| ele < thresh)
}

#[test]
fn test_all_below() {
    assert!(all_below(vec![0, 1, 3], 4));
    assert!(!all_below(vec![0, 1, 3], 3));
}

pub fn filter_below(v: Vec<usize>, thresh: usize) -> Vec<usize> {
    // This will fail -- I'm returning an iterator instead of a vector
    // v.iter().filter(|&ele| ele < thresh)

    // Fix: "collect" the iterator into a vector
    // Unfortunately, doubule references come up when using .filter()
    // v.iter().filter(|&&ele| ele < thresh).copied().collect()

    // Alternative
    v.iter().copied().filter(|&ele| ele < thresh).collect()
}

// Reminder: .copied() creates an iterator of values T from an iterator of
// references &T.

/*
    ========== Start of Lecture 7 Part 3 (short additional video) ==========
*/

pub fn copy_increasing(v: Vec<usize>) -> Vec<usize> {
    // [1, 2, 3].iter().flat_map(|...| {})
    v.iter()
        .enumerate()
        .flat_map(|(i, ele)| iter::repeat(ele).take(i))
        .copied()
        .collect()
}

pub fn pad_truncate(v: Vec<usize>, target_length: usize) -> Vec<usize> {
    // If v.len() < n, pad with 0s
    // If v.len() > n, take first n elements
    // Instead of using an if block, we can do this directly with iterators
    let result: Vec<usize> = v
        .iter()
        .copied()
        .chain(iter::repeat(0))
        // Note: if we called .collect() here we'd get an infinite loop
        .take(target_length)
        .collect();

    // Sanity check
    debug_assert_eq!(result.len(), target_length);
    result
}

pub fn sum_squares_lt(n: usize) -> usize {
    // (0..) : an iterator over all nonnegative integers
    // Note: .filter instead of .take_while leads to infinite loop :(
    (0..).map(|x| x * x).take_while(|&x| x < n).sum()
}

#[test]
fn test_sum_squares_lt() {
    assert_eq!(sum_squares_lt(4), 1);
    assert_eq!(sum_squares_lt(5), 1 + 4);
    assert_eq!(sum_squares_lt(30), 1 + 4 + 9 + 16 + 25);
}

// Recap: particularly useful methods in the Iterator trait are:
//     .map(), .take(), .take_while(), .filter()

/*
    We can also write functions that manipulate iterators directly.

    Exercise: implement some of the above using iterators as input/output.

    But what should the return values be?

    Iterators constructed via .map, .filter, etc. all create custom structs:
    you'll find (if you try to get Rust to typecheck them) dedicated structs
    called Map, Filter, etc. corresponding to each operation.

    But we don't really want to tell the compiler the exact return type
    if we are returning an iterator...

    Example return type in copy_increasing:
        Copied<FlatMap<Enumerate<std::slice::Iter<'_, usize>>,
        std::iter::Take<std::iter::Repeat<&usize>>,
        [closure@src/iterators.rs:142:19: 144:10]>>
    D:

    - Here the impl Trait syntax comes in handy again.

    Instead: we just write 'impl Iterator<Item = usize>'
*/

pub fn copy_increasing_iter1(v: &[usize]) -> impl Iterator<Item = usize> + '_ {
    v.iter().enumerate().flat_map(|(i, ele)| iter::repeat(ele).take(i)).copied()
}

// Note that the iterator needs to live as long as the input data v: &[usize]
// So we use '_ to tell Rust to figure out the appropriate lifetime.
// Alternatively we can write a function where both input/output are iterators:

pub fn copy_increasing_iter2(
    v: impl Iterator<Item = usize>,
) -> impl Iterator<Item = usize> {
    v.enumerate().flat_map(|(i, ele)| iter::repeat(ele).take(i))
}

// Could also for the input do it with a type argument T and a trait bound
// T: Iterator<Item = usize>,
// but this doesn't work for the return type, as we saw with Fn traits.

/*
    Writing iterators for your own data types

    There are two ways:

    - Easy way (if it suits your purposes):
      write a method returning `impl Iterator`

    - Slightly harder way: manually implement the Iterator trait

      Warning: you don't implement Iterator for your type!
      This is a bit of a "gotcha" in Rust
      You implement it for a dedicated iterator type.
      Why is this?

    Example:
*/

#[derive(Clone, Debug)]
pub struct SongName(String);

#[allow(unused)]
#[derive(Debug)]
pub struct SongUserProfile {
    username: String,
    liked_songs: Vec<SongName>,
    disliked_songs: Vec<SongName>,
    listens: usize,
    days_active: usize,
}
impl SongUserProfile {
    // Better to return an iterator than a vector -- why?
    // Because the user could keep playing songs forever, and
    // the next song might not just be static, it might be variable
    // depending on what song we want to play next (e.g. depending on updates to
    // the available songs, liked songs, etc.)
    pub fn play_songs(&self) -> impl Iterator<Item = SongName> + '_ {
        // What should go here?
        // Simple idea: return an iterator over liked songs
        // self.liked_songs.iter() gives an iterator over references
        // So we want to call .cloned() to get an iterator over values
        self.liked_songs.iter().cloned()

        // This is a finite iterator; we could also play songs repeatedly:
        // self.liked_songs.iter().cloned().cycle()
    }
}

/*
    Second way: implementing a dedicated iterator object

    This is what standard library traits do! See for example:
    - The std::slice::Iter struct:
      https://doc.rust-lang.org/std/slice/struct.Iter.html
    - The std::collections::hash_map::Iter struct:
      https://doc.rust-lang.org/std/collections/hash_map/struct.Iter.html

    These are NOT the same as Iterator, types, not traits.
    They are just called Iter out of convenience.
*/

pub struct SongIterator<'a> {
    user_profile: &'a SongUserProfile,
    current_song_index: usize,
    // For song index, could also use a reference (&SongName),
    // but this avoids lifetime issues, so a bit simpler
}

impl<'a> Iterator for SongIterator<'a> {
    type Item = SongName;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_song_index == self.user_profile.liked_songs.len() {
            None
        } else {
            let result =
                self.user_profile.liked_songs[self.current_song_index].clone();
            self.current_song_index += 1;
            Some(result)
        }
    }
}

// Finally we need a way to connect our SongUserProfile somehow
// with the SongIterator object
// This is what the standard library containers like Vec and HashMap have
// a .iter() method for

impl SongUserProfile {
    pub fn get_iter(&self) -> SongIterator {
        SongIterator { user_profile: &self, current_song_index: 0 }
    }
}

/*
    Recap:

    - We have a custom data structure SongUserProfile
    - We implemented a separate iterator type, SongIterator, that can be used
      to generate songs to play, using the profile
    - We implemented the Iterator trait for SongIterator
        (NOT for SongUserProfile!)
    - Finally, we made a method on SongUserProfile that initializes a
      SongIterator to iterate over the liked songs in the user profile.

    This is the general recipe for implementing an iterator over a custom
    data structure.
*/
