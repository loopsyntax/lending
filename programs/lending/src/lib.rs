pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6E3EJE7gcpDnhvmx2qFo5SXDNnAFCwyFew8JoTarNXas");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize_bank(
        ctx: Context<InitializeBank>,
        liquidation_threshold: u64,
        max_ltw: u64,
    ) -> Result<()> {
        initialize_bank::handler_initialize_bank(ctx, liquidation_threshold, max_ltw)
    }
}
