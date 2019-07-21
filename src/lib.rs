#![no_std]


extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate rustc_hex as hex;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};
// use ethereum_types::H256;
use std::cell::RefCell;


// State key name "vendors" holding a vector of Vendor structs
static VENDORS: &str = "vendors";
static DATACOLLECTIONS: &str = "datacollections";
static ALGORITHMS: &str = "algorithms";
enum VendorType {
    DS,
    DC,
    AC,
    DU,
}
enum SDCStatus {
    Defined,
    BackTesting,
    Published,
    Subcribed,
    Cancelled,
    Terminated,
    // Invested?
}
impl Default for SDCStatus {
    fn default() -> Self { SDCStatus::Defined }
}

// Struct representing a Vendor
#[derive(Default, Serialize, Deserialize)]
pub struct Vendor {
    address: H256, // field containing 32 byte hash type for a vendor's address
    // company: String, // ?? what for
    vendorType: String,
    isOrganizer: bool,
}

pub struct CoopGroup {
    organizer: H256,
    members: RefCell<Vec<Vendor>>
}

pub impl CoopGroup {
    fn created(address: H256)->CoopGroup {
        CoopGroup {
            organizer: address,
            members: RefCell::new(Vec::new()) 
        }
    }
    fn add_m(&self, member: Vendor) {  
        self.members.borrow_mut().push(member);
    }
}


pub struct DataCollection {
    dcType: String,   //  location/activity
    format: String, //  json / csv / binary
    owner: H256,    // member in coop-group's address
}

pub struct AlgorithmInfo {
    dataType: String,   //  location/activity
    requirements: String,
    owner: H256,    // member in coop-group's address
}

// Struct representing a SDC
#[derive(Serialize, Deserialize)]
pub struct SDC {
    organizer: H256,    // also be a creator
    dataCollections: Vec<DataCollection>,   // Vector Data Collections
    algoInfos: Vec<AlgorithmInfo>,
    status: String,
    CreatedAt: DateTime<Utc>,
    UpdatedAt: DateTime<Utc>,
}

// Public-facing secret contract function declarations
#[pub_interface]
pub trait VendorContractInterface {   // vendor-contract
    // fn register(address: H256, vendorType: String);
    // fn form_coop_group(); // replaced by 2 funcs right below
    fn request_join_coop_group(address: H256, vendor_type: String, is_organizer: bool);
    fn approve_request(choice: String);

    fn define_data_info();
    fn invest();    // DS will choose to invest for an SDC of a coop group
    fn on_subcribe(); // DU	can	choose to subscribe	an SDC of a coop-group
    fn cancel_subscription(); // DU can decide to cancel her subscription of an SDC by marking her subscription status in the SDC as cancelled

}

pub struct VendorContract;

// Private functions accessible only by the secret contract
impl VendorContract {
    // Read secret contract state to obtain vector of Vendors (or new vector if uninitialized)
    fn get_vendors() -> Vec<Vendor> {
        match read_state!(VENDORS) {
            Some(vec) => vec,
            None => Vec::new(),
        }
    }
    //todo: 2 methods below should belong to sdc-contract
    fn get_data_collections() -> Vec<DataCollection> {
        match read_state!(DATACOLLECTIONS) {
            Some(vec) => vec,
            None => Vec::new(),
        }
    }
    fn get_algorithms() -> Vec<AlgorithmInfo> {
        match read_state!(ALGORITHMS) {
            Some(vec) => vec,
            None => Vec::new(),
        }
    }

    fn add_member(address: H256, vendorType: String, isOrganizer: bool) {
        // Read state to get vector of Vendors
        let mut vendors = Self::get_vendors();
        // Append a new Vendor struct to this vector
        vendors.push(Vendor {
            address,
            vendorType,
            isOrganizer
        });
        // Write the updated vector to contract's state
        write_state!(VENDORS => vendors);
    }
}

impl VendorContractInterface for VendorContract {
    // a vendor send a request to join coop-group - this's triggered an user's action on GUI 
    #[no_mangle]
    fn request_join_coop_group(address: H256, vendor_type: String, is_organizer: bool) {
        //  just store in mem first then once its approved by organizer (choice = 'yes'), calling add_member
        // if is_organizer {
        Self::add_member(address, vendor_type, is_organizer);
        // }
    }

    #[no_mangle]
    fn approve_request(choice: bool) {
        
    }
}
