//! This module contains type definitions for **all** the possible packets
//! in a Terraria vanilla client-server communication.
//!
//! All of them own the objects (for example, you need to allocate a `String`
//! to send chat messages) simply because it makes it a lot easier to work
//! with. An alternative implementation would of course be possible (read
//! the fields on demand), but it would also be a lot less ergonomic.
//!
//! Unless absolutely needed, most of the packets don't have documentation,
//! since the field names should suffice.
mod add_npc_buff;
mod add_player_buff;
mod angler_quest;
mod angler_quests;
mod catch_npc;
mod client_synced_inventory;
mod client_uuid;
mod combat_text;
mod complete_angler_quest;
mod complete_connection_and_spawn;
mod connect;
mod connection_complete;
mod create_combat_text;
mod create_temporary_animation;
mod crystal_invasion_start;
mod crystal_invasion_wait;
mod crystal_invasion_wipe;
mod dead_player;
mod destroy_projectile;
mod disconnect;
mod doll_sync;
mod door_toggle;
mod emoji;
mod fish_out_npc;
mod gem_lock_toggle;
mod get_chest_name;
mod grow_fx;
mod hat_rack_sync;
mod heal_effect;
mod heal_other_player;
mod hit_switch;
mod interact_tile_entity;
mod invasion_progress;
mod kill_portal;
mod land_golf_ball;
mod load_net_module;
mod mana_effect;
mod mass_consume_wire;
mod mass_wire;
mod modify_tile;
mod moon_lord_countdown;
mod nebula_level_up;
mod npc_shop_item;
mod npc_strike;
mod npc_teleport_portal;
mod npc_update;
mod open_chest;
mod packet15;
mod packet166;
mod packet169;
mod packet177;
mod packet180;
mod packet187;
mod packet220;
mod packet230;
mod packet243;
mod packet94;
mod paint_tile;
mod paint_wall;
mod place_chest;
mod place_food;
mod place_item_frame;
mod place_object;
mod place_tile_entity;
mod place_weapon_rack;
mod placeholder;
mod play_legacy_sound;
mod play_music_item;
mod player_active;
mod player_death;
mod player_dodge;
mod player_hp;
mod player_hurt;
mod player_info;
mod player_inventory_slot;
mod player_item_animation;
mod player_mana;
mod player_npc_killed;
mod player_npc_teleport;
mod player_team;
mod player_teleport_portal;
mod player_zone;
mod poof_of_smoke;
mod projectile_update;
mod quick_stash;
mod release_npc;
mod remove_item_owner;
mod remove_revenge;
mod request_essential_tiles;
mod request_npc_debuff;
mod request_password;
mod request_sign;
mod request_world_data;
mod section_tile_frame;
mod send_password;
mod send_section;
mod send_tile_square;
mod set_active_npc;
mod set_as_host;
mod set_event;
mod set_liquid;
mod set_minion_target;
mod set_npc_home;
mod set_npc_kill_count;
mod set_player_stealth;
mod set_user_slot;
mod smart_text_message;
mod social_handshake;
mod spawn_boss_invasion;
mod spawn_player;
mod special_npc_effect;
mod status;
mod strike_npc;
mod sync_active_chest;
mod sync_emote_bubble;
mod sync_extra_value;
mod sync_monster_type;
mod sync_player_chest_index;
mod sync_revenge;
mod sync_tile_picking;
mod tamper_with_npc;
mod teleportation_potion;
mod time;
mod toggle_birthday_party;
mod toggle_pvp;
mod travelling_merchant_inventory;
mod tweak_item;
mod unlock;
mod update_chest_item;
mod update_good_evil;
mod update_item_drop;
mod update_item_drop2;
mod update_item_owner;
mod update_minion_target;
mod update_npc_buff;
mod update_npc_name;
mod update_player;
mod update_player_buff;
mod update_player_luck;
mod update_shield_strengths;
mod update_sign;
mod update_tile_entity;
mod wired_cannon_shot;
mod world_info;

