mod blockstore;

use cid::multihash::Code;
use fvm_sdk as sdk;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;
use crate::blockstore::Blockstore;


macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

#[no_mangle]
pub fn invoke(_: u32) -> u32 {
    let ret = match sdk::message::method_number() {
        1 => constructor(),
        2 => say_hello(),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method")
    };

    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        }
    }
}

pub fn constructor() -> Option<RawBytes> {
    const INIT_ACTOR_ADDR: ActorID = 1;

    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let state = State::default();
    state.save();
    None
}

pub fn say_hello() -> Option<RawBytes> {
    let mut state = State::load();
    state.count += 1;
    state.save();

    let caller = sdk::message::caller();

    let ret = to_vec(format!("{}: Hello world #{}!", caller, &state.count).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64
}

impl State {
    pub fn load() -> Self {
        let root = match sdk::sself::root() {
            Ok(root) =>root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) ->Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err)
        };

        let cid = match sdk::ipld::put(
            Code::Blake2b256.into(),
            32,
            DAG_CBOR,
            serialized.as_slice()
        ) {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err)
        };

        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:?}", err)

        }
        cid
    }
}