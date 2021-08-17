#![cfg_attr(not(feature = "std"), no_std)]

use core::str;
use core::str::FromStr;
/// CRM - Module to setup the contracts for rights management
use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

// structure to keep the voting progresses/results of the change proposals
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Voting {
    changeid: u32,
    crmid: u32,
    quorum: u32,
    nrvotesyes: u32,
    nrvotesno: u32,
    percvotesyes: u32,
    percvotesno: u32,
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Module Configuration
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The runtime storage items

decl_storage! {
    trait Store for Module<T: Config> as PolkaMusic {
        // the Contract main data in json format, the key is the uniqueid received
        CrmData get(fn get_crmdata): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // the Contract Master data in json format, the key is the uniqueid received
        CrmMasterData get(fn get_master): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // the Contract composition data in json format, the key is the uniqueid received
        CrmCompositionData get(fn get_composition): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // the Contract, Other Contracts data in json format, the key is the uniqueid received
        CrmOtherContractsData get(fn get_othercontracts): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Change proposal queue for Crm Data
        CrmDataChangeProposal get(fn get_crmdata_change_proposal): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Voting counters for the change proposals
        CrmDataChangeVotingResult get(fn get_crmdata_change_voting_result): map hasher(blake2_128_concat) u32  => Option<Voting>;
        // Votes casted for the contract main data change proposals
        CrmDataChangeVoteCasted get(fn get_crmdata_change_vote_casted): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<bool>;
        // Change proposal queue for Crm Master Data
        CrmMasterDataChangeProposal get(fn get_crm_masterdata_change_proposal): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Voting counters for the change proposals
        CrmMasterDataChangeVotingResult get(fn get_crm_masterdata_change_voting_result): map hasher(blake2_128_concat) u32  => Option<Voting>;
        // Votes casted for the change proposals
        CrmMasterDataChangeVoteCasted get(fn get_crm_masterdata_change_vote_casted): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<bool>;
        // Change proposal queue for Crm composition Data
        CrmCompositionDataChangeProposal get(fn get_crm_compositiondata_change_proposal): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Voting counters for the change proposals of composition data
        CrmCompositionDataChangeVotingResult get(fn get_crm_compositiondata_change_voting_result): map hasher(blake2_128_concat) u32  => Option<Voting>;
        // Votes casted for the change proposals of composition data
        CrmCompositionDataChangeVoteCasted get(fn get_crm_compositiondata_change_vote_casted): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<bool>;
        // Change proposal queue for Crm Other Contracts Data
        CrmOtherContractsDataChangeProposal get(fn get_crm_othercontractsdata_change_proposal): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Voting counters for the change proposals of Other Contracts data
        CrmOtherContractsDataChangeVotingResult get(fn get_crm_othercontractsdata_change_voting_result): map hasher(blake2_128_concat) u32  => Option<Voting>;
        // Votes casted for the change proposals of Other Contracts data
        CrmOtherContractsDataChangeVoteCasted get(fn get_crm_othercontractsdata_change_vote_casted): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<bool>;
    }
}

// Events used to inform users when important changes are made.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        CrmAdded(AccountId, u32),                      // New contract has been added
        CrmDataNewChangeProposal(AccountId, u32, u32), // A proposal change has been submitted
        CrmDataChangeVote(AccountId, u32, u32), // A vote for a crm data change proposal has been received
        CrmDataChanged(AccountId, u32),         // Crm data has been changed
        CrmMasterChanged(AccountId, u32),       // Crm master data has been changed
        CrmCompositionChanged(AccountId, u32),  // Crm composition data has been changed
        CrmOtherContractsChanged(AccountId, Vec<u8>), // Crm other contracts data has been changed
        CrmMasterDataNewChangeProposal(AccountId, u32, u32), // A proposal change for master data has been submitted
        CrmMasterDataChangeVote(AccountId, u32, u32), // A vote for a crm master data change proposal has been received
        CrmMasterDataChanged(AccountId, u32),         // Crm master data has been changed
        CrmCompositionDataNewChangeProposal(AccountId, u32, u32), // A proposal change for composition data has been submitted
        CrmCompositionDataChangeVote(AccountId, u32, u32), // A vote for a crm composition data change proposal has been received
        CrmCompositionDataChanged(AccountId, u32),         // Crm composition data has been changed
        CrmOtherContractsDataNewChangeProposal(AccountId, u32, u32), // A proposal change for Other Contracts data has been submitted
        CrmOtherContractsDataChangeVote(AccountId, u32, u32), // A vote for a crm Other Contracts data change proposal has been received
        CrmOtherContractsDataChanged(AccountId, u32), // Crm Other Contracts data has been changed
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// Missing value
        NoneValue,
        /// CrmData is too short to be valid
        CrmDataTooShort,
        /// CrmData is too long to be valid
        CrmDataTooLong,
        /// Master Data is too short to be valid
        MasterTooShort,
        /// Master data is too long to be valid
        MasterTooLong,
        /// Composition Data is too short to be valid
        CompositionTooShort,
        /// Composition data is too long to be valid
        CompositionTooLong,
        /// Other Contracts data is too long to be valid
        OtherContractsTooLong,
        /// Value is not valid
        InvalidValue,
        /// Invalid Json Structure
        InvalidJson,
        /// Invalid Json for Crm Data
        InvalidJsonCrmData,
        /// Invalid Json for Crm Master
        InvalidJsonCrmMaster,
        /// Invalid Json for Crm Composition
        InvalidJsonCrmComposition,
        /// Invalid Json for Crm Other Contracts
        InvalidJsonCrmOtherContracts,
        /// Duplicated Crm Id
        DuplicatedCrmId,
        /// Invalid Ipfs Hash
        InvalidIpfsHash,
        // Invalid Ipfs Hash Private
        InvalidIpfsHashPrivate,
        /// Invalid Global Quorum (must be > 0)
        InvalidGlobalQuorum,
        /// Invalid Master Shares
        InvalidMasterShare,
        /// Invalid Master Quorum
        InvalidMasterQuorum,
        /// Invalid Composition Shares
        InvalidCompositionShare,
        /// Invalid Composition Quorum
        InvalidCompositionQuorum,
        /// Invalid Other Contracts Share (can be 0..100)
        InvalidOtherContractsShare,
        /// Invalid Other Contracts Quorum (can be 0..100)
        InvalidOtherContractsQuorum,
        /// Invalid Crowd Funding Share (can be 0..100)
        InvalidCrowdFundingshares,
        /// Invalid Total Share, must be = 100
        InvalidTotalShares,
        /// Invalid ContractId
        InvalidContractId,
        /// Invalid Contract Id during Voting
        InvalidContractIdVoting,
        /// Invalid Contract Id during Voting after numeric conversion
        InvalidContractIdVotingNumeric,
        /// Missing Contract data to change
        MissingContractData,
        /// Contract ID is too short
        ContractIdTooShort,
        /// Missing Nick name in Master data record
        MissingMasterNickname,
        /// Missing Account id in Master data record
        MissingMasterAccount,
        /// Missing percentage in Master data record
        MissingMasterPercentage,
        /// Wrong Total Percentage Master data
        WrongTotalPercentageMaster,
        /// Missing Nick name in Composition data record
        MissingCompositionNickname,
        /// Missing Account id in Composition data record
        MissingCompositionAccount,
        /// Missing percentage in Composition data record
        MissingCompositionPercentage,
        /// Wrong Total Percentage Composition data
        WrongTotalPercentageComposition,
        /// Missing Other contract id
        MissingOtherContractsId,
        /// Missing other contracts percentage
        MissingOtherContractsPercentage,
        /// Wrong Total Percentage Other Contracts
        WrongTotalPercentageOtherContracts,
        /// Changed Proposal Id is already present on chain
        ChangeIdDuplicated,
        /// Missing Change Id
        MissingChangeId,
        /// Change Id not found
        ChangeIdNotFound,
        /// Signer as no rights to vote in this contract
        SignerHasNoRightsForVoting,
        /// Vote already caster for this change proposal
        VoteCastedAlready,
        /// Changed id field is empty
        ChangeIdTooShort,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized
        type Error = Error<T>;
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        // function to create a new Contract Rights Management (CRM), the crmid must be not already used json structures are expected. For crmdata:
        /*
        {
            "ipfshash": "xxxxxx"            				// ipfs hash of the metadata (one hash is usable for whole folder of files)
            "ipfshashprivate": ["xxxxxx","yyyyyyyy",..]     // ipfs hash array for the private files (audio and artworks)
            "globalquorum": 80			    				// the quorum required to change the shares of master/composition and othercontracts (crowdfundingshare are not changeable)
            "mastershare":30,               				// the shares for the master
            "masterquorum":51,								// the quorum required to change the master data
            "compositionshare": 30,         				// the shares of the composition group
            "compositionquorum":51,							// the quorum required to change the composition data
            "othercontractsshare": 20, 						// other contracts crowdfundingshare get shares (optional)
            "othercontratsquorum":75,  						// the quorum required to change the other countracts data
            "crowdfundingshare": 20,  						// crowd founders can get share
            "crowdfounders": "xxxxxx"					    // crowd funding campaign Id
        }
        for example:
        cmmrid can be: 3
        crmdata can be:
        {"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":30,"compositionquorum":51,"othercontractsshare":20,"othercontractsquorum":51}
        master can be:
        {"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage": 50},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}
        composition can be:
        {"composition": [{"nickname": "Charlie","account": "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname": "Dave","account": "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}
        Other Contracts shares can be (contracts id must exist on chain):
        {"othercontracts": [{"id": 1,"percentage":50},{"id": 2,"percentage":50}]}
        for Empty field you can use:
        {}
        */
        #[weight = 50_000]
        pub fn new_contract(origin, crmid: u32, crmdata: Vec<u8>,master: Vec<u8>,composition:Vec<u8>,othercontracts: Vec<u8>) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check crm data
            ensure!(crmdata.len() >= 32, Error::<T>::CrmDataTooShort); //check minimum length
            ensure!(crmdata.len() <= 1024, Error::<T>::CrmDataTooLong);  // check maximum length
            // check master data
            ensure!(master.len() >= 8, Error::<T>::MasterTooShort); //check minimum length
            ensure!(master.len() <= 1024, Error::<T>::MasterTooLong);  // check maximum length
            // check composition data
            ensure!(composition.len() >= 8, Error::<T>::CompositionTooShort); //check minimum length
            ensure!(composition.len() <= 1024, Error::<T>::CompositionTooLong);  // check maximum length
            // check Other Contracts data
            ensure!(othercontracts.len() <= 1024, Error::<T>::OtherContractsTooLong);  // check maximum length
            // check oracleid
            ensure!(crmid > 0, Error::<T>::InvalidValue); //check for crmid length >0
            // check of the crmid is free
            ensure!(!CrmData::contains_key(&crmid), Error::<T>::DuplicatedCrmId);
            // check json validity
            let js=crmdata.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJsonCrmData);

            // check ipfshash
            let jsf=crmdata.clone();
            let ipfshash=json_get_value(jsf,"ipfshash".as_bytes().to_vec());
            ensure!(ipfshash.len() >= 46, Error::<T>::InvalidIpfsHash); //check minimum length for the Ipfs Hash
            // check ipfshash private
            let jsfp=crmdata.clone();
            let ipfshashprivate=json_get_value(jsfp,"ipfshashprivate".as_bytes().to_vec());
            ensure!(ipfshashprivate.len() >= 46, Error::<T>::InvalidIpfsHashPrivate);  //check minimum length for the Ipfs Hash Private
            // check globalquorum
            let jsgq=crmdata.clone();
            let globalquorum=json_get_value(jsgq,"globalquorum".as_bytes().to_vec());
            let globalquorumvalue=vecu8_to_u32(globalquorum);
            ensure!(globalquorumvalue > 0, Error::<T>::InvalidGlobalQuorum); //check Global Quorum that must be > 0
            ensure!(globalquorumvalue <= 100, Error::<T>::InvalidGlobalQuorum); //check Global Quorum that must be <=100
            // check master shares
            let jsms=crmdata.clone();
            let mastershare=json_get_value(jsms,"mastershare".as_bytes().to_vec());
            let mastersharevalue=vecu8_to_u32(mastershare);
            ensure!(mastersharevalue > 0, Error::<T>::InvalidMasterShare); //check Master Shares  that must be > 0
            ensure!(mastersharevalue <= 100, Error::<T>::InvalidMasterShare); //check Master Shares that must be <=100
            // check master quorum
            let jsmq=crmdata.clone();
            let masterquorum=json_get_value(jsmq,"masterquorum".as_bytes().to_vec());
            let masterquorumvalue=vecu8_to_u32(masterquorum);
            ensure!(masterquorumvalue > 0, Error::<T>::InvalidMasterQuorum); //check Master Quorum that must be > 0
            ensure!(masterquorumvalue <= 100, Error::<T>::InvalidMasterQuorum); //check Master Quorum that must be <=100
            // check composition shares
            let jscs=crmdata.clone();
            let compositionshare=json_get_value(jscs,"compositionshare".as_bytes().to_vec());
            let compositionsharevalue=vecu8_to_u32(compositionshare);
            ensure!(compositionsharevalue > 0, Error::<T>::InvalidCompositionShare); //check Composition Shares  that must be > 0
            ensure!(compositionsharevalue <= 100, Error::<T>::InvalidCompositionShare); //check Composition Shares that must be <=100
            // check composition quorum
            let jscq=crmdata.clone();
            let compositionquorum=json_get_value(jscq,"compositionquorum".as_bytes().to_vec());
            let compositionquorumvalue=vecu8_to_u32(compositionquorum);
            ensure!(compositionquorumvalue > 0, Error::<T>::InvalidCompositionQuorum); //check Composition Quorum  that must be > 0
            ensure!(compositionquorumvalue <= 100, Error::<T>::InvalidCompositionQuorum); //check Composition Quorum that must be <=100
            // check othercontracts shares
            let jsos=crmdata.clone();
            let othercontractsshare=json_get_value(jsos,"othercontractsshare".as_bytes().to_vec());
            let othercontractssharevalue=vecu8_to_u32(othercontractsshare);
            ensure!(othercontractssharevalue <= 100, Error::<T>::InvalidOtherContractsShare); 	//check Composition Shares that must be <=100
            // check other contracts quorum
            let jsoq=crmdata.clone();
            let othercontractsquorum=json_get_value(jsoq,"othercontractsquorum".as_bytes().to_vec());
            let othercontractsquorumvalue=vecu8_to_u32(othercontractsquorum);
            ensure!(othercontractsquorumvalue <= 100, Error::<T>::InvalidOtherContractsQuorum); //check other Contracts Quorum that must be <=100
            // check crowdfundingshare
            let jscf=crmdata.clone();
            let crodwfundingshare=json_get_value(jscf,"crodwfundingshares".as_bytes().to_vec());
            let crodwfundingsharevalue=vecu8_to_u32(crodwfundingshare);
            ensure!(crodwfundingsharevalue <= 100, Error::<T>::InvalidCrowdFundingshares); //check Crowd Funding Shares that must be <=100
            // check that the total shares are = 100
            let totalshares=mastersharevalue+compositionsharevalue+othercontractssharevalue+crodwfundingsharevalue;
            ensure!(totalshares == 100, Error::<T>::InvalidTotalShares); //check total shares that must be 100

            // check validity of master data
            let masterclone=master.clone();
            // check for a valid json
            ensure!(json_check_validity(masterclone),Error::<T>::InvalidJsonCrmMaster);
            let mut x=0;
            let mut totpercentage:u32 = 0;
            // check validity of records for Master Data
            loop {
                let jr=json_get_recordvalue(master.clone(),x);
                if jr.is_empty() {
                    break;
                }
                // check for nickname
                let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
                ensure!(!nickname.is_empty(), Error::<T>::MissingMasterNickname);
                // check for account address
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingMasterPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                ensure!(percentagevalue >0, Error::<T>::MissingMasterPercentage);
                // sum percentage to totpercentage
                totpercentage += percentagevalue;
                x += 1;
            }
            // check the total percentage is = 100 TODO
            ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageMaster);

            // check validity of composition data
            let compositionclone=composition.clone();
            // check for a valid json
            ensure!(json_check_validity(compositionclone),Error::<T>::InvalidJsonCrmComposition);
            x=0;
            totpercentage=0;
            // check validity of records for Composition Data
            loop {
                let jr=json_get_recordvalue(composition.clone(),x);
                if jr.is_empty() {
                    break;
                }
                // check for nickname
                let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
                ensure!(!nickname.is_empty(), Error::<T>::MissingCompositionNickname);
                // check for account address
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingCompositionAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingCompositionPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                ensure!(percentagevalue >0, Error::<T>::MissingCompositionPercentage);
                // sum percentage to totpercentage
                totpercentage+=percentagevalue;
                x+=1;
            }
            // check the total percentage is = 100
            ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageComposition);


            // Other contracts are optional we check the validity if there is a value only
            if othercontracts.len()>10 {
                // check validity of othercontracts data
                let othercontractsclone=othercontracts.clone();
                // check for a valid json
                ensure!(json_check_validity(othercontractsclone),Error::<T>::InvalidJsonCrmOtherContracts);
                x=0;
                totpercentage= 0;
                // check validity of records for other contracts data
                loop {
                    let jr=json_get_recordvalue(othercontracts.clone(),x);
                    if jr.is_empty() {
                        break;
                    }
                    // check for id
                    let id=json_get_value(jr.clone(),"id".as_bytes().to_vec());
                    ensure!(!id.is_empty(), Error::<T>::MissingOtherContractsId);
                    let idvalue=vecu8_to_u32(id);
                    // check that the id is on chain
                    ensure!(CrmData::contains_key(&idvalue), Error::<T>::InvalidContractId);
                    // check for percentage
                    let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                    ensure!(!percentage.is_empty(), Error::<T>::MissingOtherContractsPercentage);
                    // convert percentage from vec to u32
                    let percentagevalue=vecu8_to_u32(percentage);
                    ensure!(percentagevalue >0, Error::<T>::MissingOtherContractsPercentage);
                    // sum percentage to totpercentage
                    totpercentage+=percentagevalue;
                    x+=1;
                }
                // check the total percentage is = 100
                ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageOtherContracts);
            }

            //****************************************
            // STORING DATA
            //****************************************
            // Write storage for crmdata
            CrmData::insert(&crmid, crmdata);
            // Write the storage for master data
            CrmMasterData::insert(crmid, master);
            // Write the storage for Composition data
            CrmCompositionData::insert(crmid, composition);
            // write the storage for Other Contracts data (optional)
            if !othercontracts.is_empty() {
                // Update storage for Other Contracts data
                CrmOtherContractsData::insert(crmid, othercontracts);
            }
            // Emit an event
            Self::deposit_event(RawEvent::CrmAdded(sender,crmid));
            // Return a successful DispatchResult
            Ok(())
        }



        /// Submit a change proposal for CRM main data that must be approved by voting
        #[weight = 50_000]
        pub fn change_proposal_crmdata(origin, changeid: u32, crmdata: Vec<u8>) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check that at the least some data to change has been received and it's not too long
            ensure!(!crmdata.is_empty(), Error::<T>::MissingContractData);
            ensure!(crmdata.len()<1024, Error::<T>::CrmDataTooLong);
            // check the validity of the proposed CRM data
            let js=crmdata.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check crmid field in json
            let jscm=crmdata.clone();
            let crmidjs=json_get_value(jscm,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidjs);
            // check the contract id (crmid field in json), IS on chain
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check the changeid is NOT on chain
            ensure!(!CrmDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdDuplicated);
            // get the currentquorum for Global data from main contractid
            let crmdataq=CrmData::get(&crmid).unwrap();
            let currentquorumj=json_get_value(crmdataq,"globalquorum".as_bytes().to_vec());
            let currentquorum=vecu8_to_u32(currentquorumj);
            ensure!(currentquorum >0 && currentquorum <=100, Error::<T>::InvalidMasterQuorum);
            // check ipfshash
            let jsf=crmdata.clone();
            let ipfshash=json_get_value(jsf,"ipfshash".as_bytes().to_vec());
            ensure!(ipfshash.len() >= 46, Error::<T>::InvalidIpfsHash); //check minimum length for the Ipfs Hash
            // check ipfshash private
            let jsfp=crmdata.clone();
            let ipfshashprivate=json_get_value(jsfp,"ipfshashprivate".as_bytes().to_vec());
            ensure!(ipfshashprivate.len() >= 46, Error::<T>::InvalidIpfsHashPrivate); //check minimum length for the Ipfs Hash Private
            // check globalquorum
            let jsgq=crmdata.clone();
            let globalquorum=json_get_value(jsgq,"globalquorum".as_bytes().to_vec());
            let globalquorumvalue=vecu8_to_u32(globalquorum);
            ensure!(globalquorumvalue > 0 && globalquorumvalue <= 100, Error::<T>::InvalidGlobalQuorum);
            // check master shares
            let jsms=crmdata.clone();
            let mastershare=json_get_value(jsms,"mastershare".as_bytes().to_vec());
            let mastersharevalue=vecu8_to_u32(mastershare);
            ensure!(mastersharevalue > 0 && mastersharevalue <= 100, Error::<T>::InvalidMasterShare); //check Master Shares  that must be > 0
            // check master quorum
            let jsmq=crmdata.clone();
            let masterquorum=json_get_value(jsmq,"masterquorum".as_bytes().to_vec());
            let masterquorumvalue=vecu8_to_u32(masterquorum);
            ensure!(masterquorumvalue > 0 && masterquorumvalue <= 100, Error::<T>::InvalidMasterQuorum); //check Master Quorum that must be > 0
            // check composition shares
            let jscs=crmdata.clone();
            let compositionshare=json_get_value(jscs,"compositionshare".as_bytes().to_vec());
            let compositionsharevalue=vecu8_to_u32(compositionshare);
            ensure!(compositionsharevalue > 0 && compositionsharevalue <= 100, Error::<T>::InvalidCompositionShare); //check Composition Shares  that must be > 0
            // check composition quorum
            let jscq=crmdata.clone();
            let compositionquorum=json_get_value(jscq,"compositionquorum".as_bytes().to_vec());
            let compositionquorumvalue=vecu8_to_u32(compositionquorum);
            ensure!(compositionquorumvalue > 0 && compositionquorumvalue <= 100, Error::<T>::InvalidCompositionQuorum); //check Composition Quorum  that must be > 0
            // check othercontracts shares
            let jsos=crmdata.clone();
            let othercontractsshare=json_get_value(jsos,"othercontractsshare".as_bytes().to_vec());
            let othercontractssharevalue=vecu8_to_u32(othercontractsshare);
            ensure!(othercontractssharevalue <= 100, Error::<T>::InvalidOtherContractsShare); 	//check Composition Shares that must be <=100
            // check other contracts quorum
            let jsoq=crmdata.clone();
            let othercontractsquorum=json_get_value(jsoq,"othercontractsquorum".as_bytes().to_vec());
            let othercontractsquorumvalue=vecu8_to_u32(othercontractsquorum);
            ensure!(othercontractsquorumvalue <= 100, Error::<T>::InvalidOtherContractsQuorum); //check other Contracts Quorum that must be <=100
            // check crowdfundingshare
            let jscf=crmdata.clone();
            let crodwfundingshare=json_get_value(jscf,"crodwfundingshares".as_bytes().to_vec());
            let crodwfundingsharevalue=vecu8_to_u32(crodwfundingshare);
            ensure!(crodwfundingsharevalue <= 100, Error::<T>::InvalidCrowdFundingshares); //check Crowd Funding Shares that must be <=100
            // check that the total shares are = 100
            let totalshares=mastersharevalue+compositionsharevalue+othercontractssharevalue+crodwfundingsharevalue;
            ensure!(totalshares == 100, Error::<T>::InvalidTotalShares); //check total shares that must be 100
            // store the proposal data in the queue.
            CrmDataChangeProposal::insert(changeid, crmdata);
            // store initial voting results with current quorum required to change the data
            let v= Voting {
                changeid,
                crmid,
                quorum: currentquorum,
                nrvotesyes: 0,
                nrvotesno: 0,
                percvotesyes: 0,
                percvotesno: 0,
            };
            CrmDataChangeVotingResult::insert(changeid,v);
            // Emit an event
            Self::deposit_event(RawEvent::CrmDataNewChangeProposal(sender,crmid,changeid));
            Ok(())
        }
        /// Vote a change proposal for CRM data
        #[weight = 10_000]
        pub fn vote_proposal_crmdata(origin, changeid: u32, vote: bool) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check changeid
            ensure!(changeid > 0, Error::<T>::ChangeIdTooShort); //check minimum length
            // check the changeid change proposal is on chain
            ensure!(CrmDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdNotFound);
            // check for double voting
            ensure!(!CrmDataChangeVoteCasted::<T>::contains_key(&sender,changeid), Error::<T>::VoteCastedAlready);
            // get crmid from the change proposal
            let jsc=CrmDataChangeProposal::get(&changeid).unwrap();
            let crmidj=json_get_value(jsc,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidj);
            // check the contract id is on chain
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);

            // get the percentage of votes for "Masters"
            let crmdata=CrmData::get(&crmid).unwrap_or_default();
            let js=crmdata.clone();
            let mastershare=json_get_value(js,"mastershare".as_bytes().to_vec());
            let mastersharevalue=vecu8_to_u32(mastershare);
            // get the percentage of votes for "Composition"
            let jsc=crmdata.clone();
            let compositionshare=json_get_value(jsc,"compositionshare".as_bytes().to_vec());
            let compositionsharevalue=vecu8_to_u32(compositionshare);
            // get the percentage of votes for "OtherContracts"
            let jsc=crmdata;
            let othercontractsshare=json_get_value(jsc,"othercontractsshare".as_bytes().to_vec());
            let othercontractssharevalue=vecu8_to_u32(othercontractsshare);
            // check if the signer is one of the Master Accounts
            let masterdata=CrmMasterData::get(crmid).unwrap_or_default();
            let mut x=0;
            let mut votepercentage=0;
            loop {
                let jr=json_get_recordvalue(masterdata.clone(),x);
                if jr.is_empty(){
                    break;
                }
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingMasterPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                // convert Account Vec<u8> to AccountId formatm first in str
                let account_slice=account.as_slice();
                let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                //debug::info!("MASTER - accountstr: {}",accountstr);
                //converts the str to byte array
                let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                // finally convert to AccountId
                let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap_or_default();
                // verify account matching between AccountId types
                if accountid==sender && mastersharevalue>0 {
                        votepercentage += percentagevalue*mastersharevalue/100;
                }
                x+=1;
            }
            // check if the signer is one of the Composition Accounts
            let compositiondata=CrmCompositionData::get(crmid).unwrap_or_default();
            x=0;
            loop {
                let jr=json_get_recordvalue(compositiondata.clone(),x);
                if jr.is_empty() {
                    break;
                }
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingCompositionAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingCompositionPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                // convert Account Vec<u8> to AccountId formatm first in str
                let account_slice=account.as_slice();
                let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                //converts the str to byte array
                let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                // finally convert to AccountId
                let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap();
                // verify account matching between AccountId types
                //debug::info!("COMPOSITION - accountid: {:?} Signed: {:?}",accountid,sender);
                if accountid==sender{
                    //debug::info!("COMPOSITION IS MATCHING - compositionsharevalue:{} percentagevalue: {} percentage_str: {}",compositionsharevalue,percentagevalue,percentage_str);
                    if compositionsharevalue>0 {
                        votepercentage += percentagevalue*compositionsharevalue/100
                        //debug::info!("COMPOSITION - votepercentage:{} ",votepercentage);
                    }
                }
                x+=1;
            }
            // check if the signer is part of any "other contract"
            let othercontractsdata=CrmOtherContractsData::get(crmid).unwrap_or_default();
            //debug::info!("[DEBUG] othercontractsdata: {:?}",othercontractsdata);
            if othercontractsdata.len()>10{
                x=0;
                loop {
                    let jr=json_get_recordvalue(othercontractsdata.clone(),x);
                    if jr.is_empty() {
                        break;
                    }
                    let id=json_get_value(jr.clone(),"id".as_bytes().to_vec());
                    ensure!(!id.is_empty(), Error::<T>::InvalidContractIdVoting);
                    let idvalue=vecu8_to_u32(id);
                    ensure!(idvalue >0, Error::<T>::InvalidContractIdVotingNumeric);
                    // check for percentage
                    let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                    ensure!(!percentage.is_empty(), Error::<T>::MissingOtherContractsPercentage);
                    // convert percentage from vec to u32
                    let percentagevalue=vecu8_to_u32(percentage);
                    ensure!(percentagevalue>0, Error::<T>::MissingOtherContractsPercentage);
                    // check Master record of the other contract
                    let mut xx=0;
                    let masterdata=CrmMasterData::get(idvalue).unwrap();
                    loop {
                        let jr=json_get_recordvalue(masterdata.clone(),xx);
                        if jr.is_empty() {
                            break;
                        }
                        let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                        ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                        // check for percentage
                        let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                        ensure!(!percentage.is_empty(), Error::<T>::MissingMasterPercentage);
                        // convert percentage from vec to u32
                        let percentagevalue=vecu8_to_u32(percentage);
                        // convert Account Vec<u8> to AccountId format, first in str
                        let account_slice=account.as_slice();
                        let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                        //debug::info!("[DEBUG] OTHER CONTRACTS - accountstr: {}",accountstr);
                        //converts the str to byte array
                        let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                        // finally convert to AccountId
                        let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap_or_default();
                        // verify account matching between AccountId types
                        if accountid == sender && othercontractssharevalue >0 {
                                votepercentage+=percentagevalue*othercontractssharevalue/100;
                        }
                        xx+=1;
                    }
                    x+=1;
                }
            }
            // check if the signer has rights to vote >0
            ensure!(votepercentage > 0, Error::<T>::SignerHasNoRightsForVoting);
            // store the vote
            let mut v:Voting=CrmDataChangeVotingResult::get(changeid).unwrap_or_default();
            let currentpervotesyes=v.percvotesyes;
            // update the voting structure
            if vote {
                v.nrvotesyes+=1;
                v.percvotesyes += votepercentage;
            }else {
                v.nrvotesno+=1;
                v.percvotesno+=votepercentage;
            }
            //update the storage with voting results
            CrmDataChangeVotingResult::remove(changeid);
            CrmDataChangeVotingResult::insert(changeid,v.clone());
            // store the vote for the account id
            CrmDataChangeVoteCasted::<T>::insert(sender.clone(),changeid,vote);
            // Emit an event to alert the user of the vote received
            //debug::info!("[DEBUG] Emit Event for Vote");
            Self::deposit_event(RawEvent::CrmDataChangeVote(sender.clone(),crmid,changeid));
            // if quorum has been reached, we replace the current CRM data with the one voted from the majority
            if v.percvotesyes>=v.quorum && v.quorum>=currentpervotesyes {
                //debug::info!("[DEBUG] CHANGE APPROVED ON CRMDATA!");
                let crmdata=CrmDataChangeProposal::get(changeid).unwrap();
                CrmData::remove(crmid);
                CrmData::insert(crmid, crmdata);
                // Emit an event to alert the user of the crm data change done
                Self::deposit_event(RawEvent::CrmDataChanged(sender,crmid));
            }
            // returns back with no errors
            Ok(())
        }
        /// Submit a change proposal for CRM master data that must be approved by voting from master members only
        #[weight = 50_000]
        pub fn change_proposal_crm_masterdata(origin, changeid: u32, masterdata: Vec<u8>) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check that at the least some data to change has been received and it's not too long
            ensure!(!masterdata.is_empty(), Error::<T>::MissingContractData);
            ensure!(masterdata.len()<1024, Error::<T>::CrmDataTooLong);
            // check the json validity of the proposed CRM master data
            let js=masterdata.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check crmid field in json
            let jscm=masterdata.clone();
            let crmidjs=json_get_value(jscm,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidjs);
            // check the contract id (crmid field in json), IS on chain on both storage, main and master data
            ensure!(CrmMasterData::contains_key(&crmid), Error::<T>::InvalidContractId);
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check the changeid is NOT on chain
            ensure!(!CrmMasterDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdDuplicated);
            // get the quorum for Master data from main contractid
            let crmdata=CrmData::get(&crmid).unwrap();
            let currentquorumj=json_get_value(crmdata,"masterquorum".as_bytes().to_vec());
            let currentquorum=vecu8_to_u32(currentquorumj);
            ensure!(currentquorum >0 && currentquorum <=100, Error::<T>::InvalidMasterQuorum);
            // check validity of master data
            let masterclone=masterdata.clone();
            // check for a valid json
            ensure!(json_check_validity(masterclone),Error::<T>::InvalidJson);
            let mut x=0;
            let mut totpercentage:u32 = 0;
            // check validity of records for Master Data
            loop {
                let jr=json_get_recordvalue(masterdata.clone(),x);
                if jr.is_empty() {
                    break;
                }
                // check for nickname
                let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
                ensure!(!nickname.is_empty(), Error::<T>::MissingMasterNickname);
                // check for account address
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingMasterPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                ensure!(percentagevalue >0, Error::<T>::MissingMasterPercentage);
                // sum percentage to totpercentage
                totpercentage+=percentagevalue;
                x+=1;
            }
            // check the total percentage is = 100 TODO
            ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageMaster);

            // store the proposal data in the queue.
            CrmMasterDataChangeProposal::insert(changeid, masterdata);
            // store initial voting results with current quorum required to change the data
            let v= Voting {
                changeid,
                crmid,
                quorum: currentquorum,
                nrvotesyes: 0,
                nrvotesno: 0,
                percvotesyes: 0,
                percvotesno: 0,
            };
            CrmMasterDataChangeVotingResult::insert(changeid,v);
            // Emit an event
            Self::deposit_event(RawEvent::CrmMasterDataNewChangeProposal(sender,crmid,changeid));
            Ok(())
        }
        /// Vote a change proposal for CRM master data
        #[weight = 10_000]
        pub fn vote_proposal_crm_masterdata(origin, changeid: u32, vote: bool) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check changeid
            ensure!(changeid > 0, Error::<T>::ChangeIdTooShort); //check minimum length
            // check the changeid change proposal is on chain
            ensure!(CrmMasterDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdNotFound);
            // check for double voting
            ensure!(!CrmMasterDataChangeVoteCasted::<T>::contains_key(&sender,changeid), Error::<T>::VoteCastedAlready);
            // get crmid from the change proposal
            let jsc=CrmMasterDataChangeProposal::get(&changeid).unwrap();
            let crmidj=json_get_value(jsc,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidj);
            // check the contract id is on chain
            ensure!(CrmMasterData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check if the signer is one of the Master Accounts
            let masterdata=CrmMasterData::get(crmid).unwrap_or_default();
            let mut x=0;
            let mut votepercentage=0;
            loop {
                let jr=json_get_recordvalue(masterdata.clone(),x);
                if jr.is_empty() {
                    break;
                }
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingMasterPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                // convert Account Vec<u8> to AccountId formatm first in str
                let account_slice=account.as_slice();
                let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                //converts the str to byte array
                let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                // finally convert to AccountId
                let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap_or_default();
                // verify account matching between AccountId types
                if accountid==sender{
                    votepercentage+=percentagevalue;
                }
                x+=1;
            }
            // check if the signer has rights to vote >0
            ensure!(votepercentage > 0, Error::<T>::SignerHasNoRightsForVoting);
            // store the vote
            let mut v:Voting=CrmMasterDataChangeVotingResult::get(changeid).unwrap_or_default();
            let currentpervotesyes=v.percvotesyes;
            // update the voting structure
            if vote {
                v.nrvotesyes+=1;
                v.percvotesyes+=votepercentage;
            }else {
                v.nrvotesno+=1;
                v.percvotesno+=votepercentage;
            }
            //update the storage with voting results
            CrmMasterDataChangeVotingResult::remove(changeid);
            CrmMasterDataChangeVotingResult::insert(changeid,v.clone());
            // store the vote for the account id
            CrmMasterDataChangeVoteCasted::<T>::insert(sender.clone(),changeid,vote);
            // Emit an event to alert the user of the vote received
            Self::deposit_event(RawEvent::CrmMasterDataChangeVote(sender.clone(),crmid,changeid));
            // if quorum has been reached, we replace the current CRM data with the one voted from the majority
            if v.percvotesyes>=v.quorum && v.quorum>currentpervotesyes {
                //debug::info!("[DEBUG] CHANGE APPROVED ON CRMDATA!");
                let crmdata=CrmMasterDataChangeProposal::get(changeid).unwrap();
                CrmMasterData::remove(crmid);
                CrmMasterData::insert(crmid, crmdata);
                // Emit an event to alert the user of the crm data change done
                Self::deposit_event(RawEvent::CrmMasterDataChanged(sender,crmid));
            }
            // returns back with no errors
            Ok(())
        }
        /// Submit a change proposal for CRM composition data that must be approved by voting from composition members only
        #[weight = 50_000]
        pub fn change_proposal_crm_compositiondata(origin, changeid: u32, compositiondata: Vec<u8>) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check that at the least some data to change has been received and it's not too long
            ensure!(!compositiondata.is_empty(), Error::<T>::MissingContractData);
            ensure!(compositiondata.len()<1024, Error::<T>::CrmDataTooLong);
            // check the json validity of the proposed CRM composition data
            let js=compositiondata.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check crmid field in json
            let jscm=compositiondata.clone();
            let crmidjs=json_get_value(jscm,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidjs);
            // check the contract id (crmid field in json), IS on chain on both storage, main and composition data
            ensure!(CrmCompositionData::contains_key(&crmid), Error::<T>::InvalidContractId);
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check the changeid is NOT on chain
            ensure!(!CrmCompositionDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdDuplicated);
            // get the quorum for composition data from main contractid
            let crmdata=CrmData::get(&crmid).unwrap();
            let currentquorumj=json_get_value(crmdata,"compositionquorum".as_bytes().to_vec());
            let currentquorum=vecu8_to_u32(currentquorumj);
            ensure!(currentquorum >0 && currentquorum <=100, Error::<T>::InvalidCompositionQuorum);
            // check validity of composition data
            let compositionclone=compositiondata.clone();
            // check for a valid json
            ensure!(json_check_validity(compositionclone),Error::<T>::InvalidJson);
            let mut x=0;
            let mut totpercentage:u32 = 0;
            // check validity of records for Composition Data
            loop {
                let jr=json_get_recordvalue(compositiondata.clone(),x);
                if jr.is_empty() {
                    break;
                }
                // check for nickname
                let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
                ensure!(!nickname.is_empty(), Error::<T>::MissingCompositionNickname);
                // check for account address
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingCompositionAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingCompositionPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                ensure!(percentagevalue >0, Error::<T>::MissingCompositionPercentage);
                // sum percentage to totpercentage
                totpercentage+=percentagevalue;
                x+=1;
            }
            // check the total percentage is = 100 TODO
            ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageComposition);

            // store the proposal data in the queue.
            CrmCompositionDataChangeProposal::insert(changeid, compositiondata);
            // store initial voting results with current quorum required to change the data
            let v= Voting {
                changeid,
                crmid,
                quorum: currentquorum,
                nrvotesyes: 0,
                nrvotesno: 0,
                percvotesyes: 0,
                percvotesno: 0,
            };
            CrmCompositionDataChangeVotingResult::insert(changeid,v);
            // Emit an event
            Self::deposit_event(RawEvent::CrmCompositionDataNewChangeProposal(sender,crmid,changeid));
            Ok(())
        }
        /// Vote a change proposal for CRM composition data
        #[weight = 10_000]
        pub fn vote_proposal_crm_compositiondata(origin, changeid: u32, vote: bool) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check changeid
            ensure!(changeid > 0, Error::<T>::ChangeIdTooShort); //check minimum length
            // check the changeid change proposal is on chain
            ensure!(CrmCompositionDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdNotFound);
            // check for double voting
            ensure!(!CrmCompositionDataChangeVoteCasted::<T>::contains_key(&sender,changeid), Error::<T>::VoteCastedAlready);
            // get crmid from the change proposal
            let jsc=CrmCompositionDataChangeProposal::get(&changeid).unwrap();
            let crmidj=json_get_value(jsc,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidj);
            // check the contract id is on chain
            ensure!(CrmCompositionData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check if the signer is one of the composition Accounts
            let compositiondata=CrmCompositionData::get(crmid).unwrap_or_default();
            let mut x=0;
            let mut votepercentage=0;
            loop {
                let jr=json_get_recordvalue(compositiondata.clone(),x);
                if jr.is_empty(){
                    break;
                }
                let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                ensure!(!account.is_empty(), Error::<T>::MissingCompositionAccount);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingCompositionPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                // convert Account Vec<u8> to AccountId formatm first in str
                let account_slice=account.as_slice();
                let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                //converts the str to byte array
                let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                // finally convert to AccountId
                let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap_or_default();
                // verify account matching between AccountId types
                if accountid==sender{
                    votepercentage+=percentagevalue;
                }
                x+=1;
            }
            // check if the signer has rights to vote >0
            ensure!(votepercentage > 0, Error::<T>::SignerHasNoRightsForVoting);
            // store the vote
            let mut v:Voting=CrmCompositionDataChangeVotingResult::get(changeid).unwrap_or_default();
            let currentpervotesyes=v.percvotesyes;
            // update the voting structure
            if vote {
                v.nrvotesyes+=1;
                v.percvotesyes+=votepercentage;
            }else {
                v.nrvotesno+=1;
                v.percvotesno+=votepercentage;
            }
            //update the storage with voting results
            CrmCompositionDataChangeVotingResult::remove(changeid);
            CrmCompositionDataChangeVotingResult::insert(changeid,v.clone());
            // store the vote for the account id
            CrmCompositionDataChangeVoteCasted::<T>::insert(sender.clone(),changeid,vote);
            // Emit an event to alert the user of the vote received
            Self::deposit_event(RawEvent::CrmCompositionDataChangeVote(sender.clone(),crmid,changeid));
            // if quorum has been reached, we replace the current CRM data with the one voted from the majority
            if v.percvotesyes>=v.quorum && v.quorum>currentpervotesyes {
                let crmdata=CrmCompositionDataChangeProposal::get(changeid).unwrap();
                CrmCompositionData::remove(crmid);
                CrmCompositionData::insert(crmid, crmdata);
                // Emit an event to alert the user of the crm data change done
                Self::deposit_event(RawEvent::CrmCompositionDataChanged(sender,crmid));
            }
            // returns back with no errors
            Ok(())
        }
        /// Submit a change proposal for CRM Other Contracts data that must be approved by voting from master members only of the other contracts.
        #[weight = 50_000]
        pub fn change_proposal_crm_othercontractsdata(origin, changeid: u32, othercontractsdata: Vec<u8>) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check that at the least some data to change has been received and it's not too long
            ensure!(!othercontractsdata.is_empty(), Error::<T>::MissingContractData);
            ensure!(othercontractsdata.len()<1024, Error::<T>::CrmDataTooLong);
            // check the json validity of the proposed CRM composition data
            let js=othercontractsdata.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check crmid field in json
            let jscm=othercontractsdata.clone();
            let crmidjs=json_get_value(jscm,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidjs);
            // check the contract id (crmid field in json), IS on chain on both storage, main and composition data
            ensure!(CrmOtherContractsData::contains_key(&crmid), Error::<T>::InvalidContractId);
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check the changeid is NOT on chain
            ensure!(!CrmOtherContractsDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdDuplicated);
            // get the quorum for other contracts data from main contractid
            let crmdata=CrmData::get(&crmid).unwrap();
            let currentquorumj=json_get_value(crmdata,"othercontractsquorum".as_bytes().to_vec());
            let currentquorum=vecu8_to_u32(currentquorumj);
            ensure!(currentquorum >0 && currentquorum <=100, Error::<T>::InvalidOtherContractsQuorum);
            // check validity of othercontracts data
            let othercontractsclone=othercontractsdata.clone();
            // check for a valid json
            ensure!(json_check_validity(othercontractsclone),Error::<T>::InvalidJson);
            let mut x=0;
            let mut totpercentage= 0;
            // check validity of records for other contracts data
            loop {
                let jr=json_get_recordvalue(othercontractsdata.clone(),x);
                if jr.is_empty() {
                    break;
                }
                // check for id
                let id=json_get_value(jr.clone(),"id".as_bytes().to_vec());
                ensure!(!id.is_empty(), Error::<T>::MissingOtherContractsId);
                // convert id from vec to u32
                let idvalue=vecu8_to_u32(id);
                // check that the id is on chain
                ensure!(CrmData::contains_key(&idvalue), Error::<T>::InvalidContractId);
                // check for percentage
                let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                ensure!(!percentage.is_empty(), Error::<T>::MissingOtherContractsPercentage);
                // convert percentage from vec to u32
                let percentagevalue=vecu8_to_u32(percentage);
                ensure!(percentagevalue >0, Error::<T>::MissingOtherContractsPercentage);
                // sum percentage to totpercentage
                totpercentage+=percentagevalue;
                x+=1;
            }
            // check the total percentage is = 100
            ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageOtherContracts);

            // store the proposal data in the queue.
            CrmOtherContractsDataChangeProposal::insert(changeid, othercontractsdata);
            // store initial voting results with current quorum required to change the data
            let v= Voting {
                changeid,
                crmid,
                quorum: currentquorum,
                nrvotesyes: 0,
                nrvotesno: 0,
                percvotesyes: 0,
                percvotesno: 0,
            };
            CrmOtherContractsDataChangeVotingResult::insert(changeid,v);
            // Emit an event
            Self::deposit_event(RawEvent::CrmOtherContractsDataNewChangeProposal(sender,crmid,changeid));
            Ok(())
        }
        /// Vote a change proposal for CRM data
        #[weight = 10_000]
        pub fn vote_proposal_crm_othercontractsdata(origin, changeid: u32, vote: bool) -> dispatch::DispatchResult {
            // Check that the extrinsic is signed and get the signer.
            let sender = ensure_signed(origin)?;
            // check changeid
            ensure!(changeid > 0, Error::<T>::ChangeIdTooShort); //check minimum length
            // check the changeid change proposal is on chain
            ensure!(CrmOtherContractsDataChangeProposal::contains_key(changeid), Error::<T>::ChangeIdNotFound);
            // check for double voting
            ensure!(!CrmOtherContractsDataChangeVoteCasted::<T>::contains_key(&sender,changeid), Error::<T>::VoteCastedAlready);
            // get crmid from the change proposal
            let jsc=CrmOtherContractsDataChangeProposal::get(&changeid).unwrap();
            let crmidj=json_get_value(jsc,"crmid".as_bytes().to_vec());
            let crmid=vecu8_to_u32(crmidj);
            // check the contract id is on chain
            ensure!(CrmData::contains_key(&crmid), Error::<T>::InvalidContractId);
            // check if the signer is part of any "other contract"
            let othercontractsdata=CrmOtherContractsData::get(crmid).unwrap_or_default();
            let mut votepercentage=0;
            if othercontractsdata.len()>10{
                let mut x=0;
                loop {
                    let jr=json_get_recordvalue(othercontractsdata.clone(),x);
                    if jr.is_empty() {
                        break;
                    }
                    let id=json_get_value(jr.clone(),"id".as_bytes().to_vec());
                    ensure!(!id.is_empty(), Error::<T>::InvalidContractIdVoting);
                    let idvalue=vecu8_to_u32(id);
                    ensure!(idvalue >0, Error::<T>::InvalidContractIdVotingNumeric);
                    // check for percentage
                    let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                    ensure!(!percentage.is_empty(), Error::<T>::MissingOtherContractsPercentage);
                    // convert percentage from vec to u32
                    let percentagevalue=vecu8_to_u32(percentage);
                    ensure!(percentagevalue>0, Error::<T>::MissingOtherContractsPercentage);
                    // check Master record of the other contract
                    let mut xx=0;
                    let masterdata=CrmMasterData::get(idvalue).unwrap();
                    loop {
                        let jr=json_get_recordvalue(masterdata.clone(),xx);
                        if jr.is_empty() {
                            break;
                        }
                        let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
                        ensure!(!account.is_empty(), Error::<T>::MissingMasterAccount);
                        // check for percentage
                        let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
                        ensure!(!percentage.len() >0, Error::<T>::MissingMasterPercentage);
                        let percentagevalue=vecu8_to_u32(percentage);
                        // convert Account Vec<u8> to AccountId format, first in str
                        let account_slice=account.as_slice();
                        let accountstr: &str =  str::from_utf8(&account_slice[3..]).unwrap_or_default();
                        //converts the str to byte array
                        let buffer: [u8; 32] =  hex::FromHex::from_hex(&accountstr).unwrap_or_default();
                        // finally convert to AccountId
                        let accountid=T::AccountId::decode(&mut &buffer[..]).unwrap_or_default();
                        // verify account matching between AccountId types
                        if accountid == sender {
                                votepercentage+=percentagevalue;
                        }
                        xx+=1;
                    }
                    x+=1;
                }
            }
            // check if the signer has rights to vote >0
            ensure!(votepercentage > 0, Error::<T>::SignerHasNoRightsForVoting);
            // store the vote
            let mut v:Voting=CrmOtherContractsDataChangeVotingResult::get(changeid).unwrap_or_default();
            let currentpervotesyes=v.percvotesyes;
            // update the voting structure
            if vote {
                v.nrvotesyes+=1;
                v.percvotesyes+=votepercentage;
            }else {
                v.nrvotesno+=1;
                v.percvotesno+=votepercentage;
            }
            //update the storage with voting results
            CrmOtherContractsDataChangeVotingResult::remove(changeid);
            CrmOtherContractsDataChangeVotingResult::insert(changeid,v.clone());
            // store the vote for the account id
            CrmOtherContractsDataChangeVoteCasted::<T>::insert(sender.clone(),changeid,vote);
            // Emit an event to alert the user of the vote received
            Self::deposit_event(RawEvent::CrmOtherContractsDataChangeVote(sender.clone(),crmid,changeid));
            // if quorum has been reached, we replace the current CRM Other Contracts data with the one voted from the majority
            if v.percvotesyes>=v.quorum && v.quorum>currentpervotesyes {
                let crmdata=CrmOtherContractsDataChangeProposal::get(changeid).unwrap();
                CrmOtherContractsData::remove(crmid);
                CrmOtherContractsData::insert(crmid, crmdata);
                // Emit an event to alert the user of the crm data change done
                Self::deposit_event(RawEvent::CrmOtherContractsDataChanged(sender,crmid));
            }
            // returns back with no errors
            Ok(())
        }
    }
}
// function to validate a json string for no/std. It does not allocate of memory
fn json_check_validity(j: Vec<u8>) -> bool {
    // minimum lenght of 2
    if j.len() < 2 {
        return false;
    }
    // checks star/end with {}
    if *j.get(0).unwrap() == b'{' && *j.last().unwrap() != b'}' {
        return false;
    }
    // checks start/end with []
    if *j.get(0).unwrap() == b'[' && *j.last().unwrap() != b']' {
        return false;
    }
    // check that the start is { or [
    if *j.get(0).unwrap() != b'{' && *j.get(0).unwrap() != b'[' {
        return false;
    }
    //checks that end is } or ]
    if *j.last().unwrap() != b'}' && *j.last().unwrap() != b']' {
        return false;
    }
    //checks " opening/closing and : as separator between name and values
    let mut s: bool = true;
    let mut d: bool = true;
    let mut pg: bool = true;
    let mut ps: bool = true;
    let mut bp = b' ';
    for b in j {
        if b == b'[' && s {
            ps = false;
        }
        if b == b']' && s && !ps {
            ps = true;
        } else if b == b']' && s && ps {
            ps = false;
        }
        if b == b'{' && s {
            pg = false;
        }
        if b == b'}' && s && !pg {
            pg = true;
        } else if b == b'}' && s && pg {
            pg = false;
        }
        if b == b'"' && s && bp != b'\\' {
            s = false;
            bp = b;
            d = false;
            continue;
        }
        if b == b':' && s {
            d = true;
            bp = b;
            continue;
        }
        if b == b'"' && !s && bp != b'\\' {
            s = true;
            bp = b;
            d = true;
            continue;
        }
        bp = b;
    }
    //fields are not closed properly
    if !s {
        return false;
    }
    //fields are not closed properly
    if !d {
        return false;
    }
    //fields are not closed properly
    if !ps {
        return false;
    }
    // every ok returns true
    true
}
// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
fn json_get_recordvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut op = true;
    let mut cn = 0;
    let mut lb = b' ';
    for b in ar {
        if b == b',' && op {
            cn += 1;
            continue;
        }
        if b == b'[' && op && lb != b'\\' {
            continue;
        }
        if b == b']' && op && lb != b'\\' {
            continue;
        }
        if b == b'{' && op && lb != b'\\' {
            op = false;
        }
        if b == b'}' && !op && lb != b'\\' {
            op = true;
        }
        // field found
        if cn == p {
            result.push(b);
        }
        lb = b;
    }
    result
}

