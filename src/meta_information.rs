use crate::attribute::Attribute;
use crate::parser::Parser;
use crate::accumulator::Accumulator;
use crate::prefix::detect;

pub fn parse(bytes: &[u8]) -> Result<Vec<Attribute>, ()> {
    
    if detect(bytes) == false {
        return Err(());
    }

    let accumulator = Accumulator::new();
    let mut parser = Parser::<Accumulator>::new(accumulator);
    parser.parse(&bytes[132..]);

    let result = vec![];
    Ok(result)
}

#[cfg(test)]
mod tests {
    //use super::parse;

    #[test]
    fn zero_preamble_valid_prefix_returns_true() {
    }
}