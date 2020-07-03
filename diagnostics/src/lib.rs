#![feature(decl_macro)]

mod diagnostic;
mod emit;
mod file;
mod reporter;
mod span;

pub use diagnostic::*;
pub use file::*;
pub use reporter::*;
pub use span::*;

pub macro unimpl($span:expr, $msg:literal $(, $arg:expr)*) {
    $crate::Diagnostic::new($crate::Severity::Bug, None, format!("Unimplemented feature: {}", format!($msg $(, $arg)*)))
        .label($crate::Severity::Bug, $span, None::<String>)
}

pub macro unreach($span:expr, $msg:literal $(, $arg:expr)*) {
    $crate::Diagnostic::new($crate::Severity::Bug, None, format!("Unreachable code reached: {}", format!($msg $(, $arg)*)))
        .label($crate::Severity::Bug, $span, None::<String>)
}

pub macro report_error {
    ($reporter:expr, $sev:ident $code:literal, $span:expr, ($($msg:tt)+) $(, ($lsev:ident $lspan:expr, $($lmsg:tt)+))*) => {
        $crate::report_error!(@emit
            $reporter,
            $crate::Severity::from(stringify!($sev)),
            $code,
            format!($($msg)+),
            [
                ($crate::Severity::from(stringify!($sev)), $span, ::std::option::Option::None::<::std::string::String>)
                $(, ($crate::Severity::from(stringify!($lsev)), $lspan, format!($($lmsg)+)))*
            ]
        )
    },

    (@emit $reporter:expr, $sev:expr, $code:expr, $msg:expr, [$(($lsev:expr, $lspan:expr, $lmsg:expr)),+]) => {
        $reporter.add(
            $crate::Diagnostic::new($sev, $code, $msg)
                $(.label($lsev, $lspan, $lmsg))+
        )
    }
}
