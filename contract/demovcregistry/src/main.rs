#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;
use std::{fmt::format, io::{Read, SeekFrom}, str::Bytes, thread::AccessError, vec};

use contract::{contract_api::{runtime::{self, revert}, storage, system::get_auction}, unwrap_or_revert::UnwrapOrRevert};

use types::{ApiError, BlockTime, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, U256, U512, URef, 
            account::{AccountHash}, bytesrepr::{FromBytes, ToBytes, Error}, 
            contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys}, 
            runtime_args};


#[no_mangle]
pub extern "C" fn issueDemoVC(){
    // Parameter::new("merkleRoot",CLType::ByteArray(32)),
    // Parameter::new("ipfsHash", CLType::ByteArray(32)),
    // Parameter::new("schemaHash",CLType::ByteArray(32)),
    // Parameter::new("holder", AccountHash::cl_type()),
    // Parameter::new("revocationFlag",CLType::Bool),
    
    let merkleRoot : [u8;32] = runtime::get_named_arg("merkleRoot");
    let ipfsHash : [u8;32] = runtime::get_named_arg("ipfsHash");
    let schemaHash : [u8;32] = runtime::get_named_arg("schemaHash");
    let holder : AccountHash = runtime::get_named_arg("holder");
    let revocationFlag : bool = runtime::get_named_arg("revocationFlag");

    let issuer : AccountHash = runtime::get_caller();
    let vcLengthKey = &vc_length_key(&issuer);
    let index : u64 = get_key(vcLengthKey);
    set_key(&vc_key(&issuer,index,"merkleRoot"), merkleRoot);
    set_key(&vc_key(&issuer,index,"ipfsHash"), ipfsHash);
    set_key(&vc_key(&issuer,index,"schemaHash"), schemaHash);
    set_key(&vc_key(&issuer,index,"holder"), holder);
    set_key(&vc_key(&issuer,index,"revocationFlag"), revocationFlag);

    set_key(&vclink_key(&holder, index), vc_key(&issuer, index, ""));
    
    let next_index : u64 = index + 1;
    set_key(vcLengthKey, next_index);
    set_key(&vclink_length_key(&holder), next_index);

}

#[no_mangle]
pub extern "C" fn changeVCRevocationFlag(){
    // Parameter::new("index", CLType::U64),
    // Parameter::new("revocationFlag",CLType::Bool),
    
    let index : u64 = runtime::get_named_arg("index");
    let revocationFlag : bool = runtime::get_named_arg("revocationFlag");

    let issuer : AccountHash = runtime::get_caller();
    let vc_length : u64 = get_key(&vc_length_key(&issuer));
    if index >= vc_length {
        runtime::revert(ApiError::InvalidArgument);
    }
   
    let vcRevocationFlagKey = &vc_key(&issuer, index,"revocationFlag");
    set_key(vcRevocationFlagKey, revocationFlag);
}

#[no_mangle]
pub extern "C" fn sendVPRequest(){
    // Parameter::new("ipfsHash", CLType::ByteArray(32)),
    // Parameter::new("holder",AccountHash::cl_type()),
    
    let ipfsHash : [u8;32] = runtime::get_named_arg("ipfsHash");
    let holder : AccountHash = runtime::get_named_arg("holder");

    let ipfsHashResponce : [u8;32] = [0u8;32];
   
    let verifier : AccountHash = runtime::get_caller();
    let vprequestLengthKey : &String = &vprequest_length_key(&verifier);
    let index : u64 = get_key(vprequestLengthKey);
    set_key(&vprequest_key(&verifier, index, "ipfsHash"), ipfsHash);
    set_key(&vprequest_key(&verifier, index, "ipfsHashResponce"), ipfsHashResponce);
    set_key(&vprequest_key(&verifier, index, "holder"), holder);
    set_key(&vprequest_key(&verifier, index, "status"), 0u8);   // set pending status
    
    set_key(&vprequestlink_key(&holder, index), vprequest_key(&verifier, index, ""));

    let next_index : u64 = index + 1;
    set_key(vprequestLengthKey, next_index);
    set_key( &vprequestlink_length_key(&holder), next_index);

}

