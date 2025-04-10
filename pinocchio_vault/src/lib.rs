#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod errors;
pub mod instructions;
pub mod states;

pinocchio_pubkey::declare_id!("Hsj2dmft6MSJmj8VoaW7hWp9h3G4qf4B58LFfz5amPUz");
