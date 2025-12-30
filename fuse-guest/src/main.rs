//! FUSE Guest Program Entry Point
//!
//! This is the main entry point for the RISC Zero guest program.

#![no_main]
#![no_std]

extern crate alloc;

use risc0_zkvm::guest::env;
use fuse_guest::checker;

risc0_zkvm::guest::entry!(main);

fn main() {
    // Execute the checker
    let journal_output = checker::execute_checker();
    
    // Commit complete output to journal
    env::commit(&journal_output);
}
