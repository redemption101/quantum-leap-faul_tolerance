use z3::{Context, Solver, ast::Int};

pub struct OmniscienceVerifier<'a> {
    context: &'a Context,
    solver: Solver<'a>,
}

impl<'a> OmniscienceVerifier<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self {
            context: ctx,
            solver: Solver::new(ctx),
        }
    }

    pub fn verify_accounting_invariant(&self, assets: i64, liabilities: i64, equity: i64) -> bool {
        let a = Int::from_i64(self.context, assets);
        let l = Int::from_i64(self.context, liabilities);
        let e = Int::from_i64(self.context, equity);
        
        self.solver.assert(&a._eq(&(l + e)));
        self.solver.check() == z3::SatResult::Sat
    }

    pub fn verify_thermodynamics_invariant(&self, heat_in: i64, work_done: i64, internal_energy: i64) -> bool {
        let q = Int::from_i64(self.context, heat_in);
        let w = Int::from_i64(self.context, work_done);
        let du = Int::from_i64(self.context, internal_energy);
        
        self.solver.assert(&du._eq(&(q - w)));
        self.solver.check() == z3::SatResult::Sat
    }
}