// function to get value of a field for Substrate runtime (no std library and no variable allocation)
fn json_get_value(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut k = Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl {
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in 0..jl {
        let mut m = 0;
        let mut xx = 0;
        if x + kl > jl {
            break;
        }
        for i in x..x + kl {
            if *j.get(i).unwrap() == *k.get(xx).unwrap() {
                m += 1;
            }
            xx += 1;
        }
        if m == kl {
            let mut lb = b' ';
            let mut op = true;
            let mut os = true;
            for i in x + kl..jl - 1 {
                if *j.get(i).unwrap() == b'[' && op && os {
                    os = false;
                }
                if *j.get(i).unwrap() == b'}' && op && !os {
                    os = true;
                }
                if *j.get(i).unwrap() == b':' && op {
                    continue;
                }
                if *j.get(i).unwrap() == b'"' && op && lb != b'\\' {
                    op = false;
                    continue;
                }
                if *j.get(i).unwrap() == b'"' && !op && lb != b'\\' {
                    break;
                }
                if *j.get(i).unwrap() == b'}' && op {
                    break;
                }
                if *j.get(i).unwrap() == b',' && op && os {
                    break;
                }
                result.push(j.get(i).unwrap().clone());
                lb = j.get(i).unwrap().clone();
            }
            break;
        }
    }
    result
}

// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
    let vslice = v.as_slice();
    let vstr = str::from_utf8(&vslice).unwrap_or("0");
    let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
    vvalue
}
