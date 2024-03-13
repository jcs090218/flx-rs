/**
 * $File: lib.rs $
 * $Date: 2021-10-17 20:22:21 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2021 by Shen, Jen-Chieh $
 */
mod search;

pub use search::{find_best_match, get_heatmap_str, score, Result};
