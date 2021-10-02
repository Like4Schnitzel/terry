use crate::packets::PacketBody;
use crate::SliceCursor;

pub enum SlotLocation {
    Inventory(usize),
    Armor(usize),
    Dye(usize),
    MiscEquips(usize),
    MiscDyes(usize),
    PiggyBank(usize),
    Safe(usize),
    Trash,
    DefenderForge(usize),
    VoidVault(usize),
}

/// Player inventory slot.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerInventorySlot {
    pub player_id: u8,
    pub slot_id: i16,
    pub stack: i16,
    pub prefix: u8,
    pub item_netid: i16,
}

impl PlayerInventorySlot {
    /// The slot location depends on the value of `slot_id`:
    ///
    /// * 0 - 58 = Inventory
    /// * 59 - 78 = Armor
    /// * 79 - 88 = Dye
    /// * 89 - 93 MiscEquips
    /// * 94 - 98 = MiscDyes
    /// * 99 - 138 = Piggy bank
    /// * 139 - 178 = Safe
    /// * 179 = Trash
    /// * 180 - 219 = Defender's Forge
    /// * 220 - 259 = Void Vault
    pub fn slot_location(&self) -> SlotLocation {
        let index = self.slot_id as usize;
        match self.slot_id {
            0..=58 => SlotLocation::Inventory(index),
            59..=78 => SlotLocation::Armor(index - 59),
            79..=88 => SlotLocation::Dye(index - 79),
            89..=93 => SlotLocation::MiscEquips(index - 89),
            94..=98 => SlotLocation::MiscDyes(index - 94),
            99..=138 => SlotLocation::PiggyBank(index - 99),
            139..=178 => SlotLocation::Safe(index - 139),
            179 => SlotLocation::Trash,
            180..=219 => SlotLocation::DefenderForge(index - 180),
            220..=259 => SlotLocation::VoidVault(index - 220),
            n => panic!("slot index {} is out of bounds", n),
        }
    }
}

impl PacketBody for PlayerInventorySlot {
    const TAG: u8 = 5;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.slot_id);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.item_netid);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            slot_id: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            item_netid: cursor.read(),
        }
    }
}
