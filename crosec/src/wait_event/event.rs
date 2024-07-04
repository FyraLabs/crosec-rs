use async_std::io::ReadExt;
use num::FromPrimitive;
use std::io;
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use bytemuck::{from_bytes, Pod, Zeroable};
use num_derive::FromPrimitive;

use crate::wait_event::fingerprint::EcMkbpEventFingerprint;

use super::host_event::EcMkbpEventHostEvent;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct EcResponseMotionSenseFifoInfo {
    size: u16,
    count: u16,
    timestamp: u32,
    total_lost: u16,
    lost: [u16; 0],
}

#[derive(Debug)]
#[repr(u8)]
pub enum EcMkbpEvent {
    KeyMatrix([u8; 13]),
    HostEvent(EcMkbpEventHostEvent),
    HostEvent64(u64),
    SensorFifo(EcResponseMotionSenseFifoInfo),
    Buttons(u32),
    Switches(u32),
    Fingerprint(EcMkbpEventFingerprint),
    Sysrq(u32),
    CecEvent(u32),
}
impl EcMkbpEvent {
    fn max_event_size() -> usize {
        EcMkbpEventType::iter()
            .map(|e| e.data_size())
            .max()
            .unwrap_or_default()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let event_type = EcMkbpEventType::from_u8(bytes[0]).unwrap();
        event_type.event_from_bytes(bytes[1..1 + event_type.data_size()].to_vec())
    }

    pub(crate) fn read_sync<T: std::io::Read>(stream: &mut T) -> io::Result<Self> {
        let mut buf: Vec<u8> =
            vec![Default::default(); size_of::<EcMkbpEventType>() + Self::max_event_size()];
        let bytes_read = stream.read(&mut buf)?;
        Ok(Self::from_bytes(&buf[..bytes_read]))
    }

    pub(crate) async fn read_async<T: async_std::io::Read + Unpin>(
        stream: &mut T,
    ) -> io::Result<Self> {
        let mut buf: Vec<u8> =
            vec![Default::default(); size_of::<EcMkbpEventType>() + Self::max_event_size()];
        let bytes_read = stream.read(&mut buf).await?;
        Ok(Self::from_bytes(&buf[..bytes_read]))
    }
}

#[derive(Debug, FromPrimitive, Clone, Copy, EnumIter, PartialEq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[repr(u8)]
pub enum EcMkbpEventType {
    KeyMatrix,
    HostEvent,
    SensorFifo,
    Buttons,
    Switches,
    Fingerprint,
    Sysrq,
    HostEvent64,
    CecEvent,
    CecMessage,
    DpAltModeEntered,
    OnlineCalibration,
    Pchg,
}

impl EcMkbpEventType {
    fn data_size(&self) -> usize {
        match self {
            Self::KeyMatrix => size_of::<[u8; 13]>(),
            Self::HostEvent => size_of::<u32>(),
            Self::SensorFifo => size_of::<EcResponseMotionSenseFifoInfo>(),
            Self::Buttons => size_of::<u32>(),
            Self::Switches => size_of::<u32>(),
            Self::Fingerprint => size_of::<u32>(),
            Self::Sysrq => size_of::<u32>(),
            Self::HostEvent64 => size_of::<u64>(),
            Self::CecEvent => size_of::<u32>(),
            _ => 0,
        }
    }

    fn event_from_bytes(&self, event: Vec<u8>) -> EcMkbpEvent {
        match self {
            EcMkbpEventType::Fingerprint => {
                EcMkbpEvent::Fingerprint(from_bytes::<EcMkbpEventFingerprint>(&event).to_owned())
            }
            EcMkbpEventType::HostEvent => {
                EcMkbpEvent::HostEvent(from_bytes::<EcMkbpEventHostEvent>(&event).to_owned())
            }
            event_type => panic!("{event_type:#?} from_bytes not implemented yet"),
        }
    }
}
