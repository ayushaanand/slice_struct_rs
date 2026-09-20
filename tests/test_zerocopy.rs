#![cfg(feature = "zerocopy")]
use slice_struct::slice_struct;

#[repr(C, align(8))]
struct AlignedBuf<const N: usize>(pub [u8; N]);

#[slice_struct(zerocopy)]
pub struct Packet {
    a: u32,
    #[slice]
    payload: [u8],
}

#[test]
fn test_zerocopy_ref() {
    let buf = AlignedBuf::<32>([0u8; 32]);
    let p = Packet::ref_from_bytes(&buf.0[..]).unwrap();
    assert_eq!(p.view().a, &0);
    assert_eq!(p.view().payload, &[]);
}

#[test]
fn test_zerocopy_mut() {
    let mut buf = AlignedBuf::<32>([0u8; 32]);
    let p = Packet::mut_from_bytes(&mut buf.0[..]).unwrap();
    *p.view_mut_unpin().a = 42;
    assert_eq!(p.view().a, &42);
}

#[slice_struct(zerocopy)]
pub struct ComplexPacket {
    header_flags: u32,
    session_id: u64,
    #[slice]
    config: [u16],
    #[slice]
    data: [u8],
}

#[test]
fn test_complex_zerocopy() {
    let buf = AlignedBuf::<128>([0u8; 128]);
    let p = ComplexPacket::ref_from_bytes(&buf.0[..]).unwrap();
    assert_eq!(p.view().header_flags, &0);
    assert_eq!(p.view().session_id, &0);
    assert_eq!(p.view().config, &[]);
    assert_eq!(p.view().data, &[]);
}
