// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
use std::convert::From;

pub enum ECallCommand {
    StartService,
    InitEnclave,
    FinalizeEnclave,
    RunEnclaveUnitTest,
    ServeConnection,
    Unimplemented,
}

impl From<u32> for ECallCommand {
    #[inline]
    fn from(cmd: u32) -> ECallCommand {
        match cmd {
            0x0000_1000 => ECallCommand::StartService,
            0x0000_1001 => ECallCommand::InitEnclave,
            0x0000_1002 => ECallCommand::FinalizeEnclave,
            0x0000_1003 => ECallCommand::RunEnclaveUnitTest,
            0x0000_1004 => ECallCommand::ServeConnection,
            _ => ECallCommand::Unimplemented,
        }
    }
}

impl Into<u32> for ECallCommand {
    #[inline]
    fn into(self) -> u32 {
        match self {
            ECallCommand::StartService => 0x0000_1000,
            ECallCommand::InitEnclave => 0x0000_1001,
            ECallCommand::FinalizeEnclave => 0x0000_1002,
            ECallCommand::RunEnclaveUnitTest => 0x0000_1003,
            ECallCommand::ServeConnection => 0x0000_1004,
            ECallCommand::Unimplemented => 0xffff_ffff,
        }
    }
}