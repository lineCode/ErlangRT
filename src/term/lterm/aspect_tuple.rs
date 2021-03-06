//! Do not import this file directly, use `use term::lterm::*;` instead.

use rt_defs::{Word};
use term::immediate;
use term::lterm::*;
use term::primary;
use term::raw::*;


pub trait TupleAspect {
  /// Get a proxy object for read-only accesing the cons contents.
  unsafe fn raw_tuple(&self) -> rtuple::Ptr;

  /// Get a proxy object for looking and modifying cons contents.
  unsafe fn raw_tuple_mut(&self) -> rtuple::PtrMut;

  /// Create an empty tuple value.
  fn empty_tuple() -> super::LTerm;

  /// Check whether a value is a tuple on heap or an empty tuple.
  unsafe fn is_tuple(&self) -> bool;

  /// Check whether a value is an empty tuple.
  fn is_empty_tuple(&self) -> bool;
}


impl TupleAspect for super::LTerm {

  unsafe fn is_tuple(&self) -> bool {
    if self.is_empty_tuple() { return true; }
    if !self.is_box() { return false }

    let p = self.box_ptr();
    let box_tag = primary::header::get_tag(*p);
    box_tag == primary::header::TAG_HEADER_TUPLE
  }


  /// Get a proxy object for read-only accesing the cons contents.
  unsafe fn raw_tuple(&self) -> rtuple::Ptr {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer(v);
    rtuple::Ptr::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  unsafe fn raw_tuple_mut(&self) -> rtuple::PtrMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer_mut(v);
    rtuple::PtrMut::from_pointer(boxp)
  }


  /// Create an empty tuple value.
  #[inline]
  fn empty_tuple() -> super::LTerm {
    super::LTerm { value: immediate::IMM2_SPECIAL_EMPTY_TUPLE_RAW }
  }

  /// Check whether a value is an empty tuple.
  #[inline]
  fn is_empty_tuple(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_EMPTY_TUPLE_RAW
  }
}


#[inline]
#[allow(dead_code)]
pub fn make_tuple_header(arity: Word) -> super::LTerm {
  super::LTerm { value: primary::header::make_tuple_header_raw(arity) }
}
