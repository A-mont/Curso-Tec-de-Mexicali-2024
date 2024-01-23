
#![no_std]
use gstd::{msg, collections::HashMap , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. Create the main state as a static variable.
static mut STATE:Option<CustomStruct> = None;

// 1.1 Create the init state.
static mut INIT:Option<InitStruct> = None;



// 2. Create the mutability function for your state.
fn state_mut() -> &'static mut CustomStruct {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }

}

#[warn(dead_code)]
fn init_state_mut() -> &'static mut InitStruct {

    let initstruct = unsafe { INIT.as_mut()};

    unsafe { initstruct.unwrap_unchecked() }

}

// Create a public State
#[derive(Clone, Default)]
pub struct CustomStruct {
    pub firstfield: String,
    pub secondfield: u128,
    pub thirdfield: HashMap<ActorId, u128>,
}

// Create a implementation on State
impl CustomStruct {
    #[allow(dead_code)]
    async fn firstmethod(&mut self) {}
    #[allow(dead_code)]
    async fn secondmethod(&mut self) { }
    #[allow(dead_code)]
    async fn thirdmethod(&mut self) {}
}


// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init () {

}


// 4.Create the Handle() function of your contract.
#[no_mangle]
extern "C" fn handle(){

        // We load the input message
        let action: Action = msg::load().expect("Could not load Action");

        // We receive an action from the user and update the state. Example:
        match action {
            Action::Hola => {

                msg::reply(String::from("Hello World!!"), 0).expect("Error in sending a reply message");

            }
      
        };
    }

        

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<CustomStruct> for IoCustomStruct {

    // Conversion method
    fn from(value: CustomStruct) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let CustomStruct {
            firstfield,
            secondfield,
            thirdfield,
        } = value;

        // Perform some transformation on thirdfield, cloning its elements
        let thirdfield = thirdfield.iter().map(|(k, v)| (*k, v.clone())).collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            firstfield,
            secondfield,
            thirdfield,
        }
    }
}