#[no_mangle]
pub extern "C" fn changeVPRequestStatus(){
    // Parameter::new("verifier", AccountHash::cl_type()),
    // Parameter::new("index", CLType::U64),
    // Parameter::new("newStatus",CLType::U8),
    // Parameter::new("ipfsHashResponce", CLType::ByteArray(32)),

    /* statuses:
        0 => pending
        1 => approved
        2 => rejected
        3 => canceled
    */
    let verifier : AccountHash = runtime::get_named_arg("verifier");
    let index : u64 = runtime::get_named_arg("index");
    let newStatus : u8 = runtime::get_named_arg("newStatus");
    let ipfsHashResponce : [u8;32] = runtime::get_named_arg("ipfsHashResponce");

    let vprequest_length : u64 = get_key(&vprequest_length_key(&verifier)); 
    if index >= vprequest_length || newStatus == 0u8 {
        runtime::revert(ApiError::InvalidArgument);
    }

    let vpRequestStatusKey : &String = &vprequest_key(&verifier, index, "status");
    let currentStatus : u8 = get_key(vpRequestStatusKey);
    if currentStatus != 0u8 {
        runtime::revert(ApiError::ContractHeader(currentStatus));
    }
    
    let msgSender : AccountHash = runtime::get_caller();
    if msgSender == verifier && newStatus == 3 {
        set_key(vpRequestStatusKey, 3);
        return;
    }
    let holder : AccountHash = get_key(&vprequest_key(&verifier, index, "holder"));
    if msgSender == holder && newStatus < 3u8 {
        set_key(vpRequestStatusKey, newStatus);
        set_key(&vprequest_key(&verifier, index, "ipfsHashResponce"), ipfsHashResponce);
    } else {
        runtime::revert(ApiError::PermissionDenied);
    }
}


#[no_mangle]
pub extern "C" fn call() {
    
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(endpoint(
        "issueDemoVC",
        vec![
            Parameter::new("merkleRoot",CLType::ByteArray(32)),
            Parameter::new("ipfsHash", CLType::ByteArray(32)),
            Parameter::new("schemaHash",CLType::ByteArray(32)),
            Parameter::new("holder", AccountHash::cl_type()),
            Parameter::new("revocationFlag",CLType::Bool),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "changeVCRevocationFlag",
        vec![
            Parameter::new("index", CLType::U64),
            Parameter::new("revocationFlag",CLType::Bool),
        ],
        CLType::Unit
    ));
    entry_points.add_entry_point(endpoint(
        "sendVPRequest",
        vec![
            Parameter::new("ipfsHash", CLType::ByteArray(32)),
            Parameter::new("holder",AccountHash::cl_type()),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(endpoint(
        "changeVPRequestStatus",
        vec![
            Parameter::new("verifier", AccountHash::cl_type()),
            Parameter::new("index", CLType::U64),
            Parameter::new("newStatus",CLType::U8),
            Parameter::new("ipfsHashResponce", CLType::ByteArray(32)),
        ],
        CLType::Unit
    ));
    
    // let mut named_keys = NamedKeys::new();
    // named_keys.insert("registryOwner".to_string(), storage::new_uref(msgSender).into());

    let contract_name: &str = "CasperDemoVCRegistry";
    let contract_hash_name: &str = &format!("{}_{}",contract_name,"hash");

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key(contract_name, contract_hash.into());
    runtime::put_key(contract_hash_name, storage::new_uref(contract_hash).into());

}

//==============================================

//******************** VC ******************** */
fn vc_length_key(issuer: &AccountHash) -> String {
    format!("VC_length_{}",issuer)
}

fn vc_key(issuer: &AccountHash, index: u64, attribute: &str) -> String {
    format!("VC_{}_{}_{}", issuer, index, attribute)
}

fn vclink_length_key(holder: &AccountHash) -> String {
    format!("VCLink_length_{}", holder)
}

fn vclink_key(holder: &AccountHash, index: u64) -> String {
    format!("VCLink_{}_{}", holder, index)
}

//******************** VP ******************** */

fn vprequest_length_key(verifier: &AccountHash) -> String {
    format!("VPRequest_length_{}", verifier)
}

fn vprequest_key(verifier: &AccountHash,index: u64, attribute: &str) -> String {
    format!("VPRequest_{}_{}_{}", verifier, index, attribute)
}

fn vprequestlink_length_key(holder: &AccountHash) -> String {
    format!("VPRequestLink_length_{}", holder)
}

fn vprequestlink_key(holder: &AccountHash,index: u64) -> String {
    format!("VPRequestLink_{}_{}", holder, index)
}

//******************************************** */

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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

pub(crate) fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}