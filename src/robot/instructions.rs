#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Walk,
    TurnRight,
    TurnLeft,
    If {
        condition: Condition,
        instructions: Vec<Instruction>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    // TODO
}
