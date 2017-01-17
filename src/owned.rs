use std::fmt;
use std::hash::{Hash,Hasher};
use std::cmp::*;
use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};

// use clearable::Clearable;


/// Abreviation for `ClearOnDrop` composed with `Owned`
pub type ClearOwnedOnDrop<T> = ::clear_on_drop::ClearOnDrop<T, Owned<T>>;
    // where T: Copy + ?Sized;

/// Abreviation for `ClearOnDrop::new(Owned::new(_))`
#[inline(always)]
pub fn owned_clear_on_drop<T>(t: T) -> ClearOwnedOnDrop<T>
    where T: Copy + ?Sized
{
    ::clear_on_drop::ClearOnDrop::new(Owned::new(t))
}


/// Wraps an owned value so it masquerades as a reference.
///
/// In Rust, we abstract over types of borrowing using the `Borrow<T>` and
/// `BorrowMut<T>` traits, which cover both `T` as well as references like
/// `&T`.  These permit one type to be borrowed in many ways however, so
/// that they can be used more easily as keys for `HashMap`.  
/// As a consequence, they cannot provide a cannonical target for our
/// `ClearOnDrop` type.  `ToOwned` does not make this cannonical either.
/// 
/// We use `Deref` and `DerefMut` bounds for `ClearOnDrop` because they do
/// provide a single target for dereferencing, but we cannot ask for 
/// `T: Deref<Target = T>`.  As a result, `ClearOnDrop` cannot hold an
/// owned type directly.  Instead, `Owned<T>` provides a reference type
/// compatable with `ClearOnDrop` that secretly owns its referent without
/// wasting space on real reference.
///
/// In essence, `Owned<T>` provides the caller form of the functionality
/// callees provide using `Borrow<T>` and `BorrowMut<T>`.
///
/// Example
///
/// ```
/// # use clear_on_drop::ClearOnDrop;
/// # use clear_on_drop::Owned;
/// let place: *const u16;
/// {
///     let mut key = ClearOnDrop::new(Owned::new([1,2,3,4,5,6,7]));
///     key[5] = 3;
///     place = &key[0];
///     // This causes the test to fail!  FAIL!
///     // ::std::mem::drop(key);
///  } 
/// // Warning removing the above 
/// // ::std::mem::drop(key);
/// for i in 0..7 {
///    unsafe { assert_eq!(*place.offset(i), 0); }
/// }
/// ```
pub struct Owned<T>(T) where T: Copy + ?Sized;

impl<T> Owned<T> where T: Copy + ?Sized {
    /// Wrap an owned value so it masquerades as a reference.
    pub fn new(t: T) -> Owned<T> {  Owned(t)  }
}


// --- Implement pointer traits --- //

impl<T> Deref for Owned<T>
    where T: Copy + ?Sized 
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Owned<T>
    where T: Copy + ?Sized 
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Owned<T>
    where T: Copy + ?Sized
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Owned<T>
    where T: Copy + ?Sized
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Borrow<T> for Owned<T>
    where T: Copy + ?Sized
{
    #[inline]
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> BorrowMut<T> for Owned<T>
    where T: Copy + ?Sized
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Default for Owned<T>
    where T: Copy + ?Sized + Default
{
    #[inline]
    fn default() -> Self {
        Owned(Default::default())
    }
}


// --- Delegate derivable traits --- //

impl<T> fmt::Debug for Owned<T>
    where T: Copy + ?Sized + fmt::Debug
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
      {  fmt::Debug::fmt(&self.0, f)  }
}

/*
impl<T> Clone for Owned<T>
    where T: Copy + ?Sized
{
    fn clone(&self) -> Self {  Owned(self.0.clone())  }
    fn clone_from(&mut self, source: &Self) {  self.0.clone_from(&source.0);  }
}
*/

// impl<T> Copy for Owned<T> where T: Clone + Copy + ?Sized


impl<T> Hash for Owned<T>
    where T: Copy + ?Sized + Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {  self.0.hash(state);  }
}

impl<T,R> PartialEq<Owned<R>> for Owned<T>
    where T: Copy + ?Sized + PartialEq<R>,
          R: Copy + ?Sized 
{
    fn eq(&self, other: &Owned<R>) -> bool {  self.0.eq(&other.0)  }
    fn ne(&self, other: &Owned<R>) -> bool {  self.0.ne(&other.0)  }
}

impl<T> Eq for Owned<T> where T: Copy + ?Sized + PartialEq<T> + Eq { }

impl<T,R> PartialOrd<Owned<R>> for Owned<T>
    where T: Copy + ?Sized + PartialOrd<R>,
          R: Copy + ?Sized 
{
    fn partial_cmp(&self, other: &Owned<R>) -> Option<Ordering> { self.0.partial_cmp(&other.0) }

    fn lt(&self, other: &Owned<R>) -> bool {  self.0.lt(&other.0)  }
    fn le(&self, other: &Owned<R>) -> bool {  self.0.le(&other.0)  }
    fn gt(&self, other: &Owned<R>) -> bool {  self.0.gt(&other.0)  }
    fn ge(&self, other: &Owned<R>) -> bool {  self.0.ge(&other.0)  }
}

impl<T> Ord for Owned<T>
    where T: Copy + ?Sized + Ord
{
    fn cmp(&self, other: &Self) -> Ordering {  self.0.cmp(&other.0)  }
}


#[cfg(test)]
mod tests {
    use super::*;
    use clear_on_drop::ClearOnDrop;

    #[test]
    fn owned() {
        let place: *const u16;
        {
           let mut key = ClearOnDrop::new(Owned::new([1,2,3,4,5,6,7]));
           key[5] = 3;
           place = &key[0];
           // This causes the test to fail!
           // ::std::mem::drop(key);
        } 
        for i in 0..7 {
            unsafe { assert_eq!(*place.offset(i), 0); }
        }
    }
}

