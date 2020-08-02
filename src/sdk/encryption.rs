use memlib::memory::*;
use log::*;
use std::num::Wrapping;

use super::offsets;

pub fn get_client_info_address(game_base_address: Address) -> Result<Address> {
    // Get the encrypted base address
    let encrypted_address: Address = read_memory(game_base_address + offsets::client_info::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        return Err("Could not find encrypted base address for client_info".into());
    }
    trace!("Found encrypted client_info address: 0x{:X}", encrypted_address);

    // Get last_key
    let last_key = get_last_key(game_base_address, offsets::client_info::REVERSED_ADDRESS, offsets::client_info::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting the client_info base address")?;

    // Get not_peb
    let not_peb = get_not_peb();
    trace!("not_peb: 0x{:X}", not_peb);

    let mut encrypted_address = Wrapping(encrypted_address);
    let last_key          = Wrapping(last_key);
    let not_peb           = Wrapping(not_peb);
    let game_base_address = Wrapping(game_base_address);

    encrypted_address += Wrapping(1);
    let mut v1 = not_peb + !(game_base_address + Wrapping(0xE165));
    v1 += encrypted_address;
    v1 ^= v1 >> 0x10;
    v1 ^= v1 >> 0x20;
    encrypted_address = last_key * v1;
    encrypted_address ^= Wrapping(0xE088DE06F8290B73);
    encrypted_address *= Wrapping(0xB042A27BAA05DF1F);
    encrypted_address ^= encrypted_address >> 0xC;
    encrypted_address ^= encrypted_address >> 0x18;
    encrypted_address ^= encrypted_address >> 0x30;

    trace!("Found decrypted client_info address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

pub fn get_client_base_address(game_base_address: Address, client_info_address: Address) -> Result<Address> {
    let encrypted_address = read_memory(client_info_address + offsets::client_base::BASE_OFFSET);
    if encrypted_address == 0 {
        return Err("Could not find the encrypted client_info_base address".into());
    }
    trace!("Found encrypted client_info_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key_byteswap(game_base_address, offsets::client_base::BASE_REVERSED_ADDR, offsets::client_base::BASE_DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting client_info_base")?;

    let not_peb = get_not_peb();

    let mut encrypted_address   = Wrapping(encrypted_address);
    let last_key                = Wrapping(last_key);
    let not_peb                 = Wrapping(not_peb);
    let game_base_address       = Wrapping(game_base_address);

    // Actual decryption

    encrypted_address ^= encrypted_address >> 0x3;
    encrypted_address ^= encrypted_address >> 0x6;
    encrypted_address ^= encrypted_address >> 0xC;
    encrypted_address ^= encrypted_address >> 0x18;
    encrypted_address ^= encrypted_address >> 0x30;
    encrypted_address += (game_base_address + Wrapping(0x13C53E55)) * not_peb;
    encrypted_address *= Wrapping(0x1792C64AA7FF1687);
    encrypted_address += Wrapping(0x0BC92A7E607586AA);
    encrypted_address ^= Wrapping(0x960331A328BC40CF);
    encrypted_address *= last_key;
    encrypted_address += (game_base_address + Wrapping(0x1782)) - not_peb;
    encrypted_address ^= encrypted_address >> 0x6;
    encrypted_address ^= encrypted_address >> 0xC;
    encrypted_address ^= encrypted_address >> 0x18;
    encrypted_address ^= encrypted_address >> 0x30;

    trace!("Found decrypted client_info_base address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

pub fn get_bone_base_address(game_base_address: Address) -> Result<Address> {
    let encrypted_address = read_memory(game_base_address + offsets::bones::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        return Err("Could not find the encrypted bone_base address".into());
    }
    trace!("Found encrypted bone_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key_byteswap(game_base_address, offsets::bones::REVERSED_ADDRESS, offsets::bones::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting base_address")?;

    let not_peb = get_not_peb();

    let mut encrypted_address   = Wrapping(encrypted_address);
    let last_key                = Wrapping(last_key);
    let not_peb                 = Wrapping(not_peb);
    let game_base_address       = Wrapping(game_base_address);

    encrypted_address ^= encrypted_address >> 0x3;
    encrypted_address ^= encrypted_address >> 0x6;
    encrypted_address ^= encrypted_address >> 0xC;
    encrypted_address ^= encrypted_address >> 0x18;
    encrypted_address ^= encrypted_address >> 0x30;
    encrypted_address += (game_base_address + Wrapping(0x13C53E55)) * not_peb;
    encrypted_address *= Wrapping(0x1792C64AA7FF1687);
    encrypted_address += Wrapping(0x0BC92A7E607586AA);
    encrypted_address ^= Wrapping(0x960331A328BC40CF);
    encrypted_address *= last_key;
    encrypted_address += (game_base_address + Wrapping(0x1782)) - not_peb;
    encrypted_address ^= encrypted_address >> 0x6;
    encrypted_address ^= encrypted_address >> 0xC;
    encrypted_address ^= encrypted_address >> 0x18;
    encrypted_address ^= encrypted_address >> 0x30;

    trace!("Found decrypted bone_base address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

fn get_not_peb() -> u64 {
    !get_process_info().peb_base_address
}

fn get_last_key(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reversed_address: Address = read_memory(game_base_address + reversed_address_offset);
    let last_key = read_memory(!reversed_address + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}

fn get_last_key_byteswap(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reversed_address: Address = read_memory(game_base_address + reversed_address_offset);
    let last_key = read_memory(u64::from_be(reversed_address) + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}
