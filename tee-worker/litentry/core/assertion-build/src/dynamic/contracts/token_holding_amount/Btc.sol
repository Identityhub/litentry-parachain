// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;
import "../libraries/Identities.sol";
import "../libraries/Utils.sol";
import {TokenHoldingAmount} from "./TokenHoldingAmount.sol";
import {BlockchainInfoClient} from "./BlockchainInfoClient.sol";

contract Btc is TokenHoldingAmount {
    constructor() {}

    function getTokenDecimals() internal pure override returns (uint8) {
        //check that
        return 8;
    }

    function getTokenName() internal pure override returns (string memory) {
        return "btc";
    }

    function getTokenRanges()
        internal
        pure
        override
        returns (uint256[] memory)
    {
        // [0.0, 0.001, 0.1, 0.3, 0.6, 1.0, 2.0, 5.0, 10.0, 15.0, 25.0, 30.0, 40.0, 50.0];

        uint256[] memory ranges = new uint256[](14);
        ranges[0] = 0;
        ranges[1] = 1;
        ranges[2] = 100;
        ranges[3] = 300;
        ranges[4] = 600;
        ranges[5] = 1000;
        ranges[6] = 2000;
        ranges[7] = 5000;
        ranges[8] = 10000;
        ranges[9] = 15000;
        ranges[10] = 25000;
        ranges[11] = 30000;
        ranges[12] = 40000;
        ranges[12] = 50000;
        return ranges;
    }

    function checkAssertion() public returns (string[] memory) {
        (uint256 index, uint256 min, int256 max) = calculateRange(185123167511);
        return assembleAssertions(min, max);
    }

    function queryBalance(
        Identity memory identity,
        uint32 network,
        string[] memory
    ) internal virtual override returns (uint256) {
        (bool identityToStringSuccess, string memory identityString) = Utils
            .identityToString(network, identity.value);

        if (!identityToStringSuccess) {
            return 0;
        }

        string[] memory accounts = new string[](1);
        accounts[0] = identityString;

        string memory url = "http://localhost:19528/blockchain_info/multiaddr";
        (bool balanceSuccess, int64 balance) = BlockchainInfoClient
            .getMultiAddress(url, accounts);
        if (balanceSuccess) {
            return uint256(uint64(balance));
        } else {
            return 0;
        }
    }

    function getTokenNetworks() internal pure returns (uint32[] memory) {
        uint32[] memory networks = new uint32[](5);
        networks[0] = Web3Networks.BitcoinP2tr;
        networks[1] = Web3Networks.BitcoinP2pkh;
        networks[2] = Web3Networks.BitcoinP2sh;
        networks[3] = Web3Networks.BitcoinP2wpkh;
        networks[4] = Web3Networks.BitcoinP2wsh;
        return networks;
    }

    function isSupportedNetwork(uint32 network)
        internal
        pure
        override
        returns (bool)
    {
        return
            network == Web3Networks.BitcoinP2tr ||
            network == Web3Networks.BitcoinP2pkh ||
            network == Web3Networks.BitcoinP2sh ||
            network == Web3Networks.BitcoinP2wpkh ||
            network == Web3Networks.BitcoinP2wsh;
    }
}
