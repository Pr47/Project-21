pub trait ExprVisitor {
    type ExprResult;

    fn visit_lit_int(&mut self, value: i32) -> Self::ExprResult;
    fn visit_lit_float(&mut self, value: f32) -> Self::ExprResult;
    fn visit_lit_bool(&mut self, value: bool) -> Self::ExprResult;
}
