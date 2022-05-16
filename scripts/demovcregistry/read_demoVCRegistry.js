/**
 * @fileOverview CSPR JS SDK demo: VCRegistry - view contract balances.
 */

var _ = require('lodash');

const caspersdk = require('casper-js-sdk');
const { identity } = require('lodash');
const Keys = caspersdk.Keys;
const CasperClient = caspersdk.CasperClient;
const CasperServiceByJsonRPC = caspersdk.CasperServiceByJsonRPC;

const { CONTRACT_DEMOVCREGISTRY_NAME, 
        CONTRACT_DEMOVCREGISTRY_HASH,
        DEPLOY_NODE_ADDRESS,
        DEPLOY_CHAIN_NAME } = require("../constants");
const { CLValue } = require('casper-js-sdk');



const readVC = async(issuer, index) => {
    // Step 1: Set casper node client.
    const client = new CasperClient(DEPLOY_NODE_ADDRESS);
    const clientRpc = new CasperServiceByJsonRPC(DEPLOY_NODE_ADDRESS);
    
    // Step 3: Query node for global state root hash.
    const stateRootHash = await clientRpc.getStateRootHash();

    let vc_length_key = "VC_length_"+Buffer.from(issuer.accountHash()).toString('hex');
    let vc_merkleRoot_key = "VC_"+Buffer.from(issuer.accountHash()).toString('hex') + "_"+ index+"_merkleRoot";
    let vc_ipfsHash_key = "VC_"+Buffer.from(issuer.accountHash()).toString('hex') + "_"+ index+"_ipfsHash";
    let vc_schemaHash_key = "VC_"+Buffer.from(issuer.accountHash()).toString('hex') + "_"+ index+"_schemaHash";
    let vc_holder_key = "VC_"+Buffer.from(issuer.accountHash()).toString('hex') + "_"+ index+"_holder";
    let vc_revocationFlag_key = "VC_"+Buffer.from(issuer.accountHash()).toString('hex') + "_"+index+"_revocationFlag";


    // "VC_2bb951609cf7bea4b9fee202ba2e8719c3453ed53ed0a19289063665c61b637a_0_merkleRoot"
    //  VC_2bb951609cf7bea4b9fee202ba2e8719c3453ed53ed0a19289063665c61b637a_0_merkleRoot
    //console.log(vc_merkleRoot_key)
    console.log("hash-29710161F8912257718FC2F9a8cF80a55b82f706D22609Cb8190C83De01BD690".)
    let vc_merkleRoot = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,['VC_2bb951609cf7bea4b9fee202ba2e8719c3453ed53ed0a19289063665c61b637a_0_merkleRoot']);
    console.log("vc_merkleRoot_key:");
    console.log(vc_merkleRoot);
    // try{
    //     let vcLength = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_length_key]);
    //     console.log("vc_length_key:");
    //     console.log(vcLength);
    // }catch{
    //     console.log("vc_length_key doesnt instantiated");
    // }
    // try{
    //     let vc_merkleRoot = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_merkleRoot_key]);
    //     console.log("vc_merkleRoot_key:");
    //     console.log(vc_merkleRoot);
    // }catch{
    //     console.log("vc_merkleRoot_key doesnt instantiated");
    // }
    // try{
    //     let vc_ipfsHash = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_ipfsHash_key]);
    //     console.log("vc_ipfsHash_key:");
    //     console.log(vc_ipfsHash);
    // }catch{
    //     console.log("vc_ipfsHash_key doesnt instantiated");
    // }
    // try{
    //     let vc_schemaHash = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_schemaHash_key]);
    //     console.log("vc_schemaHash_key:");
    //     console.log(vc_schemaHash);
    // }catch{
    //     console.log("vc_schemaHash_key doesnt instantiated");
    // }
    // try{
    //     let vc_holder = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_holder_key]);
    //     console.log("vc_holder_key:");
    //     console.log(vc_holder);
    // }catch{
    //     console.log("vc_holder_key doesnt instantiated");
    // }
    // try{
    //     let vc_revocationFlag = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vc_revocationFlag_key]);
    //     console.log("vc_revocationFlag_key:");
    //     console.log(vc_revocationFlag);
    // }catch{
    //     console.log("vc_revocationFlag_key doesnt instantiated");
    // }
 
}

const readVPRequest = async(verifier, index) => {
    // Step 1: Set casper node client.
    const client = new CasperClient(DEPLOY_NODE_ADDRESS);
    const clientRpc = new CasperServiceByJsonRPC(DEPLOY_NODE_ADDRESS);

    // Step 3: Query node for global state root hash.
    const stateRootHash = await clientRpc.getStateRootHash();

    let vp_length_key = "VPRequest_length_"+Buffer.from(verifier.accountHash()).toString('hex');
    let vp_ipfsHash_key = "VPRequest_"+Buffer.from(verifier.accountHash()).toString('hex') + "_"+ index+"_ipfsHash";
    let vp_ipfsHashResponce_key = "VPRequest_"+Buffer.from(verifier.accountHash()).toString('hex') + "_"+ index+"_ipfsHashResponce";
    let vp_holder_key = "VPRequest_"+Buffer.from(verifier.accountHash()).toString('hex') + "_"+ index+"_holder";
    let vp_status_key = "VPRequest_"+Buffer.from(verifier.accountHash()).toString('hex') + "_"+ index+"_status";

    try{
        let vpLength = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vp_length_key]);
        console.log("vp_length_key:");
        console.log(vpLength);
    }catch{
        console.log("vp_length_key doesnt instantiated");
    }
    try{
        let vp_ipfsHash = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vp_ipfsHash_key]);
        console.log("vp_ipfsHash_key:");
        console.log(vp_ipfsHash);
    }catch{
        console.log("vp_ipfsHash_key doesnt instantiated");
    }
    try{
        let vp_ipfsHashResponce = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vp_ipfsHashResponce_key]);
        console.log("vp_ipfsHashResponce_key:");
        console.log(vp_ipfsHashResponce);
    }catch{
        console.log("vp_ipfsHashResponce_key doesnt instantiated");
    }
    try{
        let vp_holder = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vp_holder_key]);
        console.log("vp_holder_key:");
        console.log(vp_holder);
    }catch{
        console.log("vp_holder_key doesnt instantiated");
    }
    try{
        let vp_status = await clientRpc.getBlockState(stateRootHash,CONTRACT_DEMOVCREGISTRY_HASH,[vp_status_key]);
        console.log("vp_status_key:");
        console.log(vp_status);
    }catch{
        console.log("vp_status_key doesnt instantiated");
    }


}


const main = async () => {
   
    let issuer = Keys.Ed25519.parseKeyFiles(
            './network_keys/ilya2/public_key.pem',
            './network_keys/ilya2/secret_key.pem'
        );
    let verifier = Keys.Ed25519.parseKeyFiles(
        './network_keys/demovcregistry_deployer/public_key.pem',
        './network_keys/demovcregistry_deployer/secret_key.pem'
    );
    let index = 0;
    readVC(issuer, index);
    //readVPRequest(verifier, index);
   
};
 
 main();
  