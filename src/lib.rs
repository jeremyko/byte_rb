//! Byte Ring Buffer
//!
//! The `byte_rb` crate defines a BrBuffer struct([`BrBuffer`](brb/struct.BrBuffer.html))
//! which implements an intuitive and easy-to-use byte ring buffer.
//! It internally allocates fixed length memory and performs memcpy.
//! # Example
//! ```rust
//! use byte_rb::BrBuffer;
//!
//! let mut cbuf = BrBuffer::new(6);
//!
//! assert!(cbuf.append(6, b"123456").unwrap());
//! assert_eq!(cbuf.rpos(), 0);
//! assert_eq!(cbuf.wpos(), 6);
//! // "123456"
//! let result = cbuf.get(3).unwrap();
//! assert_eq!(result, b"123");
//! assert_eq!(cbuf.cumulated_len(), 3);
//! assert_eq!(cbuf.rpos(), 3);
//! assert_eq!(cbuf.wpos(), 6);
//! // "  456"
//!
//! assert!(cbuf.append(3, b"789").unwrap());
//! assert_eq!(cbuf.cumulated_len(), 6);
//! assert_eq!(cbuf.rpos(), 3);
//! assert_eq!(cbuf.wpos(), 3);
//! // "789456"
//!
//! let result = cbuf.get(1).unwrap();
//! assert_eq!(result, b"4");
//! assert_eq!(cbuf.rpos(), 4);
//! assert_eq!(cbuf.wpos(), 3);
//! assert_eq!(cbuf.cumulated_len(), 5);
//! // "789 56"
//!
//! let result = cbuf.get(5).unwrap();
//! assert_eq!(result, b"56789");
//! assert_eq!(cbuf.rpos(), 3);
//! assert_eq!(cbuf.wpos(), 3);
//! assert_eq!(cbuf.cumulated_len(), 0);
//!
//! assert_eq!(cbuf.capacity(), 6);
//! ```

pub use self::brb::BrBuffer;
pub use self::brb::ERR_STR_BUFFER_FULL;
pub use self::brb::ERR_STR_INVALID_LEN;

pub mod brb;
