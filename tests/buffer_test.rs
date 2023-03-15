use byte_rb::BrBuffer;
//==============================================================================
#[test]
fn create() {
    let cbuf = BrBuffer::new(10);
    assert_eq!(cbuf.capacity(), 10);
    assert_eq!(cbuf.rpos(), 0);
    assert_eq!(cbuf.wpos(), 0);
}

//==============================================================================
#[test]
fn basic_contiguous() {
    let mut cbuf = BrBuffer::new(6);
    assert_eq!(cbuf.capacity(), 6);

    assert!(cbuf.append(3, b"123").unwrap());
    assert_eq!(cbuf.wpos(), 3);
    assert!(cbuf.append(3, b"456").unwrap());
    assert_eq!(cbuf.wpos(), 6);
    let result = cbuf.get(6).unwrap();
    assert_eq!(result, b"123456");
    assert_eq!(cbuf.rpos(), 6);

    assert!(cbuf.append(3, b"abc").unwrap());
    assert_eq!(cbuf.wpos(), 3);
    assert!(cbuf.append(3, b"def").unwrap());
    assert_eq!(cbuf.wpos(), 6);
    let result = cbuf.get(6).unwrap();
    assert_eq!(result, b"abcdef");

    assert_eq!(cbuf.capacity(), 6);
    println!("{:?}", cbuf);
}

//==============================================================================
#[test]
fn non_contiguous() {
    let mut cbuf = BrBuffer::new(6);

    assert!(cbuf.append(6, b"123456").unwrap());
    assert_eq!(cbuf.rpos(), 0);
    assert_eq!(cbuf.wpos(), 6);
    // "123456"
    let result = cbuf.peek(3).unwrap();
    assert_eq!(result, b"123");
    assert_eq!(cbuf.cumulated_len(), 6);

    let result = cbuf.get(3).unwrap();
    assert_eq!(result, b"123");
    assert_eq!(cbuf.cumulated_len(), 3);
    assert_eq!(cbuf.rpos(), 3);
    assert_eq!(cbuf.wpos(), 6);
    // "  456"

    assert!(cbuf.append(3, b"789").unwrap());
    assert_eq!(cbuf.cumulated_len(), 6);
    assert_eq!(cbuf.rpos(), 3);
    assert_eq!(cbuf.wpos(), 3);
    // "789456"

    let result = cbuf.get(1).unwrap();
    assert_eq!(result, b"4");
    assert_eq!(cbuf.rpos(), 4);
    assert_eq!(cbuf.wpos(), 3);
    assert_eq!(cbuf.cumulated_len(), 5);
    // "789 56"

    let result = cbuf.get(5).unwrap();
    assert_eq!(result, b"56789");
    assert_eq!(cbuf.rpos(), 3);
    assert_eq!(cbuf.wpos(), 3);
    assert_eq!(cbuf.cumulated_len(), 0);

    assert_eq!(cbuf.capacity(), 6);
    println!("{:?}", cbuf);
}

//==============================================================================
#[test]
fn too_long() {
    let mut cbuf = BrBuffer::new(2);
    let err_rslt = cbuf.append(3, b"123");
    assert_eq!(err_rslt, Err((-10001, byte_rb::ERR_STR_INVALID_LEN)));
    println!("{:?}", cbuf);
}

//==============================================================================
#[test]
fn buffer_full() {
    let mut cbuf = BrBuffer::new(3);
    assert!(cbuf.append(3, b"123").unwrap());
    let err_rslt = cbuf.append(1, b"4");
    assert_eq!(err_rslt, Err((-10002, byte_rb::ERR_STR_BUFFER_FULL)));
    println!("{:?}", cbuf);
}

//==============================================================================
#[test]
fn buffer_full_non_contiguous() {
    let mut cbuf = BrBuffer::new(6);

    assert!(cbuf.append(6, b"123456").unwrap());
    assert_eq!(cbuf.rpos(), 0);
    assert_eq!(cbuf.wpos(), 6);
    // "123456"
    let result = cbuf.get(2).unwrap();
    assert_eq!(result, b"12");
    assert_eq!(cbuf.cumulated_len(), 4);
    assert_eq!(cbuf.rpos(), 2);
    assert_eq!(cbuf.wpos(), 6);
    // " 3456"

    // assert!(cbuf.append(4, b"789A").is_err());
    let err_rslt = cbuf.append(4, b"789A");
    assert_eq!(err_rslt, Err((-10004, byte_rb::ERR_STR_BUFFER_FULL)));

    assert!(cbuf.append(1, b"7").unwrap());
    // "7 3456"
    let result = cbuf.peek(5).unwrap();
    assert_eq!(result, b"34567");

    assert_eq!(cbuf.cumulated_len(), 5);
    assert_eq!(cbuf.rpos(), 2);
    assert_eq!(cbuf.wpos(), 1);

    let err_rslt = cbuf.append(2, b"89");
    assert_eq!(err_rslt, Err((-10003, byte_rb::ERR_STR_BUFFER_FULL)));

    let result = cbuf.get(5).unwrap();
    assert_eq!(result, b"34567");
    assert_eq!(cbuf.cumulated_len(), 0);

    println!("{:?}", cbuf);
}
