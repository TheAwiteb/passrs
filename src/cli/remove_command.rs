// Lprs - A local CLI vaults manager. For human and machine use
// Copyright (C) 2024  Awiteb <a@4rs.nl>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/gpl-3.0.html>.

use std::num::NonZeroU64;

use clap::Args;

use crate::{vault::Vaults, LprsCommand, LprsError, LprsResult};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Remove command, used to remove a vault from the vaults file
pub struct Remove {
    /// The password index
    index: NonZeroU64,

    /// Force remove, will not return error if there is no password with this
    /// index
    #[arg(short, long)]
    force: bool,
}

impl LprsCommand for Remove {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        let index = (self.index.get() - 1) as usize;
        log::debug!("Removing vault at index: {index}");

        if index > vault_manager.vaults.len() {
            if self.force {
                log::error!(
                    "The index is greater than the passwords counts, but the force flag is enabled"
                );
            } else {
                return Err(LprsError::Other(
                    "The index is greater than the passwords counts".to_owned(),
                ));
            }
        } else {
            vault_manager.vaults.remove(index);
            vault_manager.try_export()?;
        }
        Ok(())
    }
}
