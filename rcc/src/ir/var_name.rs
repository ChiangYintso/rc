pub const RA: &str = "%ra";
pub const FP: &str = "%fp";

pub fn local_var(ident: &str, scope_id: u64) -> String {
    format!("{}_{}", ident, scope_id)
}

pub fn temp_local_var(temp_count: u64, scope_id: u64) -> String {
    format!("${}_{}", temp_count, scope_id)
}

pub fn is_temp_var(var_name: &str) -> bool {
    var_name.starts_with('$')
}

pub fn branch_name(func_scope_id: u64, bb_id: usize) -> String{
    format!(".L{}_{}",  func_scope_id,bb_id)
}