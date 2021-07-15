// Import the API
const { ApiPromise, WsProvider } = require('@polkadot/api');
const mysql = require('mysql');
let moment = require('moment');

async function main () {
    // shows banner
    console.log("Polkamusic Cache Engine - v. 1.00");
    // connect local database
    let connection = mysql.createConnection({
        host     : '127.0.0.1',
        user     : 'polkamusic',
        password : 'aszxqw1234'         // change to your current password, this is only a sample
    });
    connection.connect(function(err) {
        if (err) {
            console.error('[Error] error connecting to the database: ' + err.stack);
            return;
        }
        console.log("[Info] Database connected");
        // create database if not already done
        create_database(connection);
    });
    
    // connect to local node
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');  
    const api = await ApiPromise.create({ provider: wsProvider,"types" :{
        "Voting": {
            "changeid": "u32",
            "crmid": "u32",
            "quorum": "u32",
            "nrvotesyes": "u32",
            "nrvotesno": "u32",
            "percvotesyes": "u32",
            "percvotesno":"u32"
            }
        }
    });
    console.log("[INFO] Connected to Polkamusic Node with genesis: ",api.genesisHash.toHex());
    // Subscribe to system events via storage
    api.query.system.events((events) => {
        console.log(`\n[INFO] Received ${events.length} events:`);
        // Loop through the Vec<EventRecord>
        events.forEach((record) => {
            // Extract the phase, event and the event types
            let eventv;
            let phasev;
            let typesv;
            try{
                const { event, phase } = record;
                const types = event.typeDef;
                typesv=types;
                eventv=event;
                phasev=phase;
            }
            catch(err){
                console.log("Data not decodable: ",err);
                return;
            }

            // Show what we have received for debugging
            console.log(`[DEBUG] Event: \t${eventv.section}:${eventv.method}:: (phase=${phasev.toString()})`);
            console.log(`\t\t${eventv.meta.documentation.toString()}`);
            // new contract
            if (eventv.section=="crm" && eventv.method=="CrmAdded"){
                let contractid=eventv.data[1].toString();
                console.log("[INFO] Adding new contract",contractid);
                add_new_contract(connection,api,contractid)
            }
            // Change proposal for Crm Data
            if (eventv.section=="crm" && eventv.method=="CrmDataNewChangeProposal"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new change proposal for crm data",contractid," Change id: ",changeid);
                add_change_proposal_crmdata(connection,api,contractid,changeid);
            }
            // Vote for change Crm Data
            if (eventv.section=="crm" && eventv.method=="CrmDataChangeVote"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new vote for contract id: ",contractid," Change id (Main Data): ",changeid);
                add_vote_change_crmdata(connection,api,contractid,changeid);
            }
            // Changed CRM Data
            if (eventv.section=="crm" && eventv.method=="CrmDataChanged"){
                let contractid=eventv.data[1].toString();
                console.log("[INFO] Contract data has changed: ",contractid);
                change_crmdata(connection,api,contractid);
            }
            // Change proposal for Crm MasterData
            if (eventv.section=="crm" && eventv.method=="CrmMasterDataNewChangeProposal"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new change proposal for crm master data",contractid," Change id: ",changeid);
                add_change_proposal_crmmasterdata(connection,api,contractid,changeid);
            }            
            // Vote for change Crm Master Data
            if (eventv.section=="crm" && eventv.method=="CrmMasterDataChangeVote"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new vote for contract id: ",contractid," Change id (Master Data): ",changeid);
                add_vote_change_crmmasterdata(connection,api,contractid,changeid);
            }
            // Changed CRM Master Data
            if (eventv.section=="crm" && eventv.method=="CrmMasterDataChanged"){
                let contractid=eventv.data[1].toString();
                console.log("[INFO] Contract master data has changed: ",contractid);
                change_crmmasterdata(connection,api,contractid);
            }
            // Change proposal for Crm Composition Data
            if (eventv.section=="crm" && eventv.method=="CrmCompositionDataNewChangeProposal"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new change proposal for crm composition data",contractid," Change id: ",changeid);
                add_change_proposal_crmcompositiondata(connection,api,contractid,changeid);
            }            
            // Vote for change Crm Composition Data
            if (eventv.section=="crm" && eventv.method=="CrmCompositionDataChangeVote"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new vote for contract id: ",contractid," Change id (Composition Data): ",changeid);
                add_vote_change_crmcompositiondata(connection,api,contractid,changeid);
            }
            // Changed CRM Composition Data
            if (eventv.section=="crm" && eventv.method=="CrmCompositionDataChanged"){
                let contractid=eventv.data[1].toString();
                console.log("[INFO] Contract composition data has changed: ",contractid);
                change_crmcompositiondata(connection,api,contractid);
            }
            // Change proposal for Crm Other Contracts Data
            if (eventv.section=="crm" && eventv.method=="CrmOtherContractsDataNewChangeProposal"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new change proposal for crm other contracts data",contractid," Change id: ",changeid);
                add_change_proposal_crmothercontractsdata(connection,api,contractid,changeid);
            }            
            // Vote for change Crm Other Contracts Data
            if (eventv.section=="crm" && eventv.method=="CrmOthercontractsDataChangeVote"){
                let contractid=eventv.data[1].toString();
                let changeid=eventv.data[2].toString();
                console.log("[INFO] Adding new vote for contract id: ",contractid," Change id (Other contracts Data): ",changeid);
                add_vote_change_crmothercontractsdata(connection,api,contractid,changeid);
            }
            // Changed CRM Other Contracts Data
            if (eventv.section=="crm" && eventv.method=="CrmOtherContractsDataChanged"){
                let contractid=eventv.data[1].toString();
                console.log("[INFO] Contract other contracts data has changed: ",contractid);
                change_crmothercontractsdata(connection,api,contractid);
            }
            // Loop through each of the parameters, displaying the type and data
            eventv.data.forEach((data, index) => {
            console.log(`\t\t\t${typesv[index].type}: ${data.toString()}`);
            });
        });
    });
}

main().catch((error) => {
console.log("[INFO] Dropping empty/null data")
console.error(error);
});
// function to add change proposal for  crm compisition data
async function add_change_proposal_crmcompositiondata(connection,api,contractid,changeid){
    //query change proposal data
    const crmcompositiondata = await api.query.crm.crmCompositionDataChangeProposal(changeid);
    const crmcd = Buffer.from(crmcompositiondata.toString().substr(2), 'hex');
    const jmd=JSON.parse(crmcd);
    // write record
    jmd.composition.forEach(element => {
        let sqlquery="insert into polkamusic.crmcompositiondatachangeproposal set changeid=?,contractid=?,nickname=?,account=?,percentage=?";
        connection.query(
            {
                sql: sqlquery,
                values: [changeid,contractid,element.nickname,element.account,element.percentage]
            },
            function (error) {
                if (error){
                    if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                        throw error;
                }
            }
        );
    });
    //query voting results for change proposal
    const crmdatav = await api.query.crm.crmCompositionDataChangeVotingResult(changeid);
    let jdv=crmdatav.unwrap();
    // write record
    let sqlquery1="insert into polkamusic.crmcompositiondatachangevotingresult set changeid=?,contractid=?,quorum=?,nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=?";
    connection.query(
        {
            sql: sqlquery1,
            values: [changeid,contractid,jdv.quorum.toNumber(),jdv.nrvotesyes.toNumber(),jdv.nrvotesno.toNumber(),jdv.percvotesyes.toNumber(),jdv.percvotesno.toNumber()]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
    
}
// function to add vote for the change proposal of crm composiiton data
async function add_vote_change_crmcompositiondata(connection,api,contractid,changeid){
    //query voting results for change proposal
    const crmdata = await api.query.crm.crmCompositionDataChangeVotingResult(changeid);
    let jd=crmdata.unwrap();
    // write record
    let sqlquery="update polkamusic.crmcompositiondatachangevotingresult set nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=? where changeid=? and contractid=?";
    connection.query(
        {
            sql: sqlquery,
            values: [jd.nrvotesyes.toNumber(),jd.nrvotesno.toNumber(),jd.percvotesyes.toNumber(),jd.percvotesno.toNumber(),changeid,contractid]
        },
        function (error) {
            if (error){
                    throw error;
            }
        }
    );
}
// function to change crm master data
async function change_crmcompositiondata(connection,api,contractid){
    // delete previous records
    let sqlqueryd="delete from polkamusic.crmcompositiondata where contractid=?";
    connection.query(
        {
            sql: sqlqueryd,
            values: [contractid]
        },
        async function (error) {
            if (error){
                throw error;    
            }else {
                //query change proposal data
                const crmdata = await api.query.crm.crmMasterData(contractid);
                const crmd = Buffer.from(crmdata.toString().substr(2), 'hex');
                const jmd=JSON.parse(crmd);
                // write new records
                jmd.master.forEach(element => {
                    let sqlquery="insert into polkamusic.crmcompositiondata set contractid=?,nickname=?,account=?,percentage=?";
                    connection.query(
                        {
                            sql: sqlquery,
                            values: [contractid,element.nickname,element.account,element.percentage]
                        },
                        function (error) {
                            if (error){
                                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                                    throw error;
                            }
                        }
                    );
                });
            }
        }
    );
}
// function to add change proposal for  crm other contracts data
async function add_change_proposal_crmothercontractsdata(connection,api,contractid,changeid){
    //query change proposal data
    const crmdata = await api.query.crm.crmOtherContractsDataChangeProposal(changeid);
    const crmcd = Buffer.from(crmdata.toString().substr(2), 'hex');
    const jmd=JSON.parse(crmcd);
    // write record
    jmd.othercontracts.forEach(element => {
        let sqlquery="insert into polkamusic.crmothercontractsdatachangeproposal set changeid=?,contractid=?,othercontractid=?,percentage=?";
        connection.query(
            {
                sql: sqlquery,
                values: [changeid,contractid,element.id,element.percentage]
            },
            function (error) {
                if (error){
                    if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                        throw error;
                }
            }
        );
    });
    //query voting results for change proposal
    const crmdatav = await api.query.crm.crmOtherContractsDataChangeVotingResult(changeid);
    let jdv=crmdatav.unwrap();
    // write record
    let sqlquery1="insert into polkamusic.crmothercontractsdatachangevotingresult set changeid=?,contractid=?,quorum=?,nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=?";
    connection.query(
        {
            sql: sqlquery1,
            values: [changeid,contractid,jdv.quorum.toNumber(),jdv.nrvotesyes.toNumber(),jdv.nrvotesno.toNumber(),jdv.percvotesyes.toNumber(),jdv.percvotesno.toNumber()]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
    
}
// function to add vote for the change proposal of crm composiiton data
async function add_vote_change_crmothercontractsdata(connection,api,contractid,changeid){
    //query voting results for change proposal
    const crmdata = await api.query.crm.crmOtherContractsDataChangeVotingResult(changeid);
    let jd=crmdata.unwrap();
    // write record
    let sqlquery="update polkamusic.crmothercontractsdatachangevotingresult set nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=? where changeid=? and contractid=?";
    connection.query(
        {
            sql: sqlquery,
            values: [jd.nrvotesyes.toNumber(),jd.nrvotesno.toNumber(),jd.percvotesyes.toNumber(),jd.percvotesno.toNumber(),changeid,contractid]
        },
        function (error) {
            if (error){
                    throw error;
            }
        }
    );
}
// function to change crm other contractts data
async function change_crmothercontractsdata(connection,api,contractid){
    // delete previous records
    let sqlqueryd="delete from polkamusic.crmothercontractsdata where contractid=?";
    connection.query(
        {
            sql: sqlqueryd,
            values: [contractid]
        },
        async function (error) {
            if (error){
                throw error;    
            }else {
                //query change proposal data
                const crmdata = await api.query.crm.crmOtherContractsData(contractid);
                const crmd = Buffer.from(crmdata.toString().substr(2), 'hex');
                const jmd=JSON.parse(crmd);
                // write new records
                jmd.othercontracts.forEach(element => {
                    let sqlquery="insert into polkamusic.crmothercontractsdata set contractid=?,othercontractid=?,percentage=?";
                    connection.query(
                        {
                            sql: sqlquery,
                            values: [contractid,element.id,element.percentage]
                        },
                        function (error) {
                            if (error){
                                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                                    throw error;
                            }
                        }
                    );
                });
            }
        }
    );
}
// function to add change proposal for crm master data
async function add_change_proposal_crmmasterdata(connection,api,contractid,changeid){
    //query change proposal data
    const crmmasterdata = await api.query.crm.crmMasterDataChangeProposal(changeid);
    const crmd = Buffer.from(crmmasterdata.toString().substr(2), 'hex');
    const jmd=JSON.parse(crmd);
    // write record
    jmd.master.forEach(element => {
        connection.query(
            {
                sql: "insert into polkamusic.crmmasterdatachangeproposal set changeid=?,contractid=?,nickname=?,account=?,percentage=?",
                values: [changeid,contractid,element.nickname,element.account,element.percentage]
            },
            function (error) {
                if (error){
                    if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                        throw error;
                }
            }
        );
    });
    //query voting results for change proposal
    const crmdatav = await api.query.crm.crmMasterDataChangeVotingResult(changeid);
    let jdv=crmdatav.unwrap();
    // write record
    let sqlquery1="insert into polkamusic.crmmasterdatachangevotingresult set changeid=?,contractid=?,quorum=?,nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=?";
    connection.query(
        {
            sql: sqlquery1,
            values: [changeid,contractid,jdv.quorum.toNumber(),jdv.nrvotesyes.toNumber(),jdv.nrvotesno.toNumber(),jdv.percvotesyes.toNumber(),jdv.percvotesno.toNumber()]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
    
}

// function to add vote for the change proposal of crm data
async function add_vote_change_crmmasterdata(connection,api,contractid,changeid){
    //query voting results for change proposal
    const crmdata = await api.query.crm.crmMasterDataChangeVotingResult(changeid);
    let jd=crmdata.unwrap();
    // write record
    let sqlquery="update polkamusic.crmmasterdatachangevotingresult set nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=? where changeid=? and contractid=?";
    connection.query(
        {
            sql: sqlquery,
            values: [jd.nrvotesyes.toNumber(),jd.nrvotesno.toNumber(),jd.percvotesyes.toNumber(),jd.percvotesno.toNumber(),changeid,contractid]
        },
        function (error) {
            if (error){
                    throw error;
            }
        }
    );
}
// function to change crm master data
async function change_crmmasterdata(connection,api,contractid){
    // delete previous records
    let sqlqueryd="delete from polkamusic.crmmasterdata where contractid=?";
    connection.query(
        {
            sql: sqlqueryd,
            values: [contractid]
        },
        async function (error) {
            if (error){
                throw error;    
            }else {
                //query change proposal data
                const crmmasterdata = await api.query.crm.crmMasterData(contractid);
                const crmmd = Buffer.from(crmmasterdata.toString().substr(2), 'hex');
                const jmd=JSON.parse(crmmd);
                // write new records
                jmd.master.forEach(element => {
                    let sqlquery="insert into polkamusic.crmmasterdata set contractid=?,nickname=?,account=?,percentage=?";
                    connection.query(
                        {
                            sql: sqlquery,
                            values: [contractid,element.nickname,element.account,element.percentage]
                        },
                        function (error) {
                            if (error){
                                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                                    throw error;
                            }
                        }
                    );
                });
            }
        }
    );
}
// function to change crm data
async function change_crmdata(connection,api,contractid){
    //query change proposal data
    const crmdata = await api.query.crm.crmData(contractid);
    const crmd = Buffer.from(crmdata.toString().substr(2), 'hex');
    const jd=JSON.parse(crmd);
    // updated record
    let sqlquery="update polkamusic.crmdata set ipfshash=?,ipfshashprivate=?,globalquorum=?,mastershare=?,masterquorum=?,compositionshare=?,compositionquorum=?,othercontractsshare=?,othercontractsquorum=? where id=?";
    connection.query(
        {
            sql: sqlquery,
            values: [jd.ipfshash,jd.ipfshashprivate,jd.globalquorum,jd.mastershare,jd.masterquorum,jd.compositionshare,jd.compositionquorum,jd.othercontractsshare,jd.othercontractsquorum,contractid]
        },
        function (error) {
            if (error){
                throw error;
            }
        }
    );
}
// function to add vote for the change proposal of crm data
async function add_vote_change_crmdata(connection,api,contractid,changeid){
    //query voting results for change proposal
    const crmdata = await api.query.crm.crmDataChangeVotingResult(changeid);
    let jd=crmdata.unwrap();
    // write record
    let sqlquery="update polkamusic.crmdatachangevotingresult set nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=? where changeid=? and contractid=?";
    connection.query(
        {
            sql: sqlquery,
            values: [jd.nrvotesyes.toNumber(),jd.nrvotesno.toNumber(),jd.percvotesyes.toNumber(),jd.percvotesno.toNumber(),changeid,contractid]
        },
        function (error) {
            if (error){
                    throw error;
            }
        }
    );
}
// function to add change proposal for main crm data
async function add_change_proposal_crmdata(connection,api,contractid,changeid){
    //query change proposal data
    const crmdata = await api.query.crm.crmDataChangeProposal(contractid);
    const crmd = Buffer.from(crmdata.toString().substr(2), 'hex');
    const jd=JSON.parse(crmd);
    // write record
    let sqlquery="insert into polkamusic.crmdatachangeproposal set id=?,contractid=?,ipfshash=?,ipfshashprivate=?,globalquorum=?,mastershare=?,masterquorum=?,compositionshare=?,compositionquorum=?,othercontractsshare=?,othercontractsquorum=?";
    connection.query(
        {
            sql: sqlquery,
            values: [changeid,contractid,jd.ipfshash,jd.ipfshashprivate,jd.globalquorum,jd.mastershare,jd.masterquorum,jd.compositionshare,jd.compositionquorum,jd.othercontractsshare,jd.othercontractsquorum]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
    //query voting results for change proposal
    const crmdatav = await api.query.crm.crmDataChangeVotingResult(changeid);
    let jdv=crmdatav.unwrap();
    // write record
    let sqlquery1="insert into polkamusic.crmdatachangevotingresult set changeid=?,contractid=?,quorum=?,nrvotesyes=?,nrvotesno=?,percvotesyes=?,percvotesno=?";
    connection.query(
        {
            sql: sqlquery1,
            values: [changeid,contractid,jdv.quorum.toNumber(),jdv.nrvotesyes.toNumber(),jdv.nrvotesno.toNumber(),jdv.percvotesyes.toNumber(),jdv.percvotesno.toNumber()]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
}

// function to add new contract
async function add_new_contract(connection,api,contractid){
    //query main data
    const crmdata = await api.query.crm.crmData(contractid);
    const crmd = Buffer.from(crmdata.toString().substr(2), 'hex');
    const jd=JSON.parse(crmd);
    // write record
    let sqlquery="insert into polkamusic.crmdata set id=?,ipfshash=?,ipfshashprivate=?,globalquorum=?,mastershare=?,masterquorum=?,compositionshare=?,compositionquorum=?,othercontractsshare=?,othercontractsquorum=?";
    connection.query(
        {
            sql: sqlquery,
            values: [contractid,jd.ipfshash,jd.ipfshashprivate,jd.globalquorum,jd.mastershare,jd.masterquorum,jd.compositionshare,jd.compositionquorum,jd.othercontractsshare,jd.othercontractsquorum]
        },
        function (error) {
            if (error){
                if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                    throw error;
            }
        }
    );
    //query master data
    const crmmasterdata = await api.query.crm.crmMasterData(contractid);
    const crmmd = Buffer.from(crmmasterdata.toString().substr(2), 'hex');
    const jmd=JSON.parse(crmmd);
    // write record
    jmd.master.forEach(element => {
        let sqlquery="insert into polkamusic.crmmasterdata set contractid=?,nickname=?,account=?,percentage=?";
        connection.query(
            {
                sql: sqlquery,
                values: [contractid,element.nickname,element.account,element.percentage]
            },
            function (error) {
                if (error){
                    if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                        throw error;
                }
            }
        );
    });
    //query composition data
    const crmcompositiondata = await api.query.crm.crmCompositionData(contractid);
    const crmcd = Buffer.from(crmcompositiondata.toString().substr(2), 'hex');
    const jcd=JSON.parse(crmcd);
    // write record
    jcd.composition.forEach(element => {
        let sqlquery="insert into polkamusic.crmcompositiondata set contractid=?,nickname=?,account=?,percentage=?";
        connection.query(
            {
                sql: sqlquery,
                values: [contractid,element.nickname,element.account,element.percentage]
            },
            function (error) {
                if (error){
                    if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                        throw error;
                }
            }
        );
    });
    //query other contracts data
    const crmodata = await api.query.crm.crmOtherContractsData(contractid);
    const crmod = Buffer.from(crmodata.toString().substr(2), 'hex');
    const jmdo=JSON.parse(crmod);
    // write record
    if (typeof jmdo.othercontracts !== 'undefined') {
        jmdo.othercontracts.forEach(element => {
            if(element.id!==undefined){            // other contracts data may be empty
                let sqlquery="insert into polkamusic.crmothercontractsdata set contractid=?,othercontractid=?,percentage=?";
                connection.query(
                    {
                        sql: sqlquery,
                        values: [contractid,element.id,element.percentage]
                    },
                    function (error) {
                        if (error){
                            if (error.errno!=1062)      //duplicated record is ignored, because it happens in refreshing the cache
                                throw error;
                        }
                    }
                );
            }
        });    
        return;
    } else {
        return
    }
}

//function to create the empty datase and the required tables
async function create_database(connection){
    connection.query('CREATE DATABASE IF NOT EXISTS polkamusic', function (error, results, fields) {
        if (error)
            throw error;
    });
    connection.query('USE polkamusic',function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMDATA table
    let q=`CREATE TABLE IF NOT EXISTS crmdata(\n`+
                `id INT PRIMARY KEY UNIQUE,\n`+                         //contract id
                `ipfshash VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `ipfshashprivate VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `globalquorum INT DEFAULT 0 NOT NULL,\n`+
                `mastershare INT DEFAULT 0 NOT NULL,\n`+
                `masterquorum INT DEFAULT 0 NOT NULL,\n`+
                `compositionshare INT DEFAULT 0 NOT NULL,\n`+
                `compositionquorum INT DEFAULT 0 NOT NULL,\n`+
                `othercontractsshare INT DEFAULT 0 NOT NULL,\n`+
                `othercontractsquorum INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMMASTERDATA table
    let q1=`CREATE TABLE IF NOT EXISTS crmmasterdata(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+             
                `contractid INT default 0 NOT NULL,\n`+
                `nickname VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `account VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q1,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMCOMPOSITIONDATA table
    let q2=`CREATE TABLE IF NOT EXISTS crmcompositiondata(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+
                `contractid INT default 0 NOT NULL,\n`+
                `nickname VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `account VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q2,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMOTHERCONTRACTSDATA table
    let q3=`CREATE TABLE IF NOT EXISTS crmothercontractsdata(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+
                `contractid INT default 0 NOT NULL,\n`+         // contract id 
                `othercontractid INT default 0 NOT NULL,\n`+    // other contract id having rights on this contract
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q3,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMDATACHANGEPROPOSAL table
    let q4=`CREATE TABLE IF NOT EXISTS crmdatachangeproposal(\n`+
                `id INT PRIMARY KEY UNIQUE,\n`+                     //change proposal unique id
                `contractid INT default 0 NOT NULL,\n`+             //contract id to be changed
                `ipfshash VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `ipfshashprivate VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `globalquorum INT DEFAULT 0 NOT NULL,\n`+
                `mastershare INT DEFAULT 0 NOT NULL,\n`+
                `masterquorum INT DEFAULT 0 NOT NULL,\n`+
                `compositionshare INT DEFAULT 0 NOT NULL,\n`+
                `compositionquorum INT DEFAULT 0 NOT NULL,\n`+
                `othercontractsshare INT DEFAULT 0 NOT NULL,\n`+
                `othercontractsquorum INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q4,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMDATACHANGEVOTINGRESULT table
    let q5=`CREATE TABLE IF NOT EXISTS crmdatachangevotingresult(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+             //voting results unique id
                `changeid INT default 0 NOT NULL,\n`+               //change proposal id
                `contractid INT default 0 NOT NULL,\n`+             //contract id to be changed
                `quorum INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesno INT DEFAULT 0 NOT NULL,\n`+
                `percvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `percvotesno INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q5,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMMASTERDATACHANGEPROPOSAL table
    let q6=`CREATE TABLE IF NOT EXISTS crmmasterdatachangeproposal(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+ //internal  unique id
                `changeid INT default 0 NOT NULL,\n`+   //change proposal unique id
                `contractid INT default 0 NOT NULL,\n`+
                `nickname VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `account VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q6,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMMASTERDATACHANGEVOTINGRESULT table
    let q7=`CREATE TABLE IF NOT EXISTS crmmasterdatachangevotingresult(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+             //voting results unique id
                `changeid INT default 0 NOT NULL,\n`+               //change proposal id
                `contractid INT default 0 NOT NULL,\n`+             //contract id to be changed
                `quorum INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesno INT DEFAULT 0 NOT NULL,\n`+
                `percvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `percvotesno INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q7,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMCOMPOSITIONDATACHANGEPROPOSAL table
    let q8=`CREATE TABLE IF NOT EXISTS crmcompositiondatachangeproposal(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+         //internal unique id
                `changeid INT default 0 NOT NULL,\n`+   //change proposal unique id
                `contractid INT default 0 NOT NULL,\n`+
                `nickname VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `account VARCHAR(128) DEFAULT '' NOT NULL,\n`+
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q8,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMCOMPOSITIONDATACHANGEVOTINGRESULT table
    let q9=`CREATE TABLE IF NOT EXISTS crmcompositiondatachangevotingresult(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+             //voting results unique id
                `changeid INT default 0 NOT NULL,\n`+               //change proposal id
                `contractid INT default 0 NOT NULL,\n`+             //contract id to be changed
                `quorum INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesno INT DEFAULT 0 NOT NULL,\n`+
                `percvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `percvotesno INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q9,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMOTHERCONTACTSDATACHANGEPROPOSAL table
    let q10=`CREATE TABLE IF NOT EXISTS crmothercontractsdatachangeproposal(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+         //internal unique id
                `changeid INT default 0 NOT NULL,\n`+   //change proposal unique id
                `contractid INT default 0 NOT NULL,\n`+
                `othercontractid INT default 0 NOT NULL,\n`+
                `percentage INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q10,function (error, results, fields) {
        if (error) throw error;
    });
    //creation of CRMOTHERCONTRACTSDATACHANGEVOTINGRESULT table
    let q11=`CREATE TABLE IF NOT EXISTS crmothercontractsdatachangevotingresult(\n`+
                `id INT AUTO_INCREMENT PRIMARY KEY,\n`+             //voting results unique id
                `changeid INT default 0 NOT NULL,\n`+               //change proposal id
                `contractid INT default 0 NOT NULL,\n`+             //contract id to be changed
                `quorum INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `nrvotesno INT DEFAULT 0 NOT NULL,\n`+
                `percvotesyes INT DEFAULT 0 NOT NULL,\n`+
                `percvotesno INT DEFAULT 0 NOT NULL\n`+
                `)`;
    connection.query(q11,function (error, results, fields) {
        if (error) throw error;
    });
}
