use crate::parser::EqType;

pub const EQUATIONS: [(EqType, &str); 2] = [
    (
        EqType::A,
        r"=[[:blank:]]?([a-z]|[A-Z])+[[:blank:]]?[+|-|*|/|%][[:blank:]]?(-?\d+)",
    ),
    (
        EqType::B,
        r"=[[:blank:]]?([a-z]|[A-Z])+[[:blank:]]?[+|-|*|/|%][[:blank:]]?([a-z]|[A-Z])+",
    ),
];
