use crate::address::Address;
use crate::error::Error;
use alloc::string::{String};
use casper_contract::{
    contract_api::{runtime, storage, runtime::get_blocktime},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::bytesrepr::FromBytes;
use casper_types::CLTyped;

use casper_types::{bytesrepr::ToBytes, Key};
use casper_types::{system::CallStackElement, URef, U256};
use core::convert::TryInto;
use core::u64;

// Helper functions

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

pub fn get_key_from_address(addr: &Address) -> Key {
    let self_key = match *addr {
        Address::Account(acc) => Key::from(acc),
        Address::Contract(contract_package_hash) => Key::from(contract_package_hash),
    };
    self_key
}

pub fn get_self_key() -> Key {
    let self_addr = get_self_address().unwrap_or_revert();
    return get_key_from_address(&self_addr);
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}


pub fn get_self_address() -> Result<Address, Error> {
    get_last_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

fn get_last_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(0)
}

/// Gets the immediate call stack element of the current execution.
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

// pub(crate) fn get_verified_caller() -> Result<Key, Error> {
//     match *runtime::get_call_stack()
//         .iter()
//         .nth_back(1)
//         .unwrap_or_revert()
//     {
//         CallStackElement::Session {
//             account_hash: calling_account_hash,
//         } => {
//             Ok(Key::Account(calling_account_hash))
//         }
//         CallStackElement::StoredSession { contract_hash, .. }
//         | CallStackElement::StoredContract { contract_hash, .. } => {
//             Ok(contract_hash.into())
//         }
//     }
// }

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub fn get_immediate_caller_address() -> Result<Address, Error> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

pub fn get_immediate_caller_key() -> Key {
    let addr = get_immediate_caller_address().unwrap_or_revert();
    get_key_from_address(&addr)
}

// pub(crate) fn get_stored_value_with_user_errors<T: CLTyped + FromBytes>(
//     name: &str,
//     missing: Error,
//     invalid: Error,
// ) -> T {
//     let uref = get_uref(name);
//     read_with_user_errors(uref, missing, invalid)
// }
// pub(crate) fn read_with_user_errors<T: CLTyped + FromBytes>(
//     uref: URef,
//     missing: Error,
//     invalid: Error,
// ) -> T {
//     let key: Key = uref.into();
//     let (key_ptr, key_size, _bytes) = to_ptr(key);

//     let value_size = {
//         let mut value_size = MaybeUninit::uninit();
//         let ret = unsafe { ext_ffi::casper_read_value(key_ptr, key_size, value_size.as_mut_ptr()) };
//         match api_error::result_from(ret) {
//             Ok(_) => unsafe { value_size.assume_init() },
//             Err(ApiError::ValueNotFound) => runtime::revert(missing),
//             Err(e) => runtime::revert(e),
//         }
//     };

//     let value_bytes = read_host_buffer(value_size).unwrap_or_revert();

//     bytesrepr::deserialize(value_bytes).unwrap_or_revert_with(invalid)
// }


// pub(crate) fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
//     let bytes = t.into_bytes().unwrap_or_revert();
//     let ptr = bytes.as_ptr();
//     let size = bytes.len();
//     (ptr, size, bytes)
// }


// pub(crate) fn read_host_buffer(size: usize) -> Result<Vec<u8>, ApiError> {
//     let mut dest: Vec<u8> = if size == 0 {
//         Vec::new()
//     } else {
//         let bytes_non_null_ptr = contract_api::alloc_bytes(size);
//         unsafe { Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), size, size) }
//     };
//     read_host_buffer_into(&mut dest)?;
//     Ok(dest)
// }
// pub(crate) fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
//     let mut bytes_written = MaybeUninit::uninit();
//     let ret = unsafe {
//         ext_ffi::casper_read_host_buffer(dest.as_mut_ptr(), dest.len(), bytes_written.as_mut_ptr())
//     };
//     // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
//     // caller ignores the return value, execution of the contract becomes unstable and ultimately
//     // leads to `Unreachable` error.
//     api_error::result_from(ret)?;
//     Ok(unsafe { bytes_written.assume_init() })
// }


#[no_mangle]
pub fn dictionary_write(dictionary_uref: URef, address: Address, amount: U256) {
    let dictionary_item_key = make_dictionary_item_key(address);
    storage::dictionary_put(dictionary_uref, &dictionary_item_key, amount);
}

/// Creates a dictionary item key for a dictionary item.
#[no_mangle]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64::encode(&preimage)
}

/// Creates a dictionary item key for a dictionary item.
#[no_mangle]
pub fn make_dictionary_item_key_for_key(contract_hash: Key) -> String {
    let preimage = contract_hash.into_hash().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    hex::encode(&preimage)
}

#[no_mangle]
pub fn dictionary_read(dictionary_uref: URef, address: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(address);

    storage::dictionary_get(dictionary_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

pub fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name).unwrap_or_revert();
    key.into_uref().unwrap_or_revert()
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    dictionary_name: &str,
    key: &str,
) -> Option<T> {
    let seed_uref = get_uref(dictionary_name);

    match storage::dictionary_get::<T>(seed_uref, key) {
        Ok(maybe_value) => maybe_value,
        Err(_) => None,
    }
}

pub fn write_dictionary_value_from_key<T: CLTyped + FromBytes + ToBytes>(
    dictionary_name: &str,
    key: &str,
    value: T,
) {
    let seed_uref = get_uref(dictionary_name);

    match storage::dictionary_get::<T>(seed_uref, key) {
        Ok(None | Some(_)) => storage::dictionary_put(seed_uref, key, value),
        Err(error) => runtime::revert(error),
    }
}

/// Helper function that returns the current block timestamp within the range of [`u64`], i.e. `[0, 2**64 - 1]`.
pub fn current_block_timestamp() -> u64 {
    u64::from(get_blocktime()).checked_rem(u64::MAX).unwrap()
}

pub fn require(v: bool, e: Error) {
    if !v {
        runtime::revert(e);
    }
}