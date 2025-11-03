/**
 * @file old_rl.rs
 * @brief Rust conversion of Random Lottery contract from C++
 *
 * This module declares the RL (Random Lottery) contract which:
 * - Sells tickets during a SELLING epoch.
 * - Draws a pseudo-random winner when the epoch ends.
 * - Distributes fees (team, distribution, burn, winner).
 * - Records winners' history in a ring-like buffer.
 */
use crate::common::{
    EState, Id, WinnerInfo, RL_MAX_NUMBER_OF_PLAYERS, RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlayerHashSet {
    pub players: [Id; RL_MAX_NUMBER_OF_PLAYERS],
    pub occupation_flags: [u64; (RL_MAX_NUMBER_OF_PLAYERS * 2 + 63) / 64],
    pub population: u64,
    pub mark_removal_counter: u64,
}

impl Default for PlayerHashSet {
    fn default() -> Self {
        Self {
            players: [Id::default(); RL_MAX_NUMBER_OF_PLAYERS],
            occupation_flags: [0u64; (RL_MAX_NUMBER_OF_PLAYERS * 2 + 63) / 64],
            population: 0,
            mark_removal_counter: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct OldRL {
    pub team_address: Id,
    pub owner_address: Id,
    pub team_fee_percent: u8,
    pub distribution_fee_percent: u8,
    pub winner_fee_percent: u8,
    pub burn_percent: u8,
    pub ticket_price: u64,
    pub players: PlayerHashSet,
    pub winners: [WinnerInfo; RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY],
    pub winners_info_next_empty_index: u64,
    pub current_state: EState,
}

impl Default for OldRL {
    fn default() -> Self {
        Self {
            team_address: Id::zero(),
            owner_address: Id::zero(),
            team_fee_percent: 0,
            distribution_fee_percent: 0,
            winner_fee_percent: 0,
            burn_percent: 0,
            ticket_price: 0,
            players: PlayerHashSet::default(),
            winners: [WinnerInfo::default(); RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY],
            winners_info_next_empty_index: 0,
            current_state: EState::default(),
        }
    }
}

impl Display for OldRL {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "═══════════════════════════════════════════════════════════"
        )?;
        writeln!(f, "           OldRL STRUCTURE CONTENTS")?;
        writeln!(
            f,
            "═══════════════════════════════════════════════════════════\n"
        )?;

        // Addresses
        writeln!(f, "📍 ADDRESSES:")?;
        writeln!(f, "  Team Address:  {}", self.team_address)?;
        writeln!(f, "  Owner Address: {}", self.owner_address)?;

        // Fees
        writeln!(f, "\n💰 FEES:")?;
        writeln!(f, "  Team:          {}%", self.team_fee_percent)?;
        writeln!(f, "  Distribution:  {}%", self.distribution_fee_percent)?;
        writeln!(f, "  Winner:        {}%", self.winner_fee_percent)?;
        writeln!(f, "  Burn:          {}%", self.burn_percent)?;

        // Ticket price
        writeln!(f, "\n🎫 TICKETS:")?;
        writeln!(f, "  Ticket price:  {} units", self.ticket_price)?;

        // Players
        writeln!(f, "\n👥 PLAYERS:")?;
        let active_players = self.players.players.iter().filter(|p| !p.is_zero()).count();
        writeln!(f, "  Active players: {}", active_players)?;

        writeln!(f, "  Players list:")?;
        for (i, player) in self.players.players.iter().enumerate() {
            if !player.is_zero() {
                writeln!(f, "    {}. {}", i + 1, player)?;
            }
        }

        // Winners
        writeln!(f, "\n🏆 WINNERS HISTORY:")?;
        writeln!(
            f,
            "  Next index:     {}",
            self.winners_info_next_empty_index
        )?;

        writeln!(f, "  Winners list:")?;
        for (i, winner) in self.winners.iter().enumerate() {
            if !winner.winner_address.is_zero() {
                writeln!(f, "    {}. Address: {}", i + 1, winner.winner_address)?;
                writeln!(f, "       Prize:   {} units", winner.revenue)?;
                writeln!(f, "       Epoch: {}, Tick: {}", winner.epoch, winner.tick)?;
            }
        }

        // State
        writeln!(f, "\n⚙️  STATE:")?;
        writeln!(f, "  Current state: {:?}", self.current_state)?;

        writeln!(
            f,
            "\n═══════════════════════════════════════════════════════════"
        )
    }
}
