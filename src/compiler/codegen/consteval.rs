use crate::compiler::parse::cst::*;
use crate::compiler::codegen::{CodegenContext, ConstEvalResult};
use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::io_ctx::Type21;
use crate::value::RtValue;

impl CodegenContext {
    pub fn consteval_expr(&self, expr: &Expr) -> Result<Option<ConstEvalResult>, String> {
        match expr {
            Expr::AtomicExpr(atomic_expr) => self.consteval_atomic_expr(atomic_expr),
            Expr::AssignExpr(_) => Ok(None),
            Expr::MultiAssignExpr(_) => Ok(None),
            Expr::BinaryExpr(bin_expr) => self.consteval_bin_expr(bin_expr),
            Expr::UnaryExpr(unary_expr) => self.consteval_unary_expr(unary_expr),
            Expr::FuncCall(_) => Ok(None)
        }
    }

    pub fn consteval_atomic_expr(
        &self,
        atomic_expr: &AtomicExpr
    ) -> Result<Option<ConstEvalResult>, String> {
        match atomic_expr {
            AtomicExpr::Ident(ident) => if let Some(result) = self.constant.get(ident) {
                Ok(Some(*result))
            } else {
                Err(format!("未定义的常量 {}", ident))
            },
            AtomicExpr::Integer(int) => Ok(Some(ConstEvalResult {
                ty: Type21::Int32,
                value: RtValue::from(*int)
            })),
            AtomicExpr::Float(float) => Ok(Some(ConstEvalResult {
                ty: Type21::Float32,
                value: RtValue::from(*float)
            })),
            AtomicExpr::Bool(bool) => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(*bool)
            })),
            AtomicExpr::Paren(inner) => self.consteval_expr(inner),
            AtomicExpr::TypeCast(TypeCast { dest, expr }) => {
                let Some(ConstEvalResult { ty, value }) = self.consteval_expr(&expr)? else {
                    return Ok(None);
                };

                if ty == *dest {
                    return Ok(Some(ConstEvalResult { ty, value }));
                }

                Ok(Some(match (ty, dest) {
                    (Type21::Int32, Type21::Float32) => {
                        ConstEvalResult {
                            ty: Type21::Float32,
                            value: RtValue::from(unsafe { value.i } as f32)
                        }
                    },
                    (Type21::Int32, Type21::Bool) => {
                        ConstEvalResult {
                            ty: Type21::Bool,
                            value: RtValue::from(unsafe { value.i } != 0)
                        }
                    },
                    (Type21::Float32, Type21::Int32) => {
                        ConstEvalResult {
                            ty: Type21::Int32,
                            value: RtValue::from(unsafe { value.f } as i32)
                        }
                    },
                    (Type21::Float32, Type21::Bool) => {
                        ConstEvalResult {
                            ty: Type21::Bool,
                            value: RtValue::from(unsafe { value.f } != 0.0)
                        }
                    },
                    (Type21::Bool, Type21::Int32) => {
                        ConstEvalResult {
                            ty: Type21::Int32,
                            value: RtValue::from(if unsafe { value.b } { 1 } else { 0 })
                        }
                    },
                    (Type21::Bool, Type21::Float32) => {
                        ConstEvalResult {
                            ty: Type21::Float32,
                            value: RtValue::from(if unsafe { value.b } { 1.0 } else { 0.0 })
                        }
                    },
                    (_, _) => {
                        #[cfg(debug_assertions)] unreachable!();
                        #[cfg(not(debug_assertions))] return Ok(None);
                    }
                }))
            },
            AtomicExpr::FuncCall(_) => Ok(None)
        }
    }

    pub fn consteval_bin_expr(
        &self,
        bin_expr: &BinaryExpr
    ) -> Result<Option<ConstEvalResult>, String> {
        let lhs = self.consteval_expr(&bin_expr.lhs)?;
        let rhs = self.consteval_expr(&bin_expr.rhs)?;

        let Some(lhs) = lhs else { return Ok(None) };
        let Some(rhs) = rhs else { return Ok(None) };

        if lhs.ty != rhs.ty {
            return Err(format!("二元表达式的两个操作数类型不一致 ({} 和 {})", lhs.ty, rhs.ty));
        }

        match bin_expr.op {
            BinaryOp::Add => match lhs.ty {
                Type21::Int32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Int32,
                    value: RtValue::from(unsafe { lhs.value.i + rhs.value.i })
                })),
                Type21::Float32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Float32,
                    value: RtValue::from(unsafe { lhs.value.f + rhs.value.f })
                })),
                Type21::Bool => Err("无法对布尔类型应用加法".into())
            },
            BinaryOp::Sub => match lhs.ty {
                Type21::Int32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Int32,
                    value: RtValue::from(unsafe { lhs.value.i - rhs.value.i })
                })),
                Type21::Float32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Float32,
                    value: RtValue::from(unsafe { lhs.value.i - rhs.value.i })
                })),
                Type21::Bool => Err("无法对布尔类型应用减法".into())
            },
            BinaryOp::Mul => match lhs.ty {
                Type21::Int32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Int32,
                    value: RtValue::from(unsafe { lhs.value.i * rhs.value.i })
                })),
                Type21::Float32 => Ok(Some(ConstEvalResult {
                    ty: Type21::Float32,
                    value: RtValue::from(unsafe { lhs.value.i * rhs.value.i })
                })),
                Type21::Bool => Err("无法对布尔类型应用乘法".into())
            },
            BinaryOp::Div => match lhs.ty {
                Type21::Int32 => {
                    if unsafe { rhs.value.i == 0 } {
                        return Err("不能除以 0".into());
                    }

                    Ok(Some(ConstEvalResult {
                        ty: Type21::Int32,
                        value: RtValue::from(unsafe { lhs.value.i / rhs.value.i })
                    }))
                },
                Type21::Float32 => {
                    if unsafe { rhs.value.f == 0.0 } {
                        return Err("不能除以 0".into());
                    }

                    Ok(Some(ConstEvalResult {
                        ty: Type21::Float32,
                        value: RtValue::from(unsafe { lhs.value.i / rhs.value.i })
                    }))
                },
                Type21::Bool => Err("无法对布尔类型应用除法".into())
            },
            BinaryOp::Mod => match lhs.ty {
                Type21::Int32 => {
                    if unsafe { rhs.value.i == 0 } {
                        return Err("不能除以 0".into());
                    }

                    Ok(Some(ConstEvalResult {
                        ty: Type21::Int32,
                        value: RtValue::from(unsafe { lhs.value.i % rhs.value.i })
                    }))
                }
                Type21::Float32 => Err("无法对浮点类型应用取余".into()),
                Type21::Bool => Err("无法对布尔类型应用取余".into())
            },
            BinaryOp::Eq => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(unsafe { lhs.value.repr == rhs.value.repr })
            })),
            BinaryOp::Ne => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(unsafe { lhs.value.repr != rhs.value.repr })
            })),
            BinaryOp::Lt => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(match lhs.ty {
                    Type21::Int32 => unsafe { lhs.value.i < rhs.value.i }
                    Type21::Float32 => unsafe { lhs.value.f < rhs.value.f }
                    Type21::Bool => unsafe { !lhs.value.b && rhs.value.b }
                })
            })),
            BinaryOp::Le => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(match lhs.ty {
                    Type21::Int32 => unsafe { lhs.value.i <= rhs.value.i }
                    Type21::Float32 => unsafe { lhs.value.f <= rhs.value.f }
                    Type21::Bool => unsafe { !(lhs.value.b && !rhs.value.b) }
                })
            })),
            BinaryOp::Gt => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(match lhs.ty {
                    Type21::Int32 => unsafe { lhs.value.i > rhs.value.i }
                    Type21::Float32 => unsafe { lhs.value.f > rhs.value.f }
                    Type21::Bool => unsafe { lhs.value.b && !rhs.value.b }
                })
            })),
            BinaryOp::Ge => Ok(Some(ConstEvalResult {
                ty: Type21::Bool,
                value: RtValue::from(match lhs.ty {
                    Type21::Int32 => unsafe { lhs.value.i >= rhs.value.i }
                    Type21::Float32 => unsafe { lhs.value.f >= rhs.value.f }
                    Type21::Bool => unsafe { !(!lhs.value.b && rhs.value.b) }
                })
            })),
            BinaryOp::And => if let Type21::Bool = lhs.ty {
                Ok(Some(ConstEvalResult {
                    ty: Type21::Bool,
                    value: RtValue::from(unsafe { lhs.value.b && rhs.value.b })
                }))
            } else {
                Err("仅能对布尔类型应用逻辑与".into())
            }
            BinaryOp::Or => if let Type21::Bool = lhs.ty {
                Ok(Some(ConstEvalResult {
                    ty: Type21::Bool,
                    value: RtValue::from(unsafe { lhs.value.b || rhs.value.b })
                }))
            } else {
                Err("仅能对布尔类型应用逻辑或".into())
            }
        }
    }

    pub fn consteval_unary_expr(
        &self,
        unary_expr: &UnaryExpr
    ) -> Result<Option<ConstEvalResult>, String> {
        let Some(ConstEvalResult { ty, value }) = self.consteval_expr(&unary_expr.expr)? else {
            return Ok(None);
        };

        match unary_expr.op {
            UnaryOp::Negate => Ok(Some(ConstEvalResult {
                ty,
                value: match ty {
                    Type21::Int32 => RtValue::from(unsafe { -value.i }),
                    Type21::Float32 => RtValue::from(unsafe { -value.f }),
                    Type21::Bool => RtValue::from(unsafe { !value.b }),
                }
            })),
            UnaryOp::Not => if let Type21::Bool = ty {
                Ok(Some(ConstEvalResult {
                    ty,
                    value: RtValue::from(unsafe { !value.b })
                }))
            } else {
                Err("只能对布尔类型应用逻辑非".into())
            }
        }
    }
}