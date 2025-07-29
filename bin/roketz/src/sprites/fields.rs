#[derive(Clone, Debug, Default, knus::Decode)]
pub struct Origin {
    #[knus(property)]
    pub x: i32,
    #[knus(property)]
    pub y: i32,
}