pub use add_npc_buff::AddNpcBuff;
pub use add_player_buff::AddPlayerBuff;
pub use angler_quest::AnglerQuest;
pub use angler_quests::AnglerQuests;
pub use catch_npc::CatchNpc;
pub use client_synced_inventory::ClientSyncedInventory;
pub use client_uuid::ClientUuid;
pub use combat_text::CombatText;
pub use complete_angler_quest::CompleteAnglerQuest;
pub use complete_connection_and_spawn::CompleteConnectionAndSpawn;
pub use connect::Connect;
pub use connection_complete::ConnectionComplete;
pub use create_combat_text::CreateCombatText;
pub use create_temporary_animation::CreateTemporaryAnimation;
pub use crystal_invasion_start::CrystalInvasionStart;
pub use crystal_invasion_wait::CrystalInvasionWait;
pub use crystal_invasion_wipe::CrystalInvasionWipe;
pub use dead_player::DeadPlayer;
pub use destroy_projectile::DestroyProjectile;
pub use disconnect::Disconnect;
pub use doll_sync::DollSync;
pub use door_toggle::{DoorAction, DoorToggle};
pub use emoji::Emoji;
pub use fish_out_npc::FishOutNpc;
pub use gem_lock_toggle::GemLockToggle;
pub use get_chest_name::GetChestName;
pub use grow_fx::GrowFx;
pub use hat_rack_sync::HatRackSync;
pub use heal_effect::HealEffect;
pub use heal_other_player::HealOtherPlayer;
pub use hit_switch::HitSwitch;
pub use interact_tile_entity::InteractTileEntity;
pub use invasion_progress::InvasionProgress;
pub use kill_portal::KillPortal;
pub use land_golf_ball::LandGolfBall;
pub use load_net_module::LoadNetModule;
pub use mana_effect::ManaEffect;
pub use mass_consume_wire::MassConsumeWire;
pub use mass_wire::MassWire;
pub use modify_tile::{ModifyTile, TileAction};
pub use moon_lord_countdown::MoonLordCountdown;
pub use nebula_level_up::NebulaLevelUp;
pub use npc_shop_item::NpcShopItem;
pub use npc_strike::NpcStrike;
pub use npc_teleport_portal::NpcTeleportPortal;
pub use npc_update::NpcUpdate;
pub use open_chest::OpenChest;
pub use packet15::Packet15;
pub use packet166::Packet166;
pub use packet169::Packet169;
pub use packet177::Packet177;
pub use packet180::Packet180;
pub use packet187::Packet187;
pub use packet220::Packet220;
pub use packet230::Packet230;
pub use packet243::Packet243;
pub use packet94::Packet94;
pub use paint_tile::PaintTile;
pub use paint_wall::PaintWall;
pub use place_chest::PlaceChest;
pub use place_food::PlaceFood;
pub use place_item_frame::PlaceItemFrame;
pub use place_object::PlaceObject;
pub use place_tile_entity::PlaceTileEntity;
pub use place_weapon_rack::PlaceWeaponRack;
pub use placeholder::Placeholder;
pub use play_legacy_sound::PlayLegacySound;
pub use play_music_item::PlayMusicItem;
pub use player_active::PlayerActive;
pub use player_death::PlayerDeath;
pub use player_dodge::PlayerDodge;
pub use player_hp::PlayerHp;
pub use player_hurt::PlayerHurt;
pub use player_info::{Difficulty, PlayerInfo, Torches};
pub use player_inventory_slot::{PlayerInventorySlot, SlotLocation};
pub use player_item_animation::PlayerItemAnimation;
pub use player_mana::PlayerMana;
pub use player_npc_killed::PlayerNpcKilled;
pub use player_npc_teleport::PlayerNpcTeleport;
pub use player_team::PlayerTeam;
pub use player_teleport_portal::PlayerTeleportPortal;
pub use player_zone::PlayerZone;
pub use poof_of_smoke::PoofOfSmoke;
pub use projectile_update::ProjectileUpdate;
pub use quick_stash::QuickStash;
pub use release_npc::ReleaseNpc;
pub use remove_item_owner::RemoveItemOwner;
pub use remove_revenge::RemoveRevenge;
pub use request_essential_tiles::RequestEssentialTiles;
pub use request_npc_debuff::RequestNpcDebuff;
pub use request_password::RequestPassword;
pub use request_sign::RequestSign;
pub use request_world_data::RequestWorldData;
pub use section_tile_frame::SectionTileFrame;
pub use send_password::SendPassword;
pub use send_section::SendSection;
pub use send_tile_square::SendTileSquare;
pub use set_active_npc::SetActiveNpc;
pub use set_as_host::SetAsHost;
pub use set_event::SetEvent;
pub use set_liquid::SetLiquid;
pub use set_minion_target::SetMinionTarget;
pub use set_npc_home::SetNpcHome;
pub use set_npc_kill_count::SetNpcKillCount;
pub use set_player_stealth::SetPlayerStealth;
pub use set_user_slot::SetUserSlot;
pub use smart_text_message::SmartTextMessage;
pub use social_handshake::SocialHandshake;
pub use spawn_boss_invasion::SpawnBossInvasion;
pub use spawn_player::{SpawnContext, SpawnPlayer};
pub use special_npc_effect::SpecialNpcEffect;
pub use status::Status;
pub use strike_npc::StrikeNpc;
pub use sync_active_chest::SyncActiveChest;
pub use sync_emote_bubble::SyncEmoteBubble;
pub use sync_extra_value::SyncExtraValue;
pub use sync_monster_type::SyncMonsterType;
pub use sync_player_chest_index::SyncPlayerChestIndex;
pub use sync_revenge::SyncRevenge;
pub use sync_tile_picking::SyncTilePicking;
pub use tamper_with_npc::TamperWithNpc;
pub use teleportation_potion::TeleportationPotion;
pub use time::Time;
pub use toggle_birthday_party::ToggleBirthdayParty;
pub use toggle_pvp::TogglePvp;
pub use travelling_merchant_inventory::TravellingMerchantInventory;
pub use tweak_item::TweakItem;
pub use unlock::Unlock;
pub use update_chest_item::UpdateChestItem;
pub use update_good_evil::UpdateGoodEvil;
pub use update_item_drop::UpdateItemDrop;
pub use update_item_drop2::UpdateItemDrop2;
pub use update_item_owner::UpdateItemOwner;
pub use update_minion_target::UpdateMinionTarget;
pub use update_npc_buff::UpdateNpcBuff;
pub use update_npc_name::UpdateNpcName;
pub use update_player::UpdatePlayer;
pub use update_player_buff::UpdatePlayerBuff;
pub use update_player_luck::UpdatePlayerLuck;
pub use update_shield_strengths::UpdateShieldStrengths;
pub use update_sign::UpdateSign;
pub use update_tile_entity::UpdateTileEntity;
pub use wired_cannon_shot::WiredCannonShot;
pub use world_info::WorldInfo;
