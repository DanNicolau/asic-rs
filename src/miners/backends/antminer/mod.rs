pub mod v2020;

use crate::data::device::MinerModel;
use crate::miners::backends::traits::*;
use std::net::IpAddr;
use v2020::AntMinerV2020;

pub struct AntMiner;

impl AntMiner {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(ip: IpAddr, model: MinerModel, _: Option<semver::Version>) -> Box<dyn Miner> {
        Box::new(AntMinerV2020::new(ip, model))
    }

    pub fn new_with_auth(
        ip: IpAddr,
        model: MinerModel,
        _: Option<semver::Version>,
        username: String,
        password: String,
    ) -> Box<dyn Miner> {
        use crate::data::device::MinerFirmware;
        Box::new(AntMinerV2020::with_auth(
            ip,
            model,
            MinerFirmware::Stock,
            username,
            password,
        ))
    }
}
