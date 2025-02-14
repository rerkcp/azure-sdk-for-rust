#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2018-02")]
pub mod package_2018_02;
#[cfg(all(feature = "package-2018-02", not(feature = "no-default-tag")))]
pub use package_2018_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-10")]
pub mod package_2019_10;
#[cfg(all(feature = "package-2019-10", not(feature = "no-default-tag")))]
pub use package_2019_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-tag")))]
pub use package_2020_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-tag")))]
pub use package_2020_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-07")]
pub mod package_2020_07;
#[cfg(all(feature = "package-2020-07", not(feature = "no-default-tag")))]
pub use package_2020_07::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
