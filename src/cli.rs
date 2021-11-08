use structopt::StructOpt;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "test_random_crate",
    about = "A tool to fetch random crates and run a test against them"
)]
pub enum Cli {
    /// Run a test against a random crate from crates.io
    TestCrate {
        /// Selection criteria for the random crate [popular, previously-successful, previously-errored, previously-failed]
        #[structopt(short = "s", long = "selection-criteria", default_value="popular")]
        selection: CrateSelection,
    },

    /// Edit the configuration for the project in the current directory
    EditConfig {},

    /// Clean up any downloaded crates from previous tests
    Cleanup {},
}

#[derive(Debug, StructOpt)]
pub enum CrateSelection {
    Popular,
    PreviouslySuccessful,
    PreviouslyErrored,
    PreviouslyFailed,
}

impl FromStr for CrateSelection {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "popular" => Ok(Self::Popular),
            "previously-successful" => Ok(Self::PreviouslySuccessful),
            "previously-errored" => Ok(Self::PreviouslyErrored),
            "previously-failed" => Ok(Self::PreviouslyFailed),
            other => Err(eyre::eyre!(r#"Could not match {} against one of:
popular
previously-successful
previously-errored
previously-failed"#, other))
        }
    }
}
