#[derive(Clone, Debug)]
pub struct Railway {}

#[derive(Clone, Debug)]
pub struct Road {}

#[derive(Copy, Clone, Debug)]
pub enum ProfileType {
    Railway,
    Road,
}
