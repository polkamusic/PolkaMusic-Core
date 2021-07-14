Web3 Foundation Grant Milestone-1 is available on our testnet

Node: testnet.polkamusic.io

Explorer: https://ipfs.io/ipns/dotapps.io/?rpc=wss%3A%2F%2Ftestnet.polkamusic.io#/explorer

Rights Management Portal: https://rights.polkamusic.io

New Contracts: In order to create a new contract, use the newContract method under the crm extrinsic, accessible through the explorer. 

Below is the test data:

***** Contract #1

crmid: 1  (must be unique)

crmdata: {"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}

master: {"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":50}]}

composition: {"composition": [{"nickname": "Charlie","account": "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname": "Dave","account": "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}

other contracts: {}

******* Change Proposal of Main data for contract #1

{"crmid":1,"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":60,"masterquorum":51,"compositionshare":40,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}

******* Change Proposal of Master data for contract #1
{"crmid":1,"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":40},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":60}]}

****** Change proposal for Other Contracts:
{"crmid":1,"othercontracts": [{"id":1,"percentage":70},{"id":2,"percentage":30}]}



***** Contract #2

2

{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":50,"compositionquorum":51,"othercontractsshare":0,"othercontractsquorum":51}

{"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":50}]}

{"composition": [{"nickname": "Charlie","account": "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname": "Dave","account": "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}

{}



***** Contract #3

3

{"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":100,"mastershare":50,"masterquorum":51,"compositionshare":40,"compositionquorum":51,"othercontractsshare":10,"othercontractsquorum":51}

{"master": [{"nickname": "Bob","account": "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","percentage":50},{"nickname": "Bob Stash","account": "0xfe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e", "percentage":50}]}

{"composition": [{"nickname": "Charlie","account": "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","percentage":50},{"nickname": "Dave","account": "0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","percentage":50}]}

{"othercontracts": [{"id":1,"percentage":50},{"id":2,"percentage":50}]}


**** Change proposal #3

{"crmid":3,"ipfshash":"0E7071C59DF3B9454D1D18A15270AA36D54F89606A576DC621757AFD44AD1D2E","ipfshashprivate": "B45165ED3CD437B9FFAD02A2AAD22A4DDC69162470E2622982889CE5826F6E3D","globalquorum":75,"mastershare":50,"masterquorum":51,"compositionshare":40,"compositionquorum":51,"othercontractsshare":10,"othercontractsquorum":51}




