use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum OperatorExpression<Expr, TyNB> {
        BorrowExpression(v <- BorrowExpression<Expr>),
        DereferenceExpression(v <- DereferenceExpression<Expr>),
        ErrorPropagationExpression(v <- ErrorPropagationExpression<Expr>),
        NegationExpression(v <- NegationExpression<Expr>),
        ArithmeticOrLogicalExpression(v <- ArithmeticOrLogicalExpression<Expr>),
        ComparisonExpression(v <- ComparisonExpression<Expr>),
        LazyBooleanExpression(v <- LazyBooleanExpression<Expr>),
        TypeCastExpression(v <- TypeCastExpression<TyNB, Expr>),
        AssignmentExpression(v <- AssignmentExpression<Expr>),
        CompoundAssignmentExpression(v <- CompoundAssignmentExpression<Expr>),
    }
}
to_tokens! {
    impl ToTokens for enum OperatorExpression<Expr, TyNB> {
        BorrowExpression(v <- BorrowExpression<Expr>),
        DereferenceExpression(v <- DereferenceExpression<Expr>),
        ErrorPropagationExpression(v <- ErrorPropagationExpression<Expr>),
        NegationExpression(v <- NegationExpression<Expr>),
        ArithmeticOrLogicalExpression(v <- ArithmeticOrLogicalExpression<Expr>),
        ComparisonExpression(v <- ComparisonExpression<Expr>),
        LazyBooleanExpression(v <- LazyBooleanExpression<Expr>),
        TypeCastExpression(v <- TypeCastExpression<TyNB, Expr>),
        AssignmentExpression(v <- AssignmentExpression<Expr>),
        CompoundAssignmentExpression(v <- CompoundAssignmentExpression<Expr>),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum BorrowExpression<Expr> [ <- And; double peek <- And ] {
        Mut(<- KwMut; e <- Expr),
        Const(e <- Expr)
    }
}
to_tokens! {
    impl ToTokens for enum BorrowExpression<Expr> [ <- And; double peek <- And ] {
        Mut(<- KwMut; e <- Expr),
        Const(e <- Expr)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct DereferenceExpression<Expr> {
        <- Star;
        expr <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct DereferenceExpression<Expr> {
        <- Star;
        expr <- Expr;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct ErrorPropagationExpression<Expr> {
        expr <- Expr;
        <- Question;
    }
}
to_tokens! {
    impl ToTokens for struct ErrorPropagationExpression<Expr> {
        expr <- Expr;
        <- Question;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct ErrorPropagationExpressionR { <- Question; }
}

impl<Expr> Finalizer<ErrorPropagationExpression<Expr>, Expr> for ErrorPropagationExpressionR {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<ErrorPropagationExpression<Expr>, ErrorPropagationExpression<Expr>>
    {
        std::ops::ControlFlow::Break(ErrorPropagationExpression { expr })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr> Parse<RustCursor<'a>, E> for ErrorPropagationExpression<E> {
    type Finalizer = ErrorPropagationExpressionR;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum NegationExpression<Expr> {
        Neg(<- Minus; expr <- Expr),
        Not(<- Not; expr <- Expr)
    }
}
to_tokens! {
    impl ToTokens for enum NegationExpression<Expr> {
        Neg(<- Minus; expr <- Expr),
        Not(<- Not; expr <- Expr)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ArithmeticOrLogicalExpression<Expr> [ lhs <- Expr ] {
        Mul(<- Star;    rhs <- Expr),
        Div(<- Slash;   rhs <- Expr),
        Rem(<- Percent; rhs <- Expr),

        Add(<- Plus;    rhs <- Expr),
        Sub(<- Minus;   rhs <- Expr),

        Shl(<- Shl;     rhs <- Expr),
        Shr(<- Shr;     rhs <- Expr),

        And(<- And;     rhs <- Expr),
        XOr(<- Caret;   rhs <- Expr),
        Or (<- Or;      rhs <- Expr),
    }
}
to_tokens! {
    impl ToTokens for enum ArithmeticOrLogicalExpression<Expr> [ lhs <- Expr ] {
        Mul(<- Star;    rhs <- Expr),
        Div(<- Slash;   rhs <- Expr),
        Rem(<- Percent; rhs <- Expr),

        Add(<- Plus;    rhs <- Expr),
        Sub(<- Minus;   rhs <- Expr),

        Shl(<- Shl;     rhs <- Expr),
        Shr(<- Shr;     rhs <- Expr),

        And(<- And;     rhs <- Expr),
        XOr(<- Caret;   rhs <- Expr),
        Or (<- Or;      rhs <- Expr),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ArithmeticOrLogicalExpressionR<Expr> {
        Mul(<- Star;    rhs <- Expr),
        Div(<- Slash;   rhs <- Expr),
        Rem(<- Percent; rhs <- Expr),

        Add(<- Plus;    rhs <- Expr),
        Sub(<- Minus;   rhs <- Expr),

        Shl(<- Shl;     rhs <- Expr),
        Shr(<- Shr;     rhs <- Expr),

        And(<- And;     rhs <- Expr),
        XOr(<- Caret;   rhs <- Expr),
        Or (<- Or;      rhs <- Expr),
    }
}

impl<Expr> Finalizer<ArithmeticOrLogicalExpression<Expr>, Expr>
    for ArithmeticOrLogicalExpressionR<Expr>
{
    fn finalize(
        self,
        lhs: Expr,
    ) -> std::ops::ControlFlow<
        ArithmeticOrLogicalExpression<Expr>,
        ArithmeticOrLogicalExpression<Expr>,
    > {
        use ArithmeticOrLogicalExpressionR::*;
        std::ops::ControlFlow::Break(match self {
            Mul(rhs) => ArithmeticOrLogicalExpression::Mul(lhs, rhs),
            Div(rhs) => ArithmeticOrLogicalExpression::Div(lhs, rhs),
            Rem(rhs) => ArithmeticOrLogicalExpression::Rem(lhs, rhs),
            Add(rhs) => ArithmeticOrLogicalExpression::Add(lhs, rhs),
            Sub(rhs) => ArithmeticOrLogicalExpression::Sub(lhs, rhs),
            Shl(rhs) => ArithmeticOrLogicalExpression::Shl(lhs, rhs),
            Shr(rhs) => ArithmeticOrLogicalExpression::Shr(lhs, rhs),
            And(rhs) => ArithmeticOrLogicalExpression::And(lhs, rhs),
            XOr(rhs) => ArithmeticOrLogicalExpression::XOr(lhs, rhs),
            Or(rhs) => ArithmeticOrLogicalExpression::Or(lhs, rhs),
        })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E>
    for ArithmeticOrLogicalExpression<E>
{
    type Finalizer = ArithmeticOrLogicalExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ComparisonExpression<Expr> [ lhs <- Expr ] {
        Eq(<- Eq; rhs <- Expr),
        Ne(<- Ne; rhs <- Expr),
        Gt(<- Gt; rhs <- Expr),
        Lt(<- Lt; rhs <- Expr),
        Ge(<- Ge; rhs <- Expr),
        Le(<- Le; rhs <- Expr),
    }
}
to_tokens! {
    impl ToTokens for enum ComparisonExpression<Expr> [ lhs <- Expr ] {
        Eq(<- Eq; rhs <- Expr),
        Ne(<- Ne; rhs <- Expr),
        Gt(<- Gt; rhs <- Expr),
        Lt(<- Lt; rhs <- Expr),
        Ge(<- Ge; rhs <- Expr),
        Le(<- Le; rhs <- Expr),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ComparisonExpressionR<Expr> {
        Eq(<- Eq; rhs <- Expr),
        Ne(<- Ne; rhs <- Expr),
        Gt(<- Gt; rhs <- Expr),
        Lt(<- Lt; rhs <- Expr),
        Ge(<- Ge; rhs <- Expr),
        Le(<- Le; rhs <- Expr),
    }
}

impl<Expr> Finalizer<ComparisonExpression<Expr>, Expr> for ComparisonExpressionR<Expr> {
    fn finalize(
        self,
        lhs: Expr,
    ) -> std::ops::ControlFlow<ComparisonExpression<Expr>, ComparisonExpression<Expr>> {
        use ComparisonExpressionR::*;
        std::ops::ControlFlow::Break(match self {
            Eq(rhs) => ComparisonExpression::Eq(lhs, rhs),
            Ne(rhs) => ComparisonExpression::Ne(lhs, rhs),
            Gt(rhs) => ComparisonExpression::Gt(lhs, rhs),
            Lt(rhs) => ComparisonExpression::Lt(lhs, rhs),
            Ge(rhs) => ComparisonExpression::Ge(lhs, rhs),
            Le(rhs) => ComparisonExpression::Le(lhs, rhs),
        })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for ComparisonExpression<E> {
    type Finalizer = ComparisonExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum LazyBooleanExpression<Expr> [ lhs <- Expr ] {
        And(<- AndAnd; rhs <- Expr),
        Or (<- OrOr;   rhs <- Expr),
    }
}
to_tokens! {
    impl ToTokens for enum LazyBooleanExpression<Expr> [ lhs <- Expr ] {
        And(<- AndAnd; rhs <- Expr),
        Or (<- OrOr;   rhs <- Expr),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum LazyBooleanExpressionR<Expr> {
        And(<- AndAnd; rhs <- Expr),
        Or (<- OrOr;   rhs <- Expr),
    }
}

impl<Expr> Finalizer<LazyBooleanExpression<Expr>, Expr> for LazyBooleanExpressionR<Expr> {
    fn finalize(
        self,
        lhs: Expr,
    ) -> std::ops::ControlFlow<LazyBooleanExpression<Expr>, LazyBooleanExpression<Expr>> {
        use LazyBooleanExpressionR::*;

        std::ops::ControlFlow::Break(match self {
            And(rhs) => LazyBooleanExpression::And(lhs, rhs),
            Or(rhs) => LazyBooleanExpression::Or(lhs, rhs),
        })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E>
    for LazyBooleanExpression<E>
{
    type Finalizer = LazyBooleanExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TypeCastExpression<TyNB, Expr> {
        expr <- Expr;
        <- KwAs;
        ty <- TyNB;
    }
}
to_tokens! {
    impl ToTokens for struct TypeCastExpression<TyNB, Expr> {
        expr <- Expr;
        <- KwAs;
        ty <- TyNB;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TypeCastExpressionR<TyNB> {
        <- KwAs;
        ty <- TyNB;
    }
}

impl<TyNB, Expr> Finalizer<TypeCastExpression<TyNB, Expr>, Expr> for TypeCastExpressionR<TyNB> {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<TypeCastExpression<TyNB, Expr>, TypeCastExpression<TyNB, Expr>> {
        std::ops::ControlFlow::Break(TypeCastExpression { expr, ty: self.ty })
    }
}

// TODO: Specialize this to the expression type
impl<'a, TyNB: Parse<RustCursor<'a>, ()>, E: Expr + Parse<RustCursor<'a>, ()>>
    Parse<RustCursor<'a>, E> for TypeCastExpression<TyNB, E>
{
    type Finalizer = TypeCastExpressionR<TyNB>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct AssignmentExpression<Expr> {
        lhs <- Expr;
        <- Eq;
        rhs <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct AssignmentExpression<Expr> {
        lhs <- Expr;
        <- Eq;
        rhs <- Expr;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct AssignmentExpressionR<Expr> {
        <- Eq;
        rhs <- Expr;
    }
}

impl<Expr> Finalizer<AssignmentExpression<Expr>, Expr> for AssignmentExpressionR<Expr> {
    fn finalize(
        self,
        lhs: Expr,
    ) -> std::ops::ControlFlow<AssignmentExpression<Expr>, AssignmentExpression<Expr>> {
        std::ops::ControlFlow::Break(AssignmentExpression { lhs, rhs: self.rhs })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for AssignmentExpression<E> {
    type Finalizer = AssignmentExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum CompoundAssignmentExpression<Expr> [ lhs <- Expr ] {
        Mul(<- StarEq;    rhs <- Expr),
        Div(<- SlashEq;   rhs <- Expr),
        Rem(<- PercentEq; rhs <- Expr),

        Add(<- PlusEq;    rhs <- Expr),
        Sub(<- MinusEq;   rhs <- Expr),

        Shl(<- ShlEq;     rhs <- Expr),
        Shr(<- ShrEq;     rhs <- Expr),

        And(<- AndEq;     rhs <- Expr),
        XOr(<- CaretEq;   rhs <- Expr),
        Or (<- OrEq;      rhs <- Expr),
    }
}
to_tokens! {
    impl ToTokens for enum CompoundAssignmentExpression<Expr> [ lhs <- Expr ] {
        Mul(<- StarEq;    rhs <- Expr),
        Div(<- SlashEq;   rhs <- Expr),
        Rem(<- PercentEq; rhs <- Expr),

        Add(<- PlusEq;    rhs <- Expr),
        Sub(<- MinusEq;   rhs <- Expr),

        Shl(<- ShlEq;     rhs <- Expr),
        Shr(<- ShrEq;     rhs <- Expr),

        And(<- AndEq;     rhs <- Expr),
        XOr(<- CaretEq;   rhs <- Expr),
        Or (<- OrEq;      rhs <- Expr),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum CompoundAssignmentExpressionR<Expr> {
        Mul(<- StarEq;    rhs <- Expr),
        Div(<- SlashEq;   rhs <- Expr),
        Rem(<- PercentEq; rhs <- Expr),

        Add(<- PlusEq;    rhs <- Expr),
        Sub(<- MinusEq;   rhs <- Expr),

        Shl(<- ShlEq;     rhs <- Expr),
        Shr(<- ShrEq;     rhs <- Expr),

        And(<- AndEq;     rhs <- Expr),
        XOr(<- CaretEq;   rhs <- Expr),
        Or (<- OrEq;      rhs <- Expr),
    }
}

impl<Expr> Finalizer<CompoundAssignmentExpression<Expr>, Expr>
    for CompoundAssignmentExpressionR<Expr>
{
    fn finalize(
        self,
        lhs: Expr,
    ) -> std::ops::ControlFlow<CompoundAssignmentExpression<Expr>, CompoundAssignmentExpression<Expr>>
    {
        use CompoundAssignmentExpressionR::*;
        std::ops::ControlFlow::Break(match self {
            Mul(rhs) => CompoundAssignmentExpression::Mul(lhs, rhs),
            Div(rhs) => CompoundAssignmentExpression::Div(lhs, rhs),
            Rem(rhs) => CompoundAssignmentExpression::Rem(lhs, rhs),
            Add(rhs) => CompoundAssignmentExpression::Add(lhs, rhs),
            Sub(rhs) => CompoundAssignmentExpression::Sub(lhs, rhs),
            Shl(rhs) => CompoundAssignmentExpression::Shl(lhs, rhs),
            Shr(rhs) => CompoundAssignmentExpression::Shr(lhs, rhs),
            And(rhs) => CompoundAssignmentExpression::And(lhs, rhs),
            XOr(rhs) => CompoundAssignmentExpression::XOr(lhs, rhs),
            Or(rhs) => CompoundAssignmentExpression::Or(lhs, rhs),
        })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E>
    for CompoundAssignmentExpression<E>
{
    type Finalizer = CompoundAssignmentExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
