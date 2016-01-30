use std::str::FromStr;
use std::ascii::AsciiExt;

bitflags! {
    flags Flags: u8 {
        const REQUIRED   = 0b0000001,
        const MULTIPLE   = 0b0000010,
        const EMPTY_VALS = 0b0000100,
        const GLOBAL     = 0b0001000,
        const HIDDEN     = 0b0010000,
        const TAKES_VAL  = 0b0100000,
        const USE_DELIM  = 0b1000000,
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ArgFlags(Flags);

impl ArgFlags {
    pub fn new() -> Self {
        ArgFlags(EMPTY_VALS | USE_DELIM)
    }

    impl_settings!{ArgSettings,
        Required => REQUIRED,
        Multiple => MULTIPLE,
        EmptyValues => EMPTY_VALS,
        Global => GLOBAL,
        Hidden => HIDDEN,
        TakesValue => TAKES_VAL,
        UseValueDelimiter => USE_DELIM
    }
}

impl Default for ArgFlags {
    fn default() -> Self {
        ArgFlags::new()
    }
}

/// Various settings that apply to arguments and may be set, unset, and checked via getter/setter
/// methods `Arg::set`, `Arg::unset`, and `Arg::is_set`
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgSettings {
    /// The argument must be used
    Required,
    /// The argument may be used multiple times such as `--flag --flag`
    Multiple,
    /// The argument allows empty values such as `--option ""`
    EmptyValues,
    /// The argument should be propagated down through all child subcommands
    Global,
    /// The argument should **not** be shown in help text
    Hidden,
    /// The argument accepts a value, such as `--option <value>`
    TakesValue,
    /// Determines if the argument allows values to be grouped via a delimter
    UseValueDelimiter,
}

impl FromStr for ArgSettings {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "required" => Ok(ArgSettings::Required),
            "multiple" => Ok(ArgSettings::Multiple),
            "global" => Ok(ArgSettings::Global),
            "emptyvalues" => Ok(ArgSettings::EmptyValues),
            "hidden" => Ok(ArgSettings::Hidden),
            "takesvalue" => Ok(ArgSettings::TakesValue),
            "usevaluedelimiter" => Ok(ArgSettings::UseValueDelimiter),
            _ => Err("unknown ArgSetting, cannot convert from str".to_owned()),
        }
    }
}