extern crate pnet_macros;

use pnet_macros_support::packet::PrimitiveValues;
use pnet_macros_support::types::*;
use std::mem;
//use pnet_macros::packet;
use packet::hep::pnet_macros::packet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HepChunkTypeId(pub u16);

impl HepChunkTypeId {
    /// Create a new HepChunkTypeId
    pub fn new(value: u16) -> Self {
        HepChunkTypeId(value)
    }
}

impl PrimitiveValues for HepChunkTypeId {
    type T = (u16,);
    fn to_primitive_values(&self) -> (u16,) {
        (self.0,)
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod HepChunkTypeIds {
    use super::HepChunkTypeId;
    pub const IpProtocolFamily: HepChunkTypeId = HepChunkTypeId(0x0001);
    pub const IpProtocolId: HepChunkTypeId = HepChunkTypeId(0x0002);
    pub const Ipv4SourceAddress: HepChunkTypeId = HepChunkTypeId(0x0003);
    pub const Ipv4TargetAddress: HepChunkTypeId = HepChunkTypeId(0x0004);
    pub const Ipv6SourceAddress: HepChunkTypeId = HepChunkTypeId(0x0005);
    pub const Ipv6TargetAddress: HepChunkTypeId = HepChunkTypeId(0x0006);
    pub const SourcePort: HepChunkTypeId = HepChunkTypeId(0x0007);
    pub const TargetPort: HepChunkTypeId = HepChunkTypeId(0x0008);
    pub const TimestampSec: HepChunkTypeId = HepChunkTypeId(0x0009);
    pub const TimestampMicrosecOffset: HepChunkTypeId = HepChunkTypeId(0x000a);
    pub const ProtocolType: HepChunkTypeId = HepChunkTypeId(0x000b);
    pub const CaptureAgentId: HepChunkTypeId = HepChunkTypeId(0x000c);
    pub const KeepAliveTimer: HepChunkTypeId = HepChunkTypeId(0x000d);
    pub const AuthKey: HepChunkTypeId = HepChunkTypeId(0x000e);
    pub const PacketPayload: HepChunkTypeId = HepChunkTypeId(0x000f);
    pub const GzipPacketPayload: HepChunkTypeId = HepChunkTypeId(0x0010);
    pub const CorrelationId: HepChunkTypeId = HepChunkTypeId(0x0011);
    pub const VlanId: HepChunkTypeId = HepChunkTypeId(0x0012);
    pub const GroupId: HepChunkTypeId = HepChunkTypeId(0x0013);
    pub const SourceMac: HepChunkTypeId = HepChunkTypeId(0x0014);
    pub const TargetMac: HepChunkTypeId = HepChunkTypeId(0x0015);
    pub const EthernetType: HepChunkTypeId = HepChunkTypeId(0x0016);
    pub const TcpFlag: HepChunkTypeId = HepChunkTypeId(0x0017);
    // ... reserved chunks 0x18..0x1f
    pub const MosValue: HepChunkTypeId = HepChunkTypeId(0x0020);
    pub const RFactor: HepChunkTypeId = HepChunkTypeId(0x0021);
    pub const GeoLocation: HepChunkTypeId = HepChunkTypeId(0x0022);
    pub const Jitter: HepChunkTypeId = HepChunkTypeId(0x0023);
    pub const TransactionType: HepChunkTypeId = HepChunkTypeId(0x0024);
    pub const PayloadJson: HepChunkTypeId = HepChunkTypeId(0x0025);
}

//
// HEP chunk
//
#[packet]
pub struct HepChunk {
    pub vendor_id: u16be,
    #[construct_with(u16)]
    pub type_id: HepChunkTypeId,
    pub length: u16be,
    #[payload]
    #[length_fn = "chunk_length"]
    pub payload: Vec<u8>,
}

fn chunk_length<'a>(chunk: &HepChunkPacket<'a>) -> usize {
    chunk.get_length() as usize - 6
}

//
// HEP chunk subtypes
//

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod IpProtocolFamilies {
    pub const IPv4: u8 = 0x02;
    pub const IPv6: u8 = 0x0a;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod IpProtocolIds {
    pub const TCP: u8 = 0x06;
    pub const UDP: u8 = 0x11;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod SubProtocols {
    pub const Reserved: u8 = 0x00;
    pub const SIP: u8 = 0x01;
    pub const XMPP: u8 = 0x02;
    pub const SDP: u8 = 0x03;
}

pub const HEP_ID: u32 = 0x48455033; // 'HEP3'

/// HEP3 packet
#[packet]
pub struct Hep {
    #[construct_with(HEP_ID)] // @fixme
    hep_id: u32be,
    total_length: u16be, // payload.size + 6
    #[length_fn = "chunks_length"]
    chunks: Vec<HepChunk>,
    #[payload]
    #[length = "0"]
    payload: Vec<u8>,
}

fn chunks_length<'a>(hep: &HepPacket<'a>) -> usize {
    hep.get_total_length() as usize - 6
}

pub struct HepBuilder<'a> {
    hep: &'a mut MutableHepPacket<'a>,
    chunks: Vec<HepChunk>,
}

impl<'a> HepBuilder<'a> {
    pub fn new(hep: &'a mut MutableHepPacket<'a>) -> Self {
        Self {
            hep: hep,
            chunks: vec![],
        }
    }

    pub fn add_chunk(&mut self, hc: HepChunk) -> &mut Self {
        self.chunks.push(hc);
        self
    }

    pub fn build(&mut self) /*-> HepPacket<'a>*/
    {
        let total_length = self.chunks
            .iter()
            .map(|item| item.length)
            .fold(0, |acc, len| acc + len);

        self.hep.set_hep_id(HEP_ID);
        self.hep.set_total_length(total_length + 6);
        self.hep.set_chunks(&self.chunks);
        // self.hep.to_immutable()
    }
}

macro_rules! hep_chunk {
    ($name:ident, $ty:ty, $typeId:ident) => {
        pub fn $name(arg: $ty) -> HepChunk {
            HepChunk {
                vendor_id: 0x0000,
                type_id: HepChunkTypeIds::$typeId,
                length: (6 + mem::size_of::<$ty>()) as u16,
                payload: any_as_u8_slice(&arg).to_vec(),
            }
        }
    };
}

macro_rules! hep_chunk_vec {
    ($name:ident, $typeId:ident) => {
        pub fn $name(arg: Vec<u8>) -> HepChunk {
            HepChunk {
                vendor_id: 0x0000,
                type_id: HepChunkTypeIds::$typeId,
                length: (6 + arg.len()) as u16,
                payload: arg,
            }
        }
    };
}

// https://stackoverflow.com/a/42186553/145434 see note 3) about `unsafe` behavior:
// function is marked unsafe because any padding bytes in the struct may be uninitialized memory
// (giving undefined behavior). If there were a way to ensure input arguments used only structs
// which were #[repr(packed)], then it could be safe.
// Otherwise the function is fairly safe, it prevents buffer over-run since the output is
// read-only, fixed number of bytes, and its lifetime is bound to the input.
fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

pub struct Chunk;

#[allow(non_snake_case)]
impl Chunk {
    hep_chunk!(ipProtocolFamily, u8, IpProtocolFamily);
    hep_chunk!(ipProtocolId, u8, IpProtocolId);
    hep_chunk!(ipv4SourceAddress, [u8; 4], Ipv4SourceAddress);
    hep_chunk!(ipv4TargetAddress, [u8; 4], Ipv4TargetAddress);
    hep_chunk!(ipv6SourceAddress, [u8; 16], Ipv6SourceAddress);
    hep_chunk!(ipv6TargetAddress, [u8; 16], Ipv6TargetAddress);
    hep_chunk!(sourcePort, u16, SourcePort);
    hep_chunk!(targetPort, u16, TargetPort);
    hep_chunk!(timestampSec, u32, TimestampSec);
    hep_chunk!(timestampMicrosecOffset, u32, TimestampMicrosecOffset);
    hep_chunk!(protocolType, u8, ProtocolType);
    hep_chunk!(captureAgentId, u32, CaptureAgentId);
    hep_chunk!(keepAliveTimer, u16, KeepAliveTimer);
    hep_chunk_vec!(authKey, AuthKey);
    hep_chunk_vec!(packetPayload, PacketPayload);
    hep_chunk_vec!(gzipPacketPayload, GzipPacketPayload);
    hep_chunk_vec!(correlationId, CorrelationId);
    hep_chunk!(vlanId, u16, VlanId);
    hep_chunk_vec!(groupId, GroupId);
    hep_chunk!(sourceMac, u64, SourceMac); // u48 really
    hep_chunk!(targetMac, u64, TargetMac); // u48 really
    hep_chunk!(ethernetType, u16, EthernetType);
    hep_chunk!(tcpFlag, u16, TcpFlag);
    // ... reserved chunks 0x18..0x1f
    hep_chunk!(mosValue, u16, MosValue);
    hep_chunk!(rFactor, u16, RFactor);
    hep_chunk_vec!(geoLocation, GeoLocation);
    hep_chunk!(jitter, u32, Jitter);
    hep_chunk_vec!(transactionType, TransactionType);
    hep_chunk!(payloadJson, String, PayloadJson); // hep_chunk_str!()
}

#[cfg(test)]
mod tests {
    use super::Chunk;
    use pnet_macros_support::packet::Packet;

    #[test]
    fn create_payload_chunk() {
        let c = Chunk::packetPayload(vec![1, 2, 3, 4, 5u8]);
        // [0, 240, 224, 6, 1, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 30, 0, 0, 0]
        // [16, 208, 192, 6, 1, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 10, 0, 0, 0]
        // let slice = c.packet();
        // assert_eq!(slice, []);
    }
}
