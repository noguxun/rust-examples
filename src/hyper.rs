#[test]
fn hyper01() {
    let v = vec![b'1', b'2', b'3', b'4'];
    let full: http_body_util::Full<bytes::Bytes> = v.into();
    dbg!(&full);

    let s = "5678";
    let full: http_body_util::Full<bytes::Bytes> = s.into();
    dbg!(&full);
}
