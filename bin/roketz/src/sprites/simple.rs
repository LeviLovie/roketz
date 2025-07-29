use super::fields::Origin;

#[derive(Clone, Eq, PartialEq, Debug, knus::Decode)]
pub struct Simple {
    #[knus(child, unwrap(argument))]
    pub name: String,
    #[knus(child, unwrap(argument))]
    pub path: String,
    #[knus(child)]
    pub origin: Origin,
}
