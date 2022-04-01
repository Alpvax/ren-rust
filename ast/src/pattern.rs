use crate::{expression::{Literal, TemplateSegment}, Ident};

#[derive(Debug, Clone)]
pub enum Pattern {
    ArrayDestructure(Vec<Pattern>),
    LiteralPattern(Literal),
    Name(String),
    RecordDestructure(Vec<(String, Option<Pattern>)>),
    Spread(String),
    TemplateDestructure(Vec<TemplateSegment<Ident>>),
    Typeof(String, Box<Pattern>),
    VariantDestructure(String, Vec<Pattern>),
    Wildcard(Option<String>),
}
impl Pattern {
    pub fn names(&self) -> Vec<Ident> {
        match self {
            Pattern::ArrayDestructure(patterns) => {
                patterns.iter().map(Pattern::names).flatten().collect()
            }
            Pattern::Name(n) => vec![n.clone()],
            Pattern::RecordDestructure(m) => m.iter().fold(Vec::new(), |mut v, (k, p)| {
                v.extend(p.as_ref().map_or_else(|| vec![k.clone()], Pattern::names));
                v
            }),
            Pattern::Spread(n) => vec![n.clone()],
            Pattern::TemplateDestructure(segments) => segments.iter().filter_map(|s| if let TemplateSegment::Expr(n) = s { Some(n.clone()) } else { None }).collect(),
            Pattern::Typeof(_, _) => todo!(),
            Pattern::VariantDestructure(tag, patterns) => {
                patterns.iter().fold(vec![tag.clone()], |mut v, pat| {
                    v.extend(pat.names());
                    v
                })
            }
            Pattern::LiteralPattern(_) | Pattern::Wildcard(_) => Vec::new(),
        }
    }
}
