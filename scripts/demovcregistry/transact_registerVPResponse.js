var _ = require('lodash');
const caspersdk = require("casper-js-sdk");
const CasperClient = caspersdk.CasperClient;
const CLValueBuilder = caspersdk.CLValueBuilder;
const DeployUtil = caspersdk.DeployUtil;
const Keys = caspersdk.Keys;
const RuntimeArgs = caspersdk.RuntimeArgs;
const CasperServiceByJsonRPC = caspersdk.CasperServiceByJsonRPC;


const { CONTRACT_DEMOVCREGISTRY_NAME, 
        CONTRACT_DEMOVCREGISTRY_HASH,
        DEPLOY_NODE_ADDRESS,
        DEPLOY_CHAIN_NAME } = require("../constants");
const { CLList } = require('casper-js-sdk');
const DEPLOY_GAS_PRICE = 10;
const DEPLOY_GAS_PAYMENT = 50000000000;
const DEPLOY_TTL_MS = 3600000;

const registerVPResponse = async (msgSender ,newIpfsResponceHash, verifier, index) => {
    // Step 1: Set casper node client.
    const client = new CasperClient(DEPLOY_NODE_ADDRESS);
    const clientRpc = new CasperServiceByJsonRPC(DEPLOY_NODE_ADDRESS);

    // Step 2: Set contract operator key pair.
    const keyPairOfSender = msgSender;

    // Step 3: Query node for global state root hash.
    const stateRootHash = await clientRpc.getStateRootHash();

    // Step 4: Query node for contract hash.
    const contractHashAsByteArray = [...Buffer.from(CONTRACT_DEMOVCREGISTRY_HASH.slice(5), "hex")];

    // Step 5.0: Form input parametrs.
    

    // Step 5.1: Form the deploy.
    let deploy = DeployUtil.makeDeploy(
        new DeployUtil.DeployParams(
            keyPairOfSender.publicKey,
            DEPLOY_CHAIN_NAME,
            DEPLOY_GAS_PRICE,
            DEPLOY_TTL_MS
        ),
        DeployUtil.ExecutableDeployItem.newStoredContractByHash(
            contractHashAsByteArray,
            "registerVPResponse",
            RuntimeArgs.fromMap({
                newIpfsResponceHash: CLValueBuilder.byteArray(newIpfsResponceHash),
                verifier: CLValueBuilder.byteArray(verifier.accountHash()),
                index: CLValueBuilder.u64(index),
            })
        ),
        DeployUtil.standardPayment(DEPLOY_GAS_PAYMENT)
    );

    // Step 5.2: Sign deploy.
    deploy = client.signDeploy(deploy, keyPairOfSender); 
    console.log("signed deploy:");
    console.log(deploy);

    // Step 5.3: Dispatch deploy to node.
    let deployResult = await client.putDeploy(deploy);
    console.log("Deploy result");
    console.log(deployResult);
};


const main = async () => {
    
    let trent = Keys.Ed25519.parseKeyFiles(
        './network_keys/trent/public_key.pem',
        './network_keys/trent/secret_key.pem'
    );
    let victor = Keys.Ed25519.parseKeyFiles(
        './network_keys/victor/public_key.pem',
        './network_keys/victor/secret_key.pem'
    );

    let msgSender = victor

    let raw1 = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
    let newIpfsResponceHash = new Uint8Array(raw1);
    let verifier = trent;
    let index = 0;
    await registerVPResponse(msgSender, newIpfsResponceHash, verifier, index);

};



main();