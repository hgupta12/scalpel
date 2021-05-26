//! Ethernet Layer

use core::convert::TryInto;

// FIXME: Should work with no_std
use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::types::{EtherType, LayerCreatorFn, MACAddress, ETHERTYPE_IP, ETHERTYPE_IP6};
use crate::Error;
use crate::Layer;

pub const ETH_HEADER_LEN: usize = 14_usize;

lazy_static! {
    /// A Map maintaining EtherType -> Creator fns for Layer Creators of L3 Layers.
    ///
    /// The creator function simply creates a `default` L3 struct that implements the dissector
    /// for the Layer.
    static ref ETHERTYPES_MAP: RwLock<HashMap<EtherType, LayerCreatorFn>> =
        RwLock::new(HashMap::new());
}

/// Registers well-known EtherType values
pub fn register_defaults() -> Result<(), Error> {
    use super::ipv4::IPv4;
    use super::ipv6::IPv6;

    register_ethertype(ETHERTYPE_IP.clone(), IPv4::creator)?;
    register_ethertype(ETHERTYPE_IP6.clone(), IPv6::creator)?;

    Ok(())
}

/// Register for a given EtherType
///
/// A Layer that would handle subsequent decoding for a given Ethertype, should register itself
/// by calling this function.
///
pub fn register_ethertype(eth_type: EtherType, layer: fn() -> Box<dyn Layer>) -> Result<(), Error> {
    let mut map = ETHERTYPES_MAP.write().unwrap();
    if map.contains_key(&eth_type) {
        return Err(Error::RegisterError);
    }
    map.insert(eth_type, layer);

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct Ethernet {
    src_mac: MACAddress,
    dst_mac: MACAddress,
    ethertype: EtherType,
}

impl Ethernet {
    pub fn creator() -> Box<dyn Layer> {
        Box::new(Ethernet::default())
    }
}

impl Layer for Ethernet {
    fn from_u8(&mut self, bytes: &[u8]) -> Result<(Option<Box<dyn Layer>>, usize), Error> {
        if bytes.len() < ETH_HEADER_LEN {
            return Err(Error::TooShort);
        }
        self.src_mac = bytes[0..6].try_into()?;
        self.dst_mac = bytes[6..12].try_into()?;
        self.ethertype = EtherType((bytes[12] as u16) << 8 | bytes[13] as u16);
        let map = ETHERTYPES_MAP.read().unwrap();
        let layer = map.get(&self.ethertype);
        if layer.is_none() {
            return Ok((None, ETH_HEADER_LEN));
        } else {
            let l3_creator = layer.unwrap();
            return Ok((Some(l3_creator()), ETH_HEADER_LEN));
        }
    }

    fn name(&self) -> &str {
        "Ethernet"
    }

    fn short_name(&self) -> &str {
        "eth"
    }
}
