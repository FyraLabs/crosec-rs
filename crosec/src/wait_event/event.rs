use async_std::io::ReadExt;
use std::io;
use std::mem::size_of;

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

#[derive(Debug, FromPrimitive, Clone, Copy)]
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

    fn parse_event(&self, event: &mut Vec<u8>) -> EcMkbpEvent {
        debug_assert_eq!(event[0], *self as u8);
        event.remove(0);
        let data = event;
        match self {
            EcMkbpEventType::Fingerprint => {
                EcMkbpEvent::Fingerprint(from_bytes::<EcMkbpEventFingerprint>(&data).to_owned())
            }
            EcMkbpEventType::HostEvent => {
                EcMkbpEvent::HostEvent(from_bytes::<EcMkbpEventHostEvent>(&data).to_owned())
            }
            event_type => panic!("{event_type:#?} from_bytes not implemented yet"),
        }
    }

    pub(crate) fn read_sync<T: std::io::Read>(&self, stream: &mut T) -> io::Result<EcMkbpEvent> {
        let mut event = vec![Default::default(); size_of::<Self>() + self.data_size()];
        stream.read_exact(&mut event)?;
        Ok(self.parse_event(&mut event))
    }

    pub(crate) async fn read_async<T: async_std::io::Read + Unpin>(
        &self,
        stream: &mut T,
    ) -> io::Result<EcMkbpEvent> {
        let mut event = vec![Default::default(); size_of::<Self>() + self.data_size()];
        stream.read_exact(&mut event).await?;
        Ok(self.parse_event(&mut event))
    }
}
