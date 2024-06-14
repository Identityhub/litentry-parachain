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

import "../libraries/Json.sol";
import "../libraries/JsmnSolLib.sol";

contract JsonTest {
    function queryBool() public returns (bool, bool) {
        string
            memory json = '{"key": [{"bool_value": true}]} ';
        return Json.queryBool(json, "/key/0/bool_value");
    }

    function queryInt() public returns (bool, int256) {
        string
            memory json = '{"key": [{"int_value": 10}]} ';
        return Json.queryInt(json, "/key/0/int_value");
    }
}
