#![allow(
    unused_imports,
    unreachable_patterns,
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::unnecessary_operation,
    clippy::into_iter_on_ref,
    clippy::empty_docs,
    clippy::new_without_default,
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::enum_variant_names,
    clippy::redundant_field_names,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod models;
