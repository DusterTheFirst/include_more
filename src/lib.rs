#![forbid(unsafe_code)]

#[doc(ignore)]
pub use eager;

#[doc(ignore)]
pub use include_more_proc;

#[macro_export]
macro_rules! include_sass {
    ($filename:literal) => {
        {
            use $crate::eager::*;

            eager_macro_rules! { $eager_1
                macro_rules! eager_file {
                    () => {
                        file!()
                    };
                }
            }

            eager! { $crate::include_more_proc::include_sass!($filename, eager_file!()) }
        }

    };
}
