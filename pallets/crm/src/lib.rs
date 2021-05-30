#![cfg_attr(not(feature = "std"), no_std)]

/// CRM - Module to setup the contracts for rights management

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use core::str;
use core::str::FromStr;


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
	}
}

// Events used to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		CrmAdded(AccountId, u32),					// New contract has been added
		CrmDataChangeProposal(AccountId, Vec<u8>),		// A proposal change has been submitted
		CrmChangeVote(AccountId, u32),		    	// A vote to a change proposal has been received
		CrmDataChanged(AccountId, u32),				// Crm data has been changed
		CrmMasterChanged(AccountId, u32),	    	// Crm master data has been changed
		CrmCompositionChanged(AccountId, u32),		// Crm composition data has been changed
		CrmOtherContractsChanged(AccountId,Vec<u8>),// Crm other contracts data has been changed
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
		/// Invalid ContractId that should be accountId+ u32
		InvalidContractId,
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
		/// Wrong Total Percentage Other Contracts
		WrongTotalPercentageOtherContracts,
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
		
		// function to create a new Contract Rights Management (CRM), the crmid must be not already used and in the crmdata a json structure is expected with the following fields:
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
		{"master": [{"nickname": "Bob","account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","percentage": 50},{"nickname": "Bob Stash","account": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc","percentage": 50}]}
		composition can be:
		{"composition": [{"nickname": "Charlie","account": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","percentage": 50},{"nickname": "Dave","account": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy","percentage": 50}]}
		Other Contracts shares can be (contracts id must exist on chain):
		{"othercontracts": [{"id": 1,"percentage": 50},{"id": 2,"percentage": 50}]}
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
			ensure!(CrmData::contains_key(&crmid)==false, Error::<T>::DuplicatedCrmId);
			// check json validity
			let js=crmdata.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
			
			// check ipfshash
			let jsf=crmdata.clone();
			let ipfshash=json_get_value(jsf,"ipfshash".as_bytes().to_vec());
			ensure!(ipfshash.len() >= 4, Error::<T>::InvalidIpfsHash); //check minimum length for the Ipfs Hash
			// check ipfshash private
			let jsfp=crmdata.clone();
			let ipfshashprivate=json_get_value(jsfp,"ipfshashprivate".as_bytes().to_vec());
			ensure!(ipfshashprivate.len() >= 4, Error::<T>::InvalidIpfsHashPrivate); //check minimum length for the Ipfs Hash Private
			// check globalquorum
			let jsgq=crmdata.clone();
			let globalquorum=json_get_value(jsgq,"globalquorum".as_bytes().to_vec());
			let globalquorum_slice=globalquorum.as_slice();
            let globalquorum_str=match str::from_utf8(&globalquorum_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let globalquorumvalue:u64 = match u64::from_str(globalquorum_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(globalquorumvalue > 0, Error::<T>::InvalidGlobalQuorum); //check Global Quorum that must be > 0
			ensure!(globalquorumvalue <= 100, Error::<T>::InvalidGlobalQuorum); //check Global Quorum that must be <=100
			// check master shares
			let jsms=crmdata.clone();
			let mastershare=json_get_value(jsms,"mastershare".as_bytes().to_vec());
			let mastershare_slice=mastershare.as_slice();
            let mastershare_str=match str::from_utf8(&mastershare_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let mastersharevalue:u64 = match u64::from_str(mastershare_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(mastersharevalue > 0, Error::<T>::InvalidMasterShare); //check Master Shares  that must be > 0
			ensure!(mastersharevalue <= 100, Error::<T>::InvalidMasterShare); //check Master Shares that must be <=100
			// check master quorum
			let jsmq=crmdata.clone();
			let masterquorum=json_get_value(jsmq,"masterquorum".as_bytes().to_vec());
			let masterquorum_slice=masterquorum.as_slice();
            let masterquorum_str=match str::from_utf8(&masterquorum_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let masterquorumvalue:u64 = match u64::from_str(masterquorum_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(masterquorumvalue > 0, Error::<T>::InvalidMasterQuorum); //check Master Quorum that must be > 0
			ensure!(masterquorumvalue <= 100, Error::<T>::InvalidMasterQuorum); //check Master Quorum that must be <=100
			// check composition shares
			let jscs=crmdata.clone();
			let compositionshare=json_get_value(jscs,"compositionshare".as_bytes().to_vec());
			let compositionshare_slice=compositionshare.as_slice();
            let compositionshare_str=match str::from_utf8(&compositionshare_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let compositionsharevalue:u64 = match u64::from_str(compositionshare_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(compositionsharevalue > 0, Error::<T>::InvalidCompositionShare); //check Composition Shares  that must be > 0
			ensure!(compositionsharevalue <= 100, Error::<T>::InvalidCompositionShare); //check Composition Shares that must be <=100
			// check composition quorum
			let jscq=crmdata.clone();
			let compositionquorum=json_get_value(jscq,"compositionquorum".as_bytes().to_vec());
			let compositionquorum_slice=compositionquorum.as_slice();
            let compositionquorum_str=match str::from_utf8(&compositionquorum_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let compositionquorumvalue:u64 = match u64::from_str(compositionquorum_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(compositionquorumvalue > 0, Error::<T>::InvalidCompositionQuorum); //check Composition Quorum  that must be > 0
			ensure!(compositionquorumvalue <= 100, Error::<T>::InvalidCompositionQuorum); //check Composition Quorum that must be <=100
			// check othercontracts shares
			let jsos=crmdata.clone();
			let othercontractsshare=json_get_value(jsos,"othercontractsshare".as_bytes().to_vec());
			let othercontractsshare_slice=othercontractsshare.as_slice();
            let othercontractsshare_str=match str::from_utf8(&othercontractsshare_slice){
                Ok(f) => f,
                Err(_) => "100"
            };
            let othercontractssharevalue:u64 = match u64::from_str(othercontractsshare_str){
                Ok(f) => f,
                Err(_) => 100,
            };
			ensure!(othercontractssharevalue <= 100, Error::<T>::InvalidOtherContractsShare); 	//check Composition Shares that must be <=100
			// check other contracts quorum
			let jsoq=crmdata.clone();
			let othercontractsquorum=json_get_value(jsoq,"othercontractsquorum".as_bytes().to_vec());
			let othercontractsquorum_slice=othercontractsquorum.as_slice();
            let othercontractsquorum_str=match str::from_utf8(&othercontractsquorum_slice){
                Ok(f) => f,
                Err(_) => "100"
            };
            let othercontractsquorumvalue:u64 = match u64::from_str(othercontractsquorum_str){
                Ok(f) => f,
                Err(_) => 100,
            };
			ensure!(othercontractsquorumvalue <= 100, Error::<T>::InvalidOtherContractsQuorum); //check other Contracts Quorum that must be <=100
			// check crowdfundingshare
			let jscf=crmdata.clone();
			let crodwfundingshare=json_get_value(jscf,"crodwfundingshares".as_bytes().to_vec());
			let crodwfundingshare_slice=crodwfundingshare.as_slice();
            let crodwfundingshare_str=match str::from_utf8(&crodwfundingshare_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let crodwfundingsharevalue:u64 = match u64::from_str(crodwfundingshare_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(crodwfundingsharevalue <= 100, Error::<T>::InvalidCrowdFundingshares); //check Crowd Funding Shares that must be <=100
			// check that the total shares are = 100 
			let totalshares=mastersharevalue+compositionsharevalue+othercontractssharevalue+crodwfundingsharevalue;
			ensure!(totalshares == 100, Error::<T>::InvalidTotalShares); //check total shares that must be 100

			// check validity of master data
			let masterclone=master.clone();
			// check for a valid json
			ensure!(json_check_validity(masterclone),Error::<T>::InvalidJson);
			let mut x=0;
			let mut totpercentage:u32 = 0;
			// check validity of records for Master Data
			loop {
				let jr=json_get_recordvalue(master.clone(),x);
				if jr.len()==0 {
					break;
				}
				// check for nickname
				let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
				ensure!(nickname.len() >0, Error::<T>::MissingMasterNickname); 
				// check for account address
				let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
				ensure!(account.len() >0, Error::<T>::MissingMasterAccount);
				// check for percentage
				let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
				ensure!(percentage.len() >0, Error::<T>::MissingMasterPercentage);
				// convert percentage from vec to u32
				let percentage_slice=percentage.as_slice();
            	let percentage_str=match str::from_utf8(&percentage_slice){
                	Ok(f) => f,
                	Err(_) => "0"
            	};
            	let percentagevalue:u32 = match u32::from_str(percentage_str){
                	Ok(f) => f,
                	Err(_) => 0,
            	};
				// sum percentage to totpercentage
				totpercentage=totpercentage+percentagevalue;
				x=x+1;
			}
			// check the total percentage is = 100
			ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageMaster); 

			// check validity of composition data
			let compositionclone=composition.clone();
			// check for a valid json
			ensure!(json_check_validity(compositionclone),Error::<T>::InvalidJson);
			x=0;
			totpercentage= 0;
			// check validity of records for Composition Data
			loop {
				let jr=json_get_recordvalue(composition.clone(),x);
				if jr.len()==0 {
					break;
				}
				// check for nickname
				let nickname=json_get_value(jr.clone(),"nickname".as_bytes().to_vec());
				ensure!(nickname.len() >0, Error::<T>::MissingCompositionNickname); 
				// check for account address
				let account=json_get_value(jr.clone(),"account".as_bytes().to_vec());
				ensure!(account.len() >0, Error::<T>::MissingCompositionAccount);
				// check for percentage
				let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
				ensure!(percentage.len() >0, Error::<T>::MissingCompositionPercentage);
				// convert percentage from vec to u32
				let percentage_slice=percentage.as_slice();
            	let percentage_str=match str::from_utf8(&percentage_slice){
                	Ok(f) => f,
                	Err(_) => "0"
            	};
            	let percentagevalue:u32 = match u32::from_str(percentage_str){
                	Ok(f) => f,
                	Err(_) => 0,
            	};
				// sum percentage to totpercentage
				totpercentage=totpercentage+percentagevalue;
				x=x+1;
			}
			// check the total percentage is = 100
			ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageComposition); 


			// Other contracts are optional we check the validity if there is a value only
			if othercontracts.len()>0 {
				// check validity of othercontracts data
				let othercontractsclone=othercontracts.clone();
				// check for a valid json
				ensure!(json_check_validity(othercontractsclone),Error::<T>::InvalidJson);
				x=0;
				totpercentage= 0;
				// check validity of records for other contracts data
				loop {
					let jr=json_get_recordvalue(othercontracts.clone(),x);
					if jr.len()==0 {
						break;
					}
					// check for nickname
					let id=json_get_value(jr.clone(),"id".as_bytes().to_vec());
					ensure!(id.len() >0, Error::<T>::MissingOtherContractsId); 
					// check for percentage
					let percentage=json_get_value(jr.clone(),"percentage".as_bytes().to_vec());
					ensure!(percentage.len() >0, Error::<T>::MissingCompositionPercentage);
					// convert percentage from vec to u32
					let percentage_slice=percentage.as_slice();
            		let percentage_str=match str::from_utf8(&percentage_slice){
                		Ok(f) => f,
                		Err(_) => "0"
            		};
            		let percentagevalue:u32 = match u32::from_str(percentage_str){
                		Ok(f) => f,
                		Err(_) => 0,
            		};
					// sum percentage to totpercentage
					totpercentage=totpercentage+percentagevalue;
					x=x+1;
				}
				// check the total percentage is = 100
				ensure!(totpercentage == 100, Error::<T>::WrongTotalPercentageOtherContracts); 
			}

			//****************************************
			// STORING DATA 
			//****************************************
			// Write storage for crmdata
			let crmstorage=crmdata.clone();
			let crmidstorage=crmid.clone();
			CrmData::insert(&crmidstorage, crmstorage);
			// Write the storage for master data
			let masterstorage=master.clone();
			let masteridstorage=crmid.clone();
			CrmMasterData::insert(masteridstorage, masterstorage);
			// Write the storage for Composition data
			let compositionstorage=composition.clone();
			let compositionidstorage=crmid.clone();
			CrmCompositionData::insert(compositionidstorage, compositionstorage);
			// write the storage for Other Contracts data (optional)
			if othercontracts.len()>0 {
				// Update storage for Other Contracts data
				let othercontractsstorage=othercontracts.clone();
				let othercontractsidstorage=crmid.clone();
				CrmOtherContractsData::insert(othercontractsidstorage, othercontractsstorage);
			}
			// Emit an event
			Self::deposit_event(RawEvent::CrmAdded(sender,crmid));
			// Return a successful DispatchResult
			Ok(())
		}
		/// Submit a change proposal that must be approved by the quorum 
		#[weight = 50_000]
		pub fn change_proposal_crmdata(origin, crmid: Vec<u8>, crmdata: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic is signed and get the signer.
			let sender = ensure_signed(origin)?;
			// check contractid
			ensure!(crmid.len() > 1, Error::<T>::ContractIdTooShort); //check minimum length
			// check that at the least some data to change has been received and it's not too long
			ensure!(crmdata.len()>0, Error::<T>::MissingContractData); 
			ensure!(crmdata.len()<1024, Error::<T>::CrmDataTooLong); 
			//ensure!(<CrmData<T>>::contains_key(&sender_str,crmid)==true, Error::<T>::ContractIdNotFound);
			// check the sender is part of the data submitted
			// store the proposal
			// Emit an event
			Self::deposit_event(RawEvent::CrmDataChangeProposal(sender,crmid));
			Ok(())
		}

	}
}
// function to validate a json string for no/std. It does not allocate of memory
fn json_check_validity(j:Vec<u8>) -> bool{	
    // minimum lenght of 2
    if j.len()<2 {
        return false;
    }
    // checks star/end with {}
    if *j.get(0).unwrap()==b'{' && *j.get(j.len()-1).unwrap()!=b'}' {
        return false;
    }
    // checks start/end with []
    if *j.get(0).unwrap()==b'[' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    // check that the start is { or [
    if *j.get(0).unwrap()!=b'{' && *j.get(0).unwrap()!=b'[' {
            return false;
    }
    //checks that end is } or ]
    if *j.get(j.len()-1).unwrap()!=b'}' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    //checks " opening/closing and : as separator between name and values
    let mut s:bool=true;
    let mut d:bool=true;
    let mut pg:bool=true;
    let mut ps:bool=true;
    let mut bp = b' ';
    for b in j {
        if b==b'[' && s {
            ps=false;
        }
        if b==b']' && s && ps==false {
            ps=true;
        }
        else if b==b']' && s && ps==true {
            ps=false;
        }
        if b==b'{' && s {
            pg=false;
        }
        if b==b'}' && s && pg==false {
            pg=true;
        }
        else if b==b'}' && s && pg==true {
            pg=false;
        }
        if b == b'"' && s && bp != b'\\' {
            s=false;
            bp=b;
            d=false;
            continue;
        }
        if b == b':' && s {
            d=true;
            bp=b;
            continue;
        }
        if b == b'"' && !s && bp != b'\\' {
            s=true;
            bp=b;
            d=true;
            continue;
        }
        bp=b;
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
    return true;
}
// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
fn json_get_recordvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
    let mut result=Vec::new();
    let mut op=true;
    let mut cn=0;
    let mut lb=b' ';
    for b in ar {
        if b==b',' && op==true {
            cn=cn+1;
            continue;
        }
        if b==b'[' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b']' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b'{' && op==true && lb!=b'\\' { 
            op=false;
        }
        if b==b'}' && op==false && lb!=b'\\' {
            op=true;
        }
        // field found
        if cn==p {
            result.push(b);
        }
        lb=b.clone();
    }
    return result;
}

// function to get value of a field for Substrate runtime (no std library and no variable allocation)
fn json_get_value(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
    let mut result=Vec::new();
    let mut k=Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl{
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in  0..jl {
        let mut m=0;
        let mut xx=0;
        if x+kl>jl {
            break;
        }
        for i in x..x+kl {
            if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                m=m+1;
            }
            xx=xx+1;
        }
        if m==kl{
            let mut lb=b' ';
            let mut op=true;
            let mut os=true;
            for i in x+kl..jl-1 {
                if *j.get(i).unwrap()==b'[' && op==true && os==true{
                    os=false;
                }
                if *j.get(i).unwrap()==b'}' && op==true && os==false{
                    os=true;
                }
                if *j.get(i).unwrap()==b':' && op==true{
                    continue;
                }
                if *j.get(i).unwrap()==b'"' && op==true && lb!=b'\\' {
                    op=false;
                    continue
                }
                if *j.get(i).unwrap()==b'"' && op==false && lb!=b'\\' {
                    break;
                }
                if *j.get(i).unwrap()==b'}' && op==true{
                    break;
                }
                if *j.get(i).unwrap()==b',' && op==true && os==true{
                    break;
                }
                result.push(j.get(i).unwrap().clone());
                lb=j.get(i).unwrap().clone();
            }   
            break;
        }
    }
    return result;
}
