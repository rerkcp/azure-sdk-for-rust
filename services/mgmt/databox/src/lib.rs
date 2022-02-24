#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-08-preview")]
pub mod package_2021_08_preview;
#[cfg(all(feature = "package-2021-08-preview", not(feature = "no-default-version")))]
pub use package_2021_08_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-05")]
pub mod package_2021_05;
#[cfg(all(feature = "package-2021-05", not(feature = "no-default-version")))]
pub use package_2021_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-version")))]
pub use package_2021_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-11")]
pub mod package_2020_11;
#[cfg(all(feature = "package-2020-11", not(feature = "no-default-version")))]
pub use package_2020_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "no-default-version")))]
pub use package_2020_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-09")]
pub mod package_2019_09;
#[cfg(all(feature = "package-2019-09", not(feature = "no-default-version")))]
pub use package_2019_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-01")]
pub mod package_2018_01;
#[cfg(all(feature = "package-2018-01", not(feature = "no-default-version")))]
pub use package_2018_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
