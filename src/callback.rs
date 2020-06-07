use crate::attribute::*;

pub trait Callback
{
    fn element(&mut self, attribute: Attribute);
    fn data(&mut self, data: &[u8]);
}