// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use clap::Parser;

use crate::options;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutAlgorithm {
    None,
    Dot,
    Neato,
    Twopi,
    Circo,
    Fdp,
    Sfdp,
}

impl FromStr for LayoutAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "dot" => Ok(Self::Dot),
            "neato" => Ok(Self::Neato),
            "twopi" => Ok(Self::Twopi),
            "circo" => Ok(Self::Circo),
            "fdp" => Ok(Self::Fdp),
            "sfdp" => Ok(Self::Sfdp),
            _ => Err("Unrecognized layout"),
        }
    }
}

impl ToString for LayoutAlgorithm {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none",
            Self::Dot => "dot",
            Self::Neato => "neato",
            Self::Twopi => "twopi",
            Self::Circo => "circo",
            Self::Fdp => "fdp",
            Self::Sfdp => "sfdp",
        }
        .to_owned()
    }
}

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[group(id = "GenerateSelectionOptions")]
pub struct Options {
    #[command(flatten)]
    pub general: options::general::Options,

    #[command(flatten)]
    pub project: options::project::Options,

    #[command(flatten)]
    pub selection: SelectionOptions,

    /// Require graph to be acyclic
    #[arg(long = "acyclic", conflicts_with = "focus_on")]
    pub acyclic: bool,

    /// The graph layout algorithm to use
    /// (e.g. none, dot, neato, twopi, circo, fdp, sfdp).
    #[arg(long = "layout", default_value = "neato")]
    pub layout: LayoutAlgorithm,

    /// Focus the graph on a particular path or use-tree's environment,
    /// e.g. "foo::bar::{self, baz, blee::*}".
    #[arg(long = "focus-on")]
    pub focus_on: Option<String>,

    /// The maximum depth of the generated graph
    /// relative to the crate's root node, or nodes selected by '--focus-on'.
    #[arg(long = "max-depth")]
    pub max_depth: Option<usize>,
}

// Important:
// Some of the `--flag` and `--no-flag` arg pairs might look like they have
// their documentation comments and clap-args are mixed up, but they have to
// be that way in order to work-around a limitation of clap:
// https://jwodder.github.io/kbits/posts/clap-bool-negate/
// https://github.com/clap-rs/clap/issues/815bug)]
#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[group(id = "SelectionOptions")]
pub struct SelectionOptions {
    /// Filter out modules (e.g. `mod foo`, `mod foo {}`) from graph.
    #[clap(long = "no-modules")]
    pub no_modules: bool,

    /// Filter out "use" edges from graph.
    #[arg(long = "no-uses")]
    pub no_uses: bool,

    /// Filter out extern items (e.g. `core`, `std`, third-party crates) from extern crates from graph.
    #[arg(long = "no-externs")]
    pub no_externs: bool,

    /// Filter out types (e.g. structs, unions, enums) from graph.
    #[arg(long = "no-types")]
    pub no_types: bool,

    /// Filter out traits (e.g. trait, unsafe trait) from graph.
    #[arg(long = "no-traits")]
    pub no_traits: bool,

    /// Filter out functions (e.g. fns, async fns, const fns) from graph.
    #[arg(long = "no-fns")]
    pub no_fns: bool,

    /// Filter out tests (e.g. `#[test] fn …`) from graph.
    #[arg(long = "no-tests")]
    pub no_tests: bool,
}
