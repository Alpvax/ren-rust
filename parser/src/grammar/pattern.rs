use crate::{Parser, syntax::Context};

pub(super) fn parse_pattern(p: &mut Parser) {
    let m = p.start();
    m.complete(p, Context::Pattern);
    todo!("Implement pattern parsing");
}