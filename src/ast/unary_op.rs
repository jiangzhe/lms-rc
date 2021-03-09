#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOpKind {
    Exp,
    Log,
    Sqrt,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    Sinh,
    Cosh,
    Tanh,
    Erf,
}