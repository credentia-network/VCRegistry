var _ = require('lodash');
const bs58 = require("bs58");
const caspersdk = require("casper-js-sdk");
const CasperClient = caspersdk.CasperClient;
const CLValueBuilder = caspersdk.CLValueBuilder;
const DeployUtil = caspersdk.DeployUtil;
const Keys = caspersdk.Keys;
const RuntimeArgs = caspersdk.RuntimeArgs;
const CasperServiceByJsonRPC = caspersdk.CasperServiceByJsonRPC;
const fs = require('fs');

const main = async () => {
    
    const secret_key = Keys.Ed25519.parsePrivateKeyFile('./network_keys/demovcregistry_deployer/demovcregistry_deployer_secret_key.pem');
    console.log(secret_key);
    const public_key = Keys.Ed25519.privateToPublicKey(secret_key);
    console.log(public_key);
    const accHash = Keys.Ed25519.accountHash(public_key);
    console.log(accHash);
    const accHex = Keys.Ed25519.accountHex(public_key);
    console.log(accHex);

    const naclKeyPair = Keys.Ed25519.parseKeyPair(public_key, secret_key);
    const publicKeyInPem = naclKeyPair.exportPublicKeyInPem();
    const privateKeyInPem = naclKeyPair.exportPrivateKeyInPem();

    const tempDir = './network_keys/demovcregistry_deployer';
    fs.writeFileSync(tempDir + '/public_key.pem', publicKeyInPem);
    fs.writeFileSync(tempDir + '/secret_key.pem', privateKeyInPem);
    

    const signKeyPair2 = Keys.Ed25519.parseKeyFiles(
      tempDir + '/public_key.pem',
      tempDir + '/secret_key.pem'
    );
    console.log(signKeyPair2.accountHex());

};

main();