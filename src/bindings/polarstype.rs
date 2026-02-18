use polars::prelude;
use steel_derive::Steel;

#[derive(Clone, Steel)]
pub struct LazyFrame {
    pub inner: prelude::LazyFrame,
}

#[derive(Clone, Steel)]
pub struct LazyCsvReader {
    pub inner: prelude::LazyCsvReader,
}

#[derive(Clone, Steel)]
pub struct PolarsError {
    pub inner: prelude::PolarsError,
}
