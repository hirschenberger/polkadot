// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::{Pallet as Ump, *};
use frame_system::RawOrigin;

frame_benchmarking::benchmarks! {
    service_overweight {
		let a = ParaId::from(1991);
		let msg = (300u32, "a_msg_1").encode();
        crate::ump::tests::queue_upward_msg(a, msg.clone());
		Ump::process_pending_upward_messages();
    }: _(RawOrigin::Root, 0, 500)  verify {}
}

frame_benchmarking::impl_benchmark_test_suite!(
    Ump,
    crate::mock::new_test_ext(
			crate::ump::tests::GenesisConfigBuilder {
                ump_service_total_weight: 500,
                ump_max_individual_weight: 300,
                ..Default::default()
            }.build()),
    crate::mock::Test
);
