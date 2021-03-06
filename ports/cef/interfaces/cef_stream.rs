// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Structure the client can implement to provide a custom stream reader. The
// functions of this structure may be called on any thread.
//
#[repr(C)]
pub struct _cef_read_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Read raw binary data.
  //
  pub read: Option<extern "C" fn(this: *mut cef_read_handler_t, ptr: *mut (),
      size: libc::size_t, n: libc::size_t) -> libc::size_t>,

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Return zero on success and non-zero on failure.
  //
  pub seek: Option<extern "C" fn(this: *mut cef_read_handler_t, offset: i64,
      whence: libc::c_int) -> libc::c_int>,

  //
  // Return the current offset position.
  //
  pub tell: Option<extern "C" fn(this: *mut cef_read_handler_t) -> i64>,

  //
  // Return non-zero if at end of file.
  //
  pub eof: Option<extern "C" fn(this: *mut cef_read_handler_t) -> libc::c_int>,

  //
  // Return true (1) if this handler performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the handler from.
  //
  pub may_block: Option<extern "C" fn(
      this: *mut cef_read_handler_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_read_handler_t = _cef_read_handler_t;


//
// Structure the client can implement to provide a custom stream reader. The
// functions of this structure may be called on any thread.
//
pub struct CefReadHandler {
  c_object: *mut cef_read_handler_t,
}

impl Clone for CefReadHandler {
  fn clone(&self) -> CefReadHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefReadHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefReadHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefReadHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_read_handler_t) -> CefReadHandler {
    CefReadHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_read_handler_t) -> CefReadHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefReadHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_read_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_read_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Read raw binary data.
  //
  pub fn read(&self, ptr: &mut (), size: libc::size_t,
      n: libc::size_t) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).read.unwrap())(
          self.c_object,
          CefWrap::to_c(ptr),
          CefWrap::to_c(size),
          CefWrap::to_c(n)))
    }
  }

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Return zero on success and non-zero on failure.
  //
  pub fn seek(&self, offset: i64, whence: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).seek.unwrap())(
          self.c_object,
          CefWrap::to_c(offset),
          CefWrap::to_c(whence)))
    }
  }

  //
  // Return the current offset position.
  //
  pub fn tell(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).tell.unwrap())(
          self.c_object))
    }
  }

  //
  // Return non-zero if at end of file.
  //
  pub fn eof(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).eof.unwrap())(
          self.c_object))
    }
  }

  //
  // Return true (1) if this handler performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the handler from.
  //
  pub fn may_block(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).may_block.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_read_handler_t> for CefReadHandler {
  fn to_c(rust_object: CefReadHandler) -> *mut cef_read_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_read_handler_t) -> CefReadHandler {
    CefReadHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_read_handler_t> for Option<CefReadHandler> {
  fn to_c(rust_object: Option<CefReadHandler>) -> *mut cef_read_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_read_handler_t) -> Option<CefReadHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefReadHandler::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to read data from a stream. The functions of this structure
// may be called on any thread.
//
#[repr(C)]
pub struct _cef_stream_reader_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Read raw binary data.
  //
  pub read: Option<extern "C" fn(this: *mut cef_stream_reader_t, ptr: *mut (),
      size: libc::size_t, n: libc::size_t) -> libc::size_t>,

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Returns zero on success and non-zero on failure.
  //
  pub seek: Option<extern "C" fn(this: *mut cef_stream_reader_t, offset: i64,
      whence: libc::c_int) -> libc::c_int>,

  //
  // Return the current offset position.
  //
  pub tell: Option<extern "C" fn(this: *mut cef_stream_reader_t) -> i64>,

  //
  // Return non-zero if at end of file.
  //
  pub eof: Option<extern "C" fn(this: *mut cef_stream_reader_t) -> libc::c_int>,

  //
  // Returns true (1) if this reader performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the reader from.
  //
  pub may_block: Option<extern "C" fn(
      this: *mut cef_stream_reader_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_stream_reader_t = _cef_stream_reader_t;


//
// Structure used to read data from a stream. The functions of this structure
// may be called on any thread.
//
pub struct CefStreamReader {
  c_object: *mut cef_stream_reader_t,
}

impl Clone for CefStreamReader {
  fn clone(&self) -> CefStreamReader{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefStreamReader {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefStreamReader {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefStreamReader {
  pub unsafe fn from_c_object(c_object: *mut cef_stream_reader_t) -> CefStreamReader {
    CefStreamReader {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_stream_reader_t) -> CefStreamReader {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefStreamReader {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_stream_reader_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_stream_reader_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Read raw binary data.
  //
  pub fn read(&self, ptr: &mut (), size: libc::size_t,
      n: libc::size_t) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).read.unwrap())(
          self.c_object,
          CefWrap::to_c(ptr),
          CefWrap::to_c(size),
          CefWrap::to_c(n)))
    }
  }

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Returns zero on success and non-zero on failure.
  //
  pub fn seek(&self, offset: i64, whence: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).seek.unwrap())(
          self.c_object,
          CefWrap::to_c(offset),
          CefWrap::to_c(whence)))
    }
  }

  //
  // Return the current offset position.
  //
  pub fn tell(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).tell.unwrap())(
          self.c_object))
    }
  }

  //
  // Return non-zero if at end of file.
  //
  pub fn eof(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).eof.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this reader performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the reader from.
  //
  pub fn may_block(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).may_block.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_stream_reader_t object from a file.
  //
  pub fn create_for_file(fileName: &[u16]) -> interfaces::CefStreamReader {
    unsafe {
      CefWrap::to_rust(
        ::stream::cef_stream_reader_create_for_file(
          CefWrap::to_c(fileName)))
    }
  }

  //
  // Create a new cef_stream_reader_t object from data.
  //
  pub fn create_for_data(data: &mut (),
      size: libc::size_t) -> interfaces::CefStreamReader {
    unsafe {
      CefWrap::to_rust(
        ::stream::cef_stream_reader_create_for_data(
          CefWrap::to_c(data),
          CefWrap::to_c(size)))
    }
  }

  //
  // Create a new cef_stream_reader_t object from a custom handler.
  //
  pub fn create_for_handler(
      handler: interfaces::CefReadHandler) -> interfaces::CefStreamReader {
    unsafe {
      CefWrap::to_rust(
        ::stream::cef_stream_reader_create_for_handler(
          CefWrap::to_c(handler)))
    }
  }
} 

