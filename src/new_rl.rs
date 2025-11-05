use crate::common::{EState, Id, RL_MAX_NUMBER_OF_PLAYERS, RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY};
use crate::old_rl::OldRL;
use std::fmt::{self, Display, Formatter};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct WinnerInfoNew {
    pub winner_address: Id,
    pub revenue: u64,
    pub tick: u32,
    pub epoch: u16,
    pub day_of_week: u8,
}

impl Default for WinnerInfoNew {
    fn default() -> Self {
        Self {
            winner_address: Id::zero(),
            revenue: 0,
            epoch: 0,
            tick: 0,
            day_of_week: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextEpochData {
    pub new_price: u64,
    pub schedule: u8,
}

impl Default for NextEpochData {
    fn default() -> Self {
        // Zero the entire structure, including padding bytes.
        unsafe { MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// New Random Lottery contract state structure
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewRL {
    /// Circular buffer storing the history of winners.
    /// Maximum capacity is defined by RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY.
    pub winners: [WinnerInfoNew; RL_MAX_NUMBER_OF_WINNERS_IN_HISTORY],

    /// Set of players participating in the current lottery epoch.
    /// Maximum capacity is defined by RL_MAX_NUMBER_OF_PLAYERS.
    pub players: [Id; RL_MAX_NUMBER_OF_PLAYERS],

    /// Address of the team managing the lottery contract. Initialized to zero address.
    pub team_address: Id,

    /// Address of the owner of the lottery contract. Initialized to zero address.
    pub owner_address: Id,

    /// Data structure for deferred changes to apply at the end of the epoch.
    pub next_epoch_data: NextEpochData,

    /// Price of a single lottery ticket. Value is in the smallest currency unit.
    pub ticket_price: u64,

    /// Number of players (tickets sold) in the current epoch.
    pub player_counter: u64,

    /// Index pointing to the next empty slot in the winners array.
    /// Used for maintaining the circular buffer of winners.
    pub winners_counter: u64,

    /// Date/time guard for draw operations. lastDrawDateStamp prevents
    /// more than one action per calendar day (UTC).
    pub last_draw_day: u8,
    pub last_draw_hour: u8,
    pub last_draw_date_stamp: u32,

    /// Percentage of the revenue allocated to the team. [0..100]
    pub team_fee_percent: u8,
    /// Percentage of the revenue allocated for distribution. [0..100]
    pub distribution_fee_percent: u8,
    /// Percentage of the revenue allocated to the winner. auto remainder
    pub winner_fee_percent: u8,
    /// Percentage of the revenue to be burned. [0..100]
    pub burn_percent: u8,

    /// Schedule bitmask: bit 0 = WEDNESDAY, 1 = THURSDAY, ..., 6 = TUESDAY.
    /// If a bit is set, a draw may occur on that day (subject to drawHour and daily guard).
    /// Wednesday also follows the "Two-Wednesdays rule".
    pub schedule: u8,

    /// UTC hour [0..23] when a draw is allowed to run (daily time gate).
    pub draw_hour: u8,

    /// Current state of the lottery contract. SELLING/LOCKED
    pub current_state: EState,
}

impl From<&OldRL> for NewRL {
    fn from(old: &OldRL) -> Self {
        let mut new_rl = unsafe { MaybeUninit::<NewRL>::zeroed().assume_init() };

        new_rl.winners = old
            .winners
            .iter()
            .map(|w| WinnerInfoNew {
                winner_address: w.winner_address,
                revenue: w.revenue,
                epoch: w.epoch,
                tick: w.tick,
                day_of_week: 0, // Default value for new field
            })
            .collect::<Vec<WinnerInfoNew>>()
            .try_into()
            .unwrap();

        new_rl.players = old.players.players;
        new_rl.team_address = old.team_address;
        new_rl.owner_address = old.owner_address;
        new_rl.ticket_price = old.ticket_price;
        new_rl.winners_counter = old.winners_info_next_empty_index;
        new_rl.team_fee_percent = old.team_fee_percent;
        new_rl.distribution_fee_percent = old.distribution_fee_percent;
        new_rl.winner_fee_percent = old.winner_fee_percent;
        new_rl.burn_percent = old.burn_percent;
        new_rl.current_state = old.current_state;

        new_rl.next_epoch_data = NextEpochData::default();
        new_rl.player_counter = 0;
        new_rl.last_draw_day = 0;
        new_rl.last_draw_hour = 0;
        new_rl.last_draw_date_stamp = 0;
        new_rl.schedule = 0;
        new_rl.draw_hour = 0;

        new_rl
    }
}

impl Default for NewRL {
    fn default() -> Self {
        let mut new_rl = unsafe { MaybeUninit::<NewRL>::zeroed().assume_init() };
        new_rl.current_state = EState::Locked;
        new_rl
    }
}

impl Display for NewRL {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\n═══════════════════════════════════════════════════════════"
        )?;
        writeln!(f, "           NewRL STRUCTURE CONTENTS")?;
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

        // Counters and schedule
        writeln!(f, "\n📊 COUNTERS:")?;
        writeln!(f, "  Players (tickets sold): {}", self.player_counter)?;
        writeln!(f, "  Winners in history:     {}", self.winners_counter)?;

        writeln!(f, "\n🕒 SCHEDULE:")?;
        writeln!(f, "  Schedule bitmask: 0b{:08b}", self.schedule)?;
        writeln!(f, "  Draw hour (UTC): {}", self.draw_hour)?;
        writeln!(f, "  Last draw day: {}", self.last_draw_day.to_string())?;
        writeln!(f, "  Last draw hour: {}", self.last_draw_hour.to_string())?;
        writeln!(f, "  Last draw date stamp: {}", self.last_draw_date_stamp)?;

        // Players
        writeln!(f, "\n👥 PLAYERS:")?;
        writeln!(f, "  Players list:")?;
        for (i, player) in self.players.iter().enumerate() {
            if !player.is_zero() {
                writeln!(f, "    {}. {}", i + 1, player)?;
            }
        }

        // Winners
        writeln!(f, "\n🏆 WINNERS HISTORY:")?;
        writeln!(f, "  Winners list:")?;
        for (i, winner) in self.winners.iter().enumerate() {
            if !winner.winner_address.is_zero() {
                writeln!(f, "    {}. Address: {}", i + 1, winner.winner_address)?;
                writeln!(f, "       Prize:   {} units", winner.revenue)?;
                writeln!(f, "       Epoch: {}, Tick: {}", winner.epoch, winner.tick)?;
                writeln!(f, "       Day of week: {}", winner.day_of_week)?;
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
