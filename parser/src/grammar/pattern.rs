use crate::{syntax::Context, Parser};

pub(super) fn parse_pattern(p: &mut Parser) {
    let m = p.start("pattern");
    m.complete(p, Context::Pattern);
    todo!("Implement pattern parsing");
}
