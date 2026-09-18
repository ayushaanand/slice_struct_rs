#[repr(C)] struct Inner { a: u32, b: [u8] } #[repr(transparent)] struct Outer(Inner); fn main() { std::mem::size_of_val(&Outer(Inner { a: 1, b: [] })); }
