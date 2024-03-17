// Lprs - A local CLI password manager
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

use clap::Args;

use crate::{
    password::{Vault, Vaults},
    LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Add {
    #[command(flatten)]
    vault_info: Vault<Plain>,
}

impl RunCommand for Add {
    fn run(&self, mut vault_manager: Vaults<Plain>) -> LprsResult<()> {
        vault_manager.add_vault(self.vault_info.clone());
        vault_manager.try_export()
    }
}
