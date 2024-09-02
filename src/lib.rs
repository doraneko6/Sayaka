pub mod debugger;

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;

    #[test]
    fn test_debug_all() {
        debug_fn_inline!();
        debugln_inline!("test_debugln");
        let a = vec![1, 2, 3];
        debug_var_inline!(a);
    }
}