impl CefWrap<*mut cef_stream_reader_t> for CefStreamReader {
  fn to_c(rust_object: CefStreamReader) -> *mut cef_stream_reader_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_stream_reader_t) -> CefStreamReader {
    CefStreamReader::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_stream_reader_t> for Option<CefStreamReader> {
  fn to_c(rust_object: Option<CefStreamReader>) -> *mut cef_stream_reader_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_stream_reader_t) -> Option<CefStreamReader> {
    if c_object.is_null() {
      None
    } else {
      Some(CefStreamReader::from_c_object_addref(c_object))
    }
  }
}


//
// Structure the client can implement to provide a custom stream writer. The
// functions of this structure may be called on any thread.
//
#[repr(C)]
pub struct _cef_write_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Write raw binary data.
  //
  pub write: Option<extern "C" fn(this: *mut cef_write_handler_t, ptr: *const (
      ), size: libc::size_t, n: libc::size_t) -> libc::size_t>,

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Return zero on success and non-zero on failure.
  //
  pub seek: Option<extern "C" fn(this: *mut cef_write_handler_t, offset: i64,
      whence: libc::c_int) -> libc::c_int>,

  //
  // Return the current offset position.
  //
  pub tell: Option<extern "C" fn(this: *mut cef_write_handler_t) -> i64>,

  //
  // Flush the stream.
  //
  pub flush: Option<extern "C" fn(
      this: *mut cef_write_handler_t) -> libc::c_int>,

  //
  // Return true (1) if this handler performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the handler from.
  //
  pub may_block: Option<extern "C" fn(
      this: *mut cef_write_handler_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_write_handler_t = _cef_write_handler_t;


//
// Structure the client can implement to provide a custom stream writer. The
// functions of this structure may be called on any thread.
//
pub struct CefWriteHandler {
  c_object: *mut cef_write_handler_t,
}

impl Clone for CefWriteHandler {
  fn clone(&self) -> CefWriteHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefWriteHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefWriteHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefWriteHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_write_handler_t) -> CefWriteHandler {
    CefWriteHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_write_handler_t) -> CefWriteHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefWriteHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_write_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_write_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Write raw binary data.
  //
  pub fn write(&self, ptr: &(), size: libc::size_t,
      n: libc::size_t) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).write.unwrap())(
          self.c_object,
          CefWrap::to_c(ptr),
          CefWrap::to_c(size),
          CefWrap::to_c(n)))
    }
  }

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Return zero on success and non-zero on failure.
  //
  pub fn seek(&self, offset: i64, whence: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).seek.unwrap())(
          self.c_object,
          CefWrap::to_c(offset),
          CefWrap::to_c(whence)))
    }
  }

  //
  // Return the current offset position.
  //
  pub fn tell(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).tell.unwrap())(
          self.c_object))
    }
  }

  //
  // Flush the stream.
  //
  pub fn flush(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).flush.unwrap())(
          self.c_object))
    }
  }

  //
  // Return true (1) if this handler performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the handler from.
  //
  pub fn may_block(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).may_block.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_write_handler_t> for CefWriteHandler {
  fn to_c(rust_object: CefWriteHandler) -> *mut cef_write_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_write_handler_t) -> CefWriteHandler {
    CefWriteHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_write_handler_t> for Option<CefWriteHandler> {
  fn to_c(rust_object: Option<CefWriteHandler>) -> *mut cef_write_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_write_handler_t) -> Option<CefWriteHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefWriteHandler::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to write data to a stream. The functions of this structure may
// be called on any thread.
//
#[repr(C)]
pub struct _cef_stream_writer_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Write raw binary data.
  //
  pub write: Option<extern "C" fn(this: *mut cef_stream_writer_t, ptr: *const (
      ), size: libc::size_t, n: libc::size_t) -> libc::size_t>,

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Returns zero on success and non-zero on failure.
  //
  pub seek: Option<extern "C" fn(this: *mut cef_stream_writer_t, offset: i64,
      whence: libc::c_int) -> libc::c_int>,

  //
  // Return the current offset position.
  //
  pub tell: Option<extern "C" fn(this: *mut cef_stream_writer_t) -> i64>,

  //
  // Flush the stream.
  //
  pub flush: Option<extern "C" fn(
      this: *mut cef_stream_writer_t) -> libc::c_int>,

  //
  // Returns true (1) if this writer performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the writer from.
  //
  pub may_block: Option<extern "C" fn(
      this: *mut cef_stream_writer_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_stream_writer_t = _cef_stream_writer_t;


//
// Structure used to write data to a stream. The functions of this structure may
// be called on any thread.
//
pub struct CefStreamWriter {
  c_object: *mut cef_stream_writer_t,
}

impl Clone for CefStreamWriter {
  fn clone(&self) -> CefStreamWriter{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefStreamWriter {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefStreamWriter {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefStreamWriter {
  pub unsafe fn from_c_object(c_object: *mut cef_stream_writer_t) -> CefStreamWriter {
    CefStreamWriter {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_stream_writer_t) -> CefStreamWriter {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefStreamWriter {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_stream_writer_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_stream_writer_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Write raw binary data.
  //
  pub fn write(&self, ptr: &(), size: libc::size_t,
      n: libc::size_t) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).write.unwrap())(
          self.c_object,
          CefWrap::to_c(ptr),
          CefWrap::to_c(size),
          CefWrap::to_c(n)))
    }
  }

  //
  // Seek to the specified offset position. |whence| may be any one of SEEK_CUR,
  // SEEK_END or SEEK_SET. Returns zero on success and non-zero on failure.
  //
  pub fn seek(&self, offset: i64, whence: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).seek.unwrap())(
          self.c_object,
          CefWrap::to_c(offset),
          CefWrap::to_c(whence)))
    }
  }

  //
  // Return the current offset position.
  //
  pub fn tell(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).tell.unwrap())(
          self.c_object))
    }
  }

  //
  // Flush the stream.
  //
  pub fn flush(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).flush.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this writer performs work like accessing the file
  // system which may block. Used as a hint for determining the thread to access
  // the writer from.
  //
  pub fn may_block(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).may_block.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_stream_writer_t object for a file.
  //
  pub fn create_for_file(fileName: &[u16]) -> interfaces::CefStreamWriter {
    unsafe {
      CefWrap::to_rust(
        ::stream::cef_stream_writer_create_for_file(
          CefWrap::to_c(fileName)))
    }
  }

  //
  // Create a new cef_stream_writer_t object for a custom handler.
  //
  pub fn create_for_handler(
      handler: interfaces::CefWriteHandler) -> interfaces::CefStreamWriter {
    unsafe {
      CefWrap::to_rust(
        ::stream::cef_stream_writer_create_for_handler(
          CefWrap::to_c(handler)))
    }
  }
} 

impl CefWrap<*mut cef_stream_writer_t> for CefStreamWriter {
  fn to_c(rust_object: CefStreamWriter) -> *mut cef_stream_writer_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_stream_writer_t) -> CefStreamWriter {
    CefStreamWriter::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_stream_writer_t> for Option<CefStreamWriter> {
  fn to_c(rust_object: Option<CefStreamWriter>) -> *mut cef_stream_writer_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_stream_writer_t) -> Option<CefStreamWriter> {
    if c_object.is_null() {
      None
    } else {
      Some(CefStreamWriter::from_c_object_addref(c_object))
    }
  }
}

