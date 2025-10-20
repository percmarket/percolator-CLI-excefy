#![no_std]

pub mod state;
pub mod instructions;
pub mod pda;

#[cfg(feature = "bpf-entrypoint")]
mod entrypoint;

#[cfg(test)]
mod tests;

// Panic handler for no_std builds (not needed in tests)
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub use state::*;
pub use instructions::SlabInstruction;

pinocchio_pubkey::declare_id!("SLabZ6PsDLh2X6HzEoqxFDMqCVcJXDKCNEYuPzUvGPk");
