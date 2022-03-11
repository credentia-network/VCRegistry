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
const { CLTypeBuilder } = require('casper-js-sdk');
const DEPLOY_GAS_PRICE = 10;
const DEPLOY_GAS_PAYMENT = 50000000000;
const DEPLOY_TTL_MS = 3600000;

const issueDemoVC = async (msgSender, merkleRoot, ipfsHash, schemaHash, holder, revocationFlag) => {
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
            "issueDemoVC",
            RuntimeArgs.fromMap({
                merkleRoot: CLValueBuilder.byteArray(merkleRoot),
                ipfsHash:   CLValueBuilder.byteArray(ipfsHash),
                schemaHash: CLValueBuilder.byteArray(schemaHash),
                holder:     CLValueBuilder.byteArray(holder.accountHash()),
                revocationFlag:  CLValueBuilder.bool(revocationFlag),
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
   
    let msgSender = Keys.Ed25519.parseKeyFiles(
        './network_keys/demovcregistry_deployer/public_key.pem',
        './network_keys/demovcregistry_deployer/secret_key.pem'
    );
    let raw1 = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
    let merkleRoot = new Uint8Array(raw1);
    let raw2 = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
    let ipfsHash = new Uint8Array(raw2)
    let raw3 = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
    let schemaHash = new Uint8Array(raw3);
    let holder = Keys.Ed25519.parseKeyFiles(
            './network_keys/victor/public_key.pem',
            './network_keys/victor/secret_key.pem'
        );
    let revocationFlag = true;
    
    await issueDemoVC(msgSender,merkleRoot,ipfsHash,schemaHash,holder,revocationFlag);

};

main();