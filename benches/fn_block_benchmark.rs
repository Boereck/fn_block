#[macro_use]
extern crate criterion;
extern crate fn_block;


#[macro_use]
use fn_block::*;
use criterion::{Criterion, Fun};

fn pure_option(o: Option<&str>) {
    let s = o.and_then(|st| st.get(0..3)).map(|st| st.to_lowercase());
}

fn closure(o: Option<&str>) {
    let s = (|| Some(o?.get(0..3)?.to_lowercase()) )();
}

fn fn_expr_usage(o: Option<&str>) {
    let s = fn_expr!{ o?.get(0..3)?.to_lowercase().into_some() };
}

fn bench_option_combinators(c: &mut Criterion) {
	let fn_expr_fn = Fun::new("fn_expr Wrapper", |b, o| b.iter(|| fn_expr_usage(*o)));
	let closure_fn = Fun::new("Closure Wrapper", |b, o| b.iter(|| closure(*o)));
	let pure_opts_fn = Fun::new("Pure Options", |b, o| b.iter(|| pure_option(*o)));
	
	let functions = vec!(pure_opts_fn, closure_fn, fn_expr_fn);
	let opt = Some("Foobar");

	c.bench_functions("Option combinations", functions, opt);
}


criterion_group! {
	name = benches; 
    config = Criterion::default().sample_size(500);
	targets = bench_option_combinators
}

criterion_main!(benches);
