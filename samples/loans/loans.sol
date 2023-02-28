pragma solidity ^0.8.17;

import {CommonTypes} from "../filecoin-solidity/contracts/v0.8/types/CommonTypes.sol";
import {MinerTypes} from "../filecoin-solidity/contracts/v0.8/types/MinerTypes.sol";
import { MinerAPI } from "../filecoin-solidity/contracts/v0.8/MinerAPI.sol";
import { PrecompilesAPI } from "../filecoin-solidity/contracts/v0.8/PrecompilesAPI.sol";
import { SendAPI } from "../filecoin-solidity/contracts/v0.8/SendAPI.sol";

contract loans {  
    event LogUint(uint64 data);

    function getBeneficiary(uint64 minerId) public returns (uint64){
        CommonTypes.FilActorId _miner_id = CommonTypes.FilActorId.wrap(minerId);

        MinerTypes.GetBeneficiaryReturn memory beneficiary = MinerAPI.getBeneficiary(_miner_id);

        CommonTypes.FilAddress  memory filActor = beneficiary.active.beneficiary;
        uint64 actorId = PrecompilesAPI.resolveAddress(filActor);
        emit LogUint(actorId);
        return actorId;
    }

    function transfer(uint64 actorId, uint256 value) public {
        CommonTypes.FilActorId filActorid = CommonTypes.FilActorId.wrap(actorId);
        SendAPI.send(filActorid, value);
    }
}