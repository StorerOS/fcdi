use fvm_ipld_blockstore::Block;
use fvm_sdk;
use cid::multihash::Code;
use anyhow::{anyhow};
use cid::Cid;

pub struct Blockstore;

impl fvm_ipld_blockstore::Blockstore for Blockstore {
    fn get(&self, cid: &Cid) -> anyhow::Result<Option<Vec<u8>>> {
        fvm_sdk::ipld::get(cid)
            .map(Some)
            .map_err(|e| anyhow!("get failed with {:?} on CID '{}'", e, cid))
    }

    fn put_keyed(&self, k: &cid::Cid, block: &[u8]) -> anyhow::Result<()> {
        let code = Code::try_from(k.hash().code()).map_err(|e|anyhow!(e.to_string()))?;
        let k2 = self.put(code, &Block::new(k.codec(), block))?;
        if k != &k2 {
            return Err(anyhow!("put block with cid {} but has cid {}", k, k2));
        }
        Ok(())
    }

    fn put<D>(&self, mh_code: Code, block: &Block<D>) -> anyhow::Result<cid::Cid> where Self: Sized, D: AsRef<[u8]> {
        const SIZE: u32 = 32;
        let k = fvm_sdk::ipld::put(mh_code.into(), SIZE, block.codec, block.data.as_ref())
            .map_err(|e|anyhow!("put failed with {:?}", e))?;
        Ok(k)
    }
}
