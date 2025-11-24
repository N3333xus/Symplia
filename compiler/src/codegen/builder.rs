use inkwell::values::{/*BasicValue,*/ BasicValueEnum /* , FunctionValue, PointerValue*/};
use inkwell::AddressSpace;
use crate::parser::ast::{Type, Literal, BinaryOperator, UnaryOperator};
//use crate::semantic::semantic::AnnotatedExpr;

pub struct IRBuilder<'ctx> {
    pub context: &'ctx inkwell::context::Context,
}

impl<'ctx> IRBuilder<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context) -> Self {
        Self { context }
    }

    pub fn get_llvm_type(&self, symplia_type: &Type) -> inkwell::types::BasicTypeEnum<'ctx> {
        match symplia_type {
            Type::Inteiro => self.context.i32_type().into(),
            Type::Decimal => self.context.f64_type().into(),
            // CORREÇÃO: Usar AddressSpace::from(0) em vez de Generic
            Type::Texto => self.context.ptr_type(AddressSpace::from(0)).into(),
            Type::Logico => self.context.bool_type().into(),
        }
    }

    // Converte literal para valor LLVM
    pub fn build_literal(&self, literal: &Literal) -> BasicValueEnum<'ctx> {
        match literal {
            Literal::Inteiro(n) => self.context.i32_type().const_int(*n as u64, false).into(),
            Literal::Decimal(n) => self.context.f64_type().const_float(*n).into(),
            Literal::Texto(_s) => {
                // ATENÇÃO: Isso ainda precisa ser corrigido no llvm_ir.rs
                // Por enquanto retorna null pointer
                // CORREÇÃO: Usar AddressSpace::from(0) aqui também
                self.context.ptr_type(AddressSpace::from(0)).const_null().into()
            }
            Literal::Logico(b) => self.context.bool_type().const_int(*b as u64, false).into(),
        }
    }

    pub fn build_binary_op(
        &self,
        builder: &inkwell::builder::Builder<'ctx>,
        op: &BinaryOperator,
        left: BasicValueEnum<'ctx>,
        right: BasicValueEnum<'ctx>,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match (left_type, right_type) {
            (Type::Inteiro, Type::Inteiro) => {
                let left_int = left.into_int_value();
                let right_int = right.into_int_value();
                
                match op {
                    BinaryOperator::Add => {
                        // CORREÇÃO: Usar map_err para converter BuilderError para String
                        let result = builder.build_int_add(left_int, right_int, "addtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Subtract => {
                        let result = builder.build_int_sub(left_int, right_int, "subtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Multiply => {
                        let result = builder.build_int_mul(left_int, right_int, "multmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Divide => {
                        let result = builder.build_int_signed_div(left_int, right_int, "divtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Modulo => {
                        let result = builder.build_int_signed_rem(left_int, right_int, "modtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Equal => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::EQ, left_int, right_int, "eqtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::NotEqual => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::NE, left_int, right_int, "netmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Less => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::SLT, left_int, right_int, "lttmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::LessEqual => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::SLE, left_int, right_int, "letmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Greater => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::SGT, left_int, right_int, "gttmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::GreaterEqual => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::SGE, left_int, right_int, "getmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    _ => Err(format!("Operador binário não suportado para inteiros: {:?}", op)),
                }
            }
            (Type::Decimal, Type::Decimal) => {
                let left_float = left.into_float_value();
                let right_float = right.into_float_value();
                
                match op {
                    BinaryOperator::Add => {
                        let result = builder.build_float_add(left_float, right_float, "faddtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Subtract => {
                        let result = builder.build_float_sub(left_float, right_float, "fsubtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Multiply => {
                        let result = builder.build_float_mul(left_float, right_float, "fmultmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Divide => {
                        let result = builder.build_float_div(left_float, right_float, "fdivtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Equal => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::OEQ, left_float, right_float, "feqtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::NotEqual => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::ONE, left_float, right_float, "fnetmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Less => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::OLT, left_float, right_float, "flttmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::LessEqual => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::OLE, left_float, right_float, "fletmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Greater => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::OGT, left_float, right_float, "fgttmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::GreaterEqual => {
                        let result = builder.build_float_compare(inkwell::FloatPredicate::OGE, left_float, right_float, "fgetmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    _ => Err(format!("Operador binário não suportado para decimais: {:?}", op)),
                }
            }
            (Type::Logico, Type::Logico) => {
                let left_bool = left.into_int_value();
                let right_bool = right.into_int_value();
                
                match op {
                    BinaryOperator::And => {
                        let result = builder.build_and(left_bool, right_bool, "andtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Or => {
                        let result = builder.build_or(left_bool, right_bool, "ortmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::Equal => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::EQ, left_bool, right_bool, "booleqtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    BinaryOperator::NotEqual => {
                        let result = builder.build_int_compare(inkwell::IntPredicate::NE, left_bool, right_bool, "boolnetmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    },
                    _ => Err(format!("Operador binário não suportado para booleanos: {:?}", op)),
                }
            }
            _ => Err(format!("Tipos incompatíveis para operação binária: {:?} e {:?}", left_type, right_type)),
        }
    }

    pub fn build_unary_op(
        &self,
        builder: &inkwell::builder::Builder<'ctx>,
        op: &UnaryOperator,
        operand: BasicValueEnum<'ctx>,
        operand_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match op {
            UnaryOperator::Negate => {
                match operand_type {
                    Type::Inteiro => {
                        let zero = self.context.i32_type().const_int(0, false);
                        let operand_int = operand.into_int_value();
                        let result = builder.build_int_sub(zero, operand_int, "negtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    }
                    Type::Decimal => {
                        let zero = self.context.f64_type().const_float(0.0);
                        let operand_float = operand.into_float_value();
                        let result = builder.build_float_sub(zero, operand_float, "fnegtmp")
                            .map_err(|e| e.to_string())?;
                        Ok(result.into())
                    }
                    _ => Err("Operador de negação só suportado para tipos numéricos".to_string()),
                }
            }
            UnaryOperator::Not => {
                if let Type::Logico = operand_type {
                    let operand_bool = operand.into_int_value();
                    let one = self.context.bool_type().const_int(1, false);
                    let result = builder.build_xor(operand_bool, one, "nottmp")
                        .map_err(|e| e.to_string())?;
                    Ok(result.into())
                } else {
                    Err("Operador 'not' só suportado para booleanos".to_string())
                }
            }
            UnaryOperator::Plus => Ok(operand), // +operand é o próprio operand
        }
    }
}