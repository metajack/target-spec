#[derive(Debug, PartialEq)]
pub enum Atom {
    Ident(String),
    Value(String),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Any(Vec<Expr>),
    All(Vec<Expr>),
    Not(Box<Expr>),
    TestSet(Atom),
    TestEqual((Atom, Atom)),
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Triple(String),
    Spec(Expr),
}
