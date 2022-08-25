#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    line: usize,
    col: usize,
}
impl Default for Pos {
    fn default() -> Self {
        Self { line: 1, col: 1 }
    }
}
impl From<(usize, usize)> for Pos {
    fn from((line, col): (usize, usize)) -> Self {
        Self { line, col }
    }
}
impl From<Pos> for (usize, usize) {
    fn from(Pos { line, col }: Pos) -> Self {
        (line, col)
    }
}

type SpanTuple = ((usize, usize), (usize, usize));

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: Pos,
    end: Pos,
}
impl ::serde::Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SpanTuple::from(*self).serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <SpanTuple>::deserialize(deserializer).map(Self::from)
    }
}

impl From<SpanTuple> for Span {
    fn from((start, end): SpanTuple) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}
impl From<(Pos, Pos)> for Span {
    fn from((start, end): (Pos, Pos)) -> Self {
        Self { start, end }
    }
}
impl From<Span> for SpanTuple {
    fn from(Span { start, end }: Span) -> Self {
        (start.into(), end.into())
    }
}
