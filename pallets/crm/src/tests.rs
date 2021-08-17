use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
#[test]
fn testing_default_values() {
    new_test_ext().execute_with(|| {
        // Contract creation and reading data back
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_ok!(CrmModule::new_contract(Origin::signed(1),1,crmdata.clone(),crmmaster.clone(),crmcomposition.clone(),crmothercontracts.clone()));
        // read contract data
        assert_eq!(CrmModule::get_crmdata(1), Some(crmdata));
        assert_eq!(CrmModule::get_master(1), Some(crmmaster));
        assert_eq!(CrmModule::get_composition(1), Some(crmcomposition));
        // create change proposal for Crm Data
        let cdp=r#"{"crmid":1,"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":75,"mastershare":60,"masterquorum":51,"compositionshare":40,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdatachangeproposal=cdp.as_bytes().to_vec();
        assert_ok!(CrmModule::change_proposal_crmdata(Origin::signed(1),1,crmdatachangeproposal.clone()));
        // read change proposal for crm data
        assert_eq!(CrmModule::get_crmdata_change_proposal(1), Some(crmdatachangeproposal));
        // vote change proposal for crm data should deny the access because no Rights to vote
        assert_noop!(CrmModule::vote_proposal_crmdata(Origin::signed(1),1,true),Error::<Test>::SignerHasNoRightsForVoting);
        // create change proposal for Crm Master Data
        let cmp=r#"{"crmid":1,"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":40},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":60}]}"#;
        let crmmasterchangeproposal=cmp.as_bytes().to_vec();
        assert_ok!(CrmModule::change_proposal_crm_masterdata(Origin::signed(1),1,crmmasterchangeproposal.clone()));
        // read change proposal for Crm Master Data
        assert_eq!(CrmModule::get_crm_masterdata_change_proposal(1), Some(crmmasterchangeproposal));
        // vote change proposal for crm Master should deny the access because no Rights to vote
        assert_noop!(CrmModule::vote_proposal_crm_masterdata(Origin::signed(1),1,true),Error::<Test>::SignerHasNoRightsForVoting);
       // create change proposal for Crm Composition Data
       let cmc=r#"{"crmid":1,"composition":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":40},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":60}]}"#;
       let crmcompositionchangeproposal=cmc.as_bytes().to_vec();
       assert_ok!(CrmModule::change_proposal_crm_compositiondata(Origin::signed(1),1,crmcompositionchangeproposal.clone()));
       // read change proposal for Crm Composition Data
       assert_eq!(CrmModule::get_crm_compositiondata_change_proposal(1), Some(crmcompositionchangeproposal));
       // vote change proposal for crm Composition should deny the access because no Rights to vote
       assert_noop!(CrmModule::vote_proposal_crm_compositiondata(Origin::signed(1),1,true),Error::<Test>::SignerHasNoRightsForVoting);
       // creating of a new contract #2 (required for the test on Other Contacrat change proposal)
       let cdb=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
       let crmdatab=cdb.as_bytes().to_vec();
       let cmb=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
       let crmmasterb=cmb.as_bytes().to_vec();
       let ccb=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
       let crmcompositionb=ccb.as_bytes().to_vec();
       let cob=r#"{}"#;
       let crmothercontractsb=cob.as_bytes().to_vec();
       assert_ok!(CrmModule::new_contract(Origin::signed(1),2,crmdatab.clone(),crmmasterb.clone(),crmcompositionb.clone(),crmothercontractsb.clone()));
       // create change proposal for Crm Other Contracts Data
       let cmo=r#"{"crmid":1,"othercontracts": [{"id":1,"percentage":70},{"id":2,"percentage":30}]}"#;
       let crmothercontractschangeproposal=cmo.as_bytes().to_vec();
       assert_ok!(CrmModule::change_proposal_crm_othercontractsdata(Origin::signed(1),1,crmothercontractschangeproposal.clone()));
       // read change proposal for Crm Composition Data
       assert_eq!(CrmModule::get_crm_othercontractsdata_change_proposal(1), Some(crmothercontractschangeproposal));
       // vote change proposal for crm Other Contracts data should deny the access because no Rights to vote
       assert_noop!(CrmModule::vote_proposal_crm_othercontractsdata(Origin::signed(1),1,true),Error::<Test>::SignerHasNoRightsForVoting);
    });
}

#[test]
fn correct_error_for_wrong_values() {
    new_test_ext().execute_with(|| {

        // Submit wrong json for crm data
        let cd=r#"{"ipfshash:"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::InvalidJsonCrmData);
        // Submit wrong json for crm master data
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#""master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::InvalidJsonCrmMaster);
        // Submit wrong json for crm composition data
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::InvalidJsonCrmComposition);

        // Submit wrong shares value in main data
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":40,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::InvalidTotalShares);
        // Submit wrong shares value in master data
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":30},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::WrongTotalPercentageMaster);

        // Submit wrong shares value in composition data
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":40},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::WrongTotalPercentageComposition);

        // Submit wrong ipfshash
        let cd=r#"{"ipfshash":"0E7071C59DF3B9454","ipfshashprivate":"B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}"#;
        let crmdata=cd.as_bytes().to_vec();
        let cm=r#"{"master":[{"nickname":"Bob","account":"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname":"Bob Stash","account":"0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","percentage":50}]}"#;
        let crmmaster=cm.as_bytes().to_vec();
        let cc=r#"{"composition":[{"nickname":"Charlie","account":"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname":"Dave","account":"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}"#;
        let crmcomposition=cc.as_bytes().to_vec();
        let co=r#"{}"#;
        let crmothercontracts=co.as_bytes().to_vec();
        assert_noop!(CrmModule::new_contract(Origin::signed(1),1,crmdata,crmmaster,crmcomposition,crmothercontracts),Error::<Test>::InvalidIpfsHash);
    });
}
