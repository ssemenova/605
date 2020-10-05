
//Borrowing from Sofiya
extern crate rustc_driver;
#[macro_use]
extern crate rustc_lint;
#[macro_use]
extern crate rustc_session;

use rustc_driver::plugin::Registry;
use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};
use rustc_ast::ast;

declare_lint! {

    USES_GLOBAL,
    Error,
    "Global memory access for thread denied"
}

declare_lint_pass!(
    GlobalUsage => [USES_GLOBAL]
);

impl<'tcx> LateLintPass<'tcx> for GlobalUsage{
	fn check_mem() {
		if mem_bad(){
			.emit();
		}
	} 
}
