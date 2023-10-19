#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Walk,
    TurnRight,
    TurnLeft,
    If {
        condition: Condition,
        instructions: Vec<Instruction>,
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Placeholder
}
