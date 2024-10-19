#[derive(Debug, PartialEq)]
pub struct AbstractProgram {
    pub global_statements: Vec<Box<GlobalStatement>>,
}

#[derive(Debug, PartialEq)]
pub enum GlobalStatement {
    FunctionDefinition {
        name: String,
        retval_dtype: Box<Dtype>,
        formal_args: Vec<Box<FormalArg>>,
        body: Box<LocalStatement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Dtype {
    Int,
}

#[derive(Debug, PartialEq)]
pub struct FormalArg {
    pub name: String,
    pub dtype: Box<Dtype>,
}

#[derive(Debug, PartialEq)]
pub enum LocalStatement {
    VariableDeclaration {
        name: String,
        dtype: Box<Dtype>,
    },
    Evaluation {
        expression: Box<Expression>,
    },
    Block {
        statements: Vec<Box<LocalStatement>>,
    },
    If {
        condition: Box<Expression>,
        true_branch: Box<LocalStatement>,
    },
    IfElse {
        condition: Box<Expression>,
        true_branch: Box<LocalStatement>,
        false_branch: Box<LocalStatement>,
    },
    While {
        condition: Box<Expression>,
        body: Box<LocalStatement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    IntLiteral {
        value: i128,
    },
    Variable {
        name: String,
    },
    Assignment {
        lhs: Box<LValue>,
        rhs: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum LValue {
    Variable { name: String },
}
