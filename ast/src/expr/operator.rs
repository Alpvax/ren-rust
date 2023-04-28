macro_rules! make_operator {
    ($(($op:ident, $name:literal, $symbol:literal)),+ $(,)?) => {
        // static OPERATORS: &'static [(Operator, &'static str, &'static str)] = &[$((Operator::$op, $name, $symbol)),+];
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum Operator {$(
            /// $symbol
            $op,
        )+}
        impl Operator {
            pub fn from_symbol(symbol: &str) -> Option<Self> {
                match symbol {
                    $($symbol => Some(Self::$op),)+
                    _ => None,
                }
            }
        }
    };
}
make_operator![
    (Add, "add", "+"),
    (And, "and", "and"),
    (Concat, "concat", "<>"),
    (Cons, "cons", "::"),
    (Div, "div", "/"),
    (Eq, "eq", "=="),
    (Gte, "gte", ">="),
    (Gt, "gt", ">"),
    (Lte, "lte", "<="),
    (Lt, "lt", "<"),
    (Mod, "mod", "%"),
    (Mul, "mul", "*"),
    (Neq, "neq", "!="),
    (Or, "or", "or"),
    (Pipe, "pipe", "|>"),
    (Sub, "sub", "-"),
];

// impl Operator {
// fn from_name(name: &str) -> Option<Self> {
//     OPERATORS_FULL
//         .iter()
//         .find(|(_, op_name, _)| *op_name == name)
//         .map(|(op, ..)| *op)
// }
// pub fn from_symbol(symbol: &str) -> Option<Self> {
//     OPERATORS_FULL
//         .iter()
//         .find(|(.., sym)| *sym == symbol)
//         .map(|(op, ..)| *op)
// }
// fn find_data(&self) -> [&'static str; 2] {
//     OPERATORS_FULL
//         .iter()
//         .find(|(op, ..)| op == self)
//         .map(|(_, name, sym)| [*name, *sym])
//         .expect(&format!(
//             "OPERATOR {:?} is not in {:?}",
//             self, OPERATORS_FULL
//         ))
// }
// fn name(&self) -> &'static str {
//     self.find_data()[0]
// }
// fn symbol(&self) -> &'static str {
//     self.find_data()[1]
// }
// }
