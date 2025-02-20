//! Basic, threaded network implementation.
use crate::packets::PlayerInfo;
use crate::serde::SliceCursor;
use crate::{packets, Packet};
use log::trace;
use std::io::{self, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const DEFAULT_PROTOCOL_VERSION: &str = "Terraria279";

const DEFAULT_PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

const DEFAULT_PLAYER_HEALTH: packets::PlayerHp = packets::PlayerHp {
    player_id: 0,
    hp: 100,
    max_hp: 100,
};

const DEFAULT_PLAYER_MANA: packets::PlayerMana = packets::PlayerMana {
    player_id: 0,
    mana: 200,
    max_mana: 200,
};

const DEFAULT_PLAYER_BUFFS: packets::UpdatePlayerBuff = packets::UpdatePlayerBuff {
    player_id: 0,
    buffs: [0u16; 22],
};

const DEFAULT_PLAYER_INVENTORY_SLOT: packets::PlayerInventorySlot = packets::PlayerInventorySlot {
    player_id: 0,
    slot_id: 0,
    stack: 0,
    prefix: 0,
    item_netid: 0,
};

const INVENTORY_SIZE: usize = 260;

const READ_MESSAGE_BUFFER: usize = 16;

pub struct Terraria {
    stream: TcpStream,
    out_buffer: Vec<u8>,
    _reader_thread: thread::JoinHandle<io::Result<()>>,
    packet_rx: mpsc::Receiver<Packet>,
}

pub struct ConnectArgs<A: ToSocketAddrs + std::marker::Copy> {
    pub addr: A,
    pub protocol_version: Option<String>,
    pub player_info: Option<packets::PlayerInfo>,
    pub player_uuid: Option<String>,
    pub player_hp: Option<packets::PlayerHp>,
    pub player_mana: Option<packets::PlayerMana>,
    pub player_buffs: Option<packets::UpdatePlayerBuff>,
    pub player_inventory: Option<[packets::PlayerInventorySlot; INVENTORY_SIZE]>,
}

impl Default for ConnectArgs<&str> {
    fn default() -> Self {
        ConnectArgs {
            addr: "127.0.0.1",
            protocol_version: Some(DEFAULT_PROTOCOL_VERSION.to_string()),
            player_info: Some(PlayerInfo::terry()),
            player_uuid: Some(DEFAULT_PLAYER_UUID.to_string()), // random
            player_hp: Some(DEFAULT_PLAYER_HEALTH),
            player_mana: Some(DEFAULT_PLAYER_MANA),
            player_buffs: Some(DEFAULT_PLAYER_BUFFS),
            player_inventory: Some([DEFAULT_PLAYER_INVENTORY_SLOT; INVENTORY_SIZE]),
        }
    }
}

impl From<()> for ConnectArgs<&str> {
    fn from(_: ()) -> Self {
        ConnectArgs::default()
    }
}

fn reader_worker(
    mut reader: BufReader<TcpStream>,
    sender: mpsc::SyncSender<Packet>,
) -> io::Result<()> {
    let mut packet = vec![0u8; 2];

    loop {
        reader.read_exact(&mut packet)?;
        let mut cursor = SliceCursor::new(&mut packet);
        let len = cursor.read::<u16>().unwrap() as usize;
        cursor.finish();

        packet.reserve(len - 2);
        while packet.len() < len {
            packet.push(0);
        }
        reader.read_exact(&mut packet[2..len])?;

        trace!("< {} : {}", packet[2], crate::utils::HexString(&packet),);
        let mut cursor = SliceCursor::new(&mut packet);
        if sender.send(cursor.read::<Packet>().unwrap()).is_err() {
            break Ok(());
        }
    }
}

impl Terraria {
    pub fn connect_timeout<A: ToSocketAddrs + std::marker::Copy>(connect_args: ConnectArgs<A>, timeout: Duration) -> io::Result<Self> {
        // convert addr to SocketAddr
        let socket_addr = connect_args.addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid address")
        })?;

        let stream = TcpStream::connect_timeout(&socket_addr, timeout)?;
        return Self::connect_shared(connect_args, stream);
    }

    pub fn connect<A: ToSocketAddrs + std::marker::Copy>(connect_args: ConnectArgs<A>) -> io::Result<Self> {        
        // connection
        let stream = TcpStream::connect(connect_args.addr)?;
        return Self::connect_shared(connect_args, stream);
    }

    fn connect_shared<A: ToSocketAddrs + std::marker::Copy>(connect_args: ConnectArgs<A>, stream: TcpStream) -> io::Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        let (packet_tx, packet_rx) = mpsc::sync_channel(READ_MESSAGE_BUFFER);
        let _reader_thread = thread::Builder::new()
            .name("reader thread".to_string())
            .spawn(move || reader_worker(reader, packet_tx))?;
        let mut this = Self {
            stream,
            out_buffer: vec![0; 1024],
            _reader_thread,
            packet_rx,
        };

        // handshake
        this.send_packet(
            &packets::Connect {
                version: connect_args.protocol_version.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            }
            .into(),
        )?;

        this.send_packet(
            &connect_args.player_info.unwrap_or_else(|| PlayerInfo::terry()).into(),
        )?;

        this.send_packet(
            &packets::ClientUuid {
                uuid4: connect_args.player_uuid.unwrap_or_else(|| DEFAULT_PLAYER_UUID.to_string()),
            }
            .into(),
        )?;

        // TODO rename to Health?
        this.send_packet(
            &connect_args.player_hp.unwrap_or_else(|| DEFAULT_PLAYER_HEALTH).into(),
        )?;

        this.send_packet(
            &connect_args.player_mana.unwrap_or_else(|| DEFAULT_PLAYER_MANA).into(),
        )?;

        this.send_packet(
            &connect_args.player_buffs.unwrap_or_else(|| DEFAULT_PLAYER_BUFFS).into(),
        )?;

        for (i, slot) in connect_args.player_inventory.unwrap_or_else(
                || [DEFAULT_PLAYER_INVENTORY_SLOT; INVENTORY_SIZE]
            ).iter().enumerate() {
            let mut slot = slot.clone();
            slot.slot_id = i as i16;
            this.send_packet(&Packet::PlayerInventorySlot(slot))?;
        }

        this.send_packet(&packets::RequestWorldData {}.into())?;

        this.send_packet(
            &packets::RequestEssentialTiles {
                spawn_x: -1,
                spawn_y: -1,
            }
            .into(),
        )?;

        this.send_packet(
            &packets::SpawnPlayer {
                player_id: 0,
                spawn_x: -1,
                spawn_y: -1,
                respawn_time_remaining: 0,
                player_spawn_context: packets::SpawnContext::SpawningIntoWorld,
            }
            .into(),
        )?;

        Ok(this)
    }

    pub fn send_packet(&mut self, packet: &Packet) -> io::Result<()> {
        let mut cursor = SliceCursor::new(self.out_buffer.as_mut_slice());
        cursor.write(packet).unwrap();
        let (slice, pos) = cursor.finish();
        self.stream.write_all(&slice[..pos])?;
        self.stream.flush()?;
        trace!(
            "> {} : {}",
            packet.tag(),
            crate::utils::HexString(&slice[..pos])
        );
        Ok(())
    }

    pub fn recv_packet(&mut self) -> Result<Packet, mpsc::RecvError> {
        self.packet_rx.recv()
    }

    pub fn try_recv_packet(&mut self) -> Result<Packet, mpsc::TryRecvError> {
        self.packet_rx.try_recv()
    }
}
