
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
