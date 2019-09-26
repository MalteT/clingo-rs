use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let lit = ast::Literal::from_term(ast::Sign::None, &term1);
    let term2 = ast::Term::from(sym);
    let lit2 = ast::Literal::from_term(ast::Sign::None, &term2);
    let args = vec![lit2];
    let cond = ast::ConditionalLiteral::new(&lit, &args);
    drop(lit);
    drop(args);
    let _end = cond;
}