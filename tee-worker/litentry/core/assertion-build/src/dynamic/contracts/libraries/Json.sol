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

import "./JsmnSolLib.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.9.0/contracts/utils/Strings.sol";

library Json {
    function queryString(string memory json, string memory path)
        internal
        pure
        returns (bool success, string memory value)
    {
        return queryValueToken(json, path);
    }

    function queryBool(string memory json, string memory path)
        internal
        pure
        returns (bool success, bool value)
    {
        (bool querySuccess, string memory tokenValue) = queryValueToken(
            json,
            path
        );
        if (querySuccess) {
            success = true;
            value = JsmnSolLib.parseBool(tokenValue);
        } else {
            success = false;
        }
    }

    function queryInt(string memory json, string memory path)
        internal
        pure
        returns (bool success, int256 value)
    {
        (bool querySuccess, string memory tokenValue) = queryValueToken(
            json,
            path
        );
        if (querySuccess) {
            success = true;
            value = JsmnSolLib.parseInt(tokenValue);
        } else {
            success = false;
        }
    }

    function queryInt(
        string memory json,
        string memory path,
        uint256 decimals
    ) internal pure returns (bool success, int256 value) {
        (bool querySuccess, string memory tokenValue) = queryValueToken(
            json,
            path
        );
        if (querySuccess) {
            success = true;
            value = JsmnSolLib.parseInt(tokenValue, decimals);
        } else {
            success = false;
        }
    }

    function queryValueToken(string memory json, string memory path)
        internal
        pure
        returns (bool success, string memory value)
    {
        (
            uint256 parseSuccess,
            JsmnSolLib.Token[] memory tokens,
            uint256 elements
        ) = JsmnSolLib.parse(json, 10000);


        if (parseSuccess == 0) {
            for (uint256 i = 0; i < tokens.length; i++) {
                if (!tokens[i].key && Strings.equal(tokens[i].path, path)) {
                    value = JsmnSolLib.getBytes(
                        json,
                        tokens[i].start,
                        tokens[i].end
                    );
                    success = true;
                    break;
                }
            }
        } else {
            success = false;
        }
    }
}
