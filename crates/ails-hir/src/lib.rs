use ails_ast::Module;

#[derive(Debug, Clone)]
pub struct HirModule {
    pub module_name: String,
    pub function_names: Vec<String>,
}

pub fn lower_module(module: &Module) -> HirModule {
    HirModule {
        module_name: module.name.clone(),
        function_names: module.functions.iter().map(|f| f.name.clone()).collect(),
    }
}
