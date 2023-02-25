//==============================================================================
macro_rules! dbg_log {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}
pub const ERR_STR_INVALID_LEN: &str = "invalid length";
pub const ERR_STR_BUFFER_FULL: &str = "buffer is full";
//==============================================================================
pub struct BrBuffer {
    buffer: Vec<u8>,
    buffer_len: usize,
    rpos: usize,
    wpos: usize,
    cumulated_len: usize,
    contiguous: Vec<u8>,
}

//==============================================================================
impl BrBuffer {
    //--------------------------------------------------------------------------
    /// Create buffer by the length of `len`.
    pub fn new(len: usize) -> Self {
        BrBuffer {
            buffer: vec![0; len],
            buffer_len: len,
            rpos: 0,
            wpos: 0,
            cumulated_len: 0,
            contiguous: vec![],
        }
    }

    //--------------------------------------------------------------------------
    /// Copy `data` to `self` by the length of `len`.
    ///
    /// Returns `Ok(true)` on success, otherwise returns an error.
    /// # Errors
    ///
    /// | values  |
    /// | --- |
    /// | Err((-10001,[`byte_rb::ERR_STR_INVALID_LEN`](ERR_STR_INVALID_LEN) )) |
    /// | Err((-10002,[`byte_rb::ERR_STR_BUFFER_FULL`](ERR_STR_BUFFER_FULL) )) |
    /// | Err((-10003,[`byte_rb::ERR_STR_BUFFER_FULL`](ERR_STR_BUFFER_FULL) )) |
    /// | Err((-10004,[`byte_rb::ERR_STR_BUFFER_FULL`](ERR_STR_BUFFER_FULL) )) |

    pub fn append(&mut self, len: usize, data: &[u8]) -> Result<bool, (i32, &str)> {
        self.debug_me(Some(data), len, "append before", line!());
        if self.buffer_len < len {
            eprintln!("error ({}) -> {}", line!(), ERR_STR_INVALID_LEN);
            return Err((-10001, ERR_STR_INVALID_LEN));
        } else if self.buffer_len == self.cumulated_len {
            eprintln!("error ({}) -> {}", line!(), ERR_STR_BUFFER_FULL);
            return Err((-10002, ERR_STR_BUFFER_FULL));
        }
        if self.rpos > self.wpos {
            if self.rpos - self.wpos >= len {
                dbg_log!("append : rpos > wpos (reversed and free space exists)");
                self.buffer[self.wpos..(self.wpos + len)].copy_from_slice(data);
                self.wpos += len;
                self.cumulated_len += len;
                self.debug_me(Some(data), len, "append after", line!());
                Ok(true)
            } else {
                eprintln!("error ({}) -> {}", line!(), ERR_STR_BUFFER_FULL);
                Err((-10003, ERR_STR_BUFFER_FULL))
            }
        } else {
            dbg_log!("append : wpos >= rpos (not reversed)");
            if self.buffer_len < self.wpos + len {
                dbg_log!("append : wpos 이후 버퍼로 모자라는 경우");
                if (self.wpos > 0) && (len - (self.buffer_len - self.wpos) <= self.rpos) {
                    dbg_log!("append  : 2번 나누어서 들어갈 공간이 있는 경우");
                    let first_block_len = self.buffer_len - self.wpos;
                    let second_block_len = len - first_block_len;
                    dbg_log!("first {},second {}", first_block_len, second_block_len);
                    if first_block_len > 0 {
                        self.buffer[self.wpos..self.wpos + first_block_len]
                            .copy_from_slice(&data[..first_block_len]);
                    }
                    self.buffer[..second_block_len].copy_from_slice(
                        &data[first_block_len..first_block_len + second_block_len],
                    );
                    self.wpos = second_block_len;
                    self.cumulated_len += len;
                    self.debug_me(Some(data), len, "append after", line!());
                    Ok(true)
                } else {
                    dbg_log!("append : 2번 나누어서 들어갈 공간 없는 경우");
                    eprintln!("error ({}) -> {}", line!(), ERR_STR_BUFFER_FULL);
                    Err((-10004, ERR_STR_BUFFER_FULL))
                }
            } else {
                dbg_log!("append : most general case");
                self.buffer[self.wpos..(self.wpos + len)].copy_from_slice(data);
                self.wpos += len;
                self.cumulated_len += len;
                self.debug_me(Some(data), len, "append after", line!());
                Ok(true)
            }
        }
    }

    //--------------------------------------------------------------------------
    /// Returns byte slice by the length of `len` on success, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// | values  |
    /// | --- |
    /// | Err((-20001,[`byte_rb::ERR_STR_INVALID_LEN`](ERR_STR_INVALID_LEN) )) |
    /// | Err((-20002,[`byte_rb::ERR_STR_BUFFER_FULL`](ERR_STR_BUFFER_FULL) )) |
    pub fn get(&mut self, len: usize) -> Result<&[u8], (i32, &str)> {
        self.debug_me(None, len, "get before", line!());

        if self.wpos > self.rpos {
            if self.wpos < self.rpos + len {
                eprintln!("error ({}) -> {}", line!(), ERR_STR_INVALID_LEN);
                Err((-20001, ERR_STR_INVALID_LEN))
            } else {
                dbg_log!("get : general case");
                let rslt_slice = &self.buffer[self.rpos..(self.rpos + len)];
                self.rpos += len;
                self.cumulated_len -= len;
                self.debug_me(None, len, "get after", line!());
                Ok(rslt_slice)
            }
        } else {
            dbg_log!("get : reversed : rpos >= wpos ");
            if self.buffer_len < self.rpos + len {
                let first_block_len = self.buffer_len - self.rpos;
                let second_block_len = len - first_block_len;
                dbg_log!("{},{}", first_block_len, second_block_len);
                if self.wpos > 0 && self.wpos >= second_block_len {
                    // to combine 2 parts
                    dbg_log!("get : reversed (need to combine 2 parts)");
                    let slice1 = &self.buffer[self.rpos..self.rpos + first_block_len];
                    let slice2 = &self.buffer[..second_block_len];
                    self.rpos = second_block_len;
                    self.cumulated_len -= len;
                    //XXX
                    self.contiguous = [slice1, slice2].concat();
                    self.debug_me(None, len, "get after", line!());
                    Ok(self.contiguous.as_slice())
                } else {
                    eprintln!("error ({}) -> {}", line!(), ERR_STR_INVALID_LEN);
                    Err((-20002, ERR_STR_INVALID_LEN))
                }
            } else {
                dbg_log!("get : reversed (no need to combine)");
                let rslt_slice = &self.buffer[self.rpos..(self.rpos + len)];
                self.rpos += len;
                self.cumulated_len -= len;
                self.debug_me(None, len, "get after", line!());
                Ok(rslt_slice)
            }
        }
    }

    //--------------------------------------------------------------------------
    /// Returns allocated total buffer length
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
    //--------------------------------------------------------------------------
    /// Returns cumulated total user data length
    pub fn cumulated_len(&self) -> usize {
        self.cumulated_len
    }
    //--------------------------------------------------------------------------
    /// Returns current read position of buffer
    pub fn rpos(&self) -> usize {
        self.rpos
    }
    //--------------------------------------------------------------------------
    /// Returns current write position of buffer
    pub fn wpos(&self) -> usize {
        self.wpos
    }
    //--------------------------------------------------------------------------
    #[cfg(debug_assertions)]
    fn debug_me(&self, _data: Option<&[u8]>, _data_len: usize, _msg: &str, _fline: u32) {
        println!(
            "{} ({}) : {:?}, buffer_len={}, len={}, rpos={}, wpos={}, cumulated={}",
            _msg,
            _fline,
            _data,
            self.buffer_len,
            _data_len,
            self.rpos,
            self.wpos,
            self.cumulated_len
        );
    }
    #[cfg(not(debug_assertions))]
    fn debug_me(&self, _data: Option<&[u8]>, _data_len: usize, _msg: &str, _fline: u32) {}
}
