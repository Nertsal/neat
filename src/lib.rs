use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod client;
mod connection_gene;
mod gene;
mod genome;
mod id_generator;
mod neat;
mod node_gene;
mod species;

use client::*;
use connection_gene::*;
use gene::*;
use genome::*;
pub use id_generator::Id;
use id_generator::*;
pub use neat::*;
use node_gene::*;
use species::*;
