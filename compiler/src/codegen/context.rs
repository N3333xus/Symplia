use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine};
use std::collections::HashMap;

pub struct CodeGenContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: inkwell::builder::Builder<'ctx>,
    pub named_values: HashMap<String, inkwell::values::BasicValueEnum<'ctx>>,
    pub execution_engine: Option<ExecutionEngine<'ctx>>,
}

impl<'ctx> CodeGenContext<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        Self {
            context,
            module,
            builder,
            named_values: HashMap::new(),
            execution_engine: None,
        }
    }

    pub fn initialize_execution_engine(&mut self) -> Result<(), String> {
        match self.module.create_jit_execution_engine(inkwell::OptimizationLevel::None) {
            Ok(execution_engine) => {
                self.execution_engine = Some(execution_engine);
                Ok(())
            }
            Err(e) => Err(format!("Failed to create execution engine: {}", e)),
        }
    }

    pub fn get_function_address(&self, name: &str) -> Option<usize> {
        self.execution_engine.as_ref()
            .and_then(|ee| ee.get_function_address(name).ok())
    }
}