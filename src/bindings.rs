use polars::prelude::*;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
mod polarstype;

pub fn register_bindings(vm: &mut Engine) {
    vm.register_type::<polarstype::LazyFrame>("DataFrame?");
    vm.register_fn("pl:LazyCsvReader", |path: String| {
        polarstype::LazyCsvReader {
            inner: LazyCsvReader::new(PlRefPath::new(path)),
        }
    });
    vm.register_fn("pl:finish", |lazy_self: polarstype::LazyCsvReader| {
        match LazyCsvReader::finish(lazy_self.inner) {
            Ok(v) => Ok(polarstype::LazyFrame { inner: v }),
            Err(e) => Err(polarstype::PolarsError { inner: e }),
        }
    });
}
