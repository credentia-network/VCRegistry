#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::thread::AccessError;

use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};

use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};

const CONTRACT_WASM: &str = "demovcregistry.wasm";
const CONTRACT_KEY_NAME: &str = "CasperDemoVCRegistry";

pub struct DemovcregistryTestFixture {
    pub context: TestContext,
    pub issuer: AccountHash,
    pub holder: AccountHash,
    pub verifier: AccountHash
}

impl DemovcregistryTestFixture {

    pub fn install_contract() -> DemovcregistryTestFixture {
        let issuer = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let holder = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let verifier = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(issuer.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(holder.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(verifier.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_WASM);
        let session_args = runtime_args! {};

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(issuer.to_account_hash())
            .with_authorization_keys(&[issuer.to_account_hash()])
            .build();

        context.run(session);
        DemovcregistryTestFixture {
            context,
            issuer: issuer.to_account_hash(),
            holder: holder.to_account_hash(),
            verifier: verifier.to_account_hash()
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.issuer)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.issuer, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: AccountHash, method: &str, args: RuntimeArgs) {
        let address = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn vc_length(
        &self, 
        issuer: AccountHash) -> u64 {
        let vc_length_key = format!("VC_length_{}",issuer);
        let vc_length_value: Option<u64> = self.query_contract(&vc_length_key);
        match vc_length_value {
            None => 0,
            Some(vc_length_value) => vc_length_value
        }
    }

    pub fn vc(
        &self, 
        issuer: AccountHash, 
        index: u64
    ) -> ([u8;32], [u8;32], [u8;32], AccountHash, bool) {

        let merkleRoot_key =  format!("VC_{}_{}_{}", issuer, index, "merkleRoot");
        let ipfsHash_key =  format!("VC_{}_{}_{}", issuer, index, "ipfsHash");
        let schemaHash_key =  format!("VC_{}_{}_{}", issuer, index, "schemaHash");
        let holder_key =  format!("VC_{}_{}_{}", issuer, index, "holder");
        let revocationFlag_key =  format!("VC_{}_{}_{}", issuer, index, "revocationFlag");
        
        let merkleRoot_value: Option<[u8;32]> = self.query_contract(&merkleRoot_key);
        let ipfsHash_value: Option<[u8;32]> = self.query_contract(&ipfsHash_key);
        let schemaHash_value: Option<[u8;32]> = self.query_contract(&schemaHash_key);
        let holder_value: Option<AccountHash> = self.query_contract(&holder_key);
        let revocationFlag_value: Option<bool> = self.query_contract(&revocationFlag_key);

        
        let merkleRoot_value = match merkleRoot_value {
            None => [0u8;32] ,
            Some(merkleRoot_value) => merkleRoot_value 
        };
        let ipfsHash_value = match ipfsHash_value {
            None => [0u8;32] ,
            Some(ipfsHash_value) => ipfsHash_value 
        };
        let schemaHash_value = match schemaHash_value {
            None => [0u8;32] ,
            Some(schemaHash_value) => schemaHash_value 
        };
        let holder_value = match holder_value {
            None => AccountHash::new([0u8;32]) ,
            Some(holder_value) => holder_value 
        };
        let revocationFlag_value = match revocationFlag_value {
            None => false ,
            Some(revocationFlag_value) => revocationFlag_value 
        };

        (merkleRoot_value, ipfsHash_value, schemaHash_value, holder_value, revocationFlag_value)
    }

    pub fn issueDemoVC(
        &mut self, 
        sender: AccountHash, 
        merkleRoot: [u8;32], 
        ipfsHash: [u8;32],
        schemaHash: [u8;32],
        holder: AccountHash,
        revocationFlag: bool
    ) {
        self.call(
            sender, 
            "issueDemoVC",
            runtime_args! {
                "merkleRoot" => merkleRoot,
                "ipfsHash" => ipfsHash,
                "schemaHash" => schemaHash,
                "holder" => holder,
                "revocationFlag" => revocationFlag
            }
        )
    }

    pub fn changeVCRevocationFlag(
        &mut self, 
        sender: AccountHash,
        index: u64,
        revocationFlag: bool
    ) {
        self.call(
            sender,
            "changeVCRevocationFlag",
            runtime_args! {
                "index" => index,
                "revocationFlag" => revocationFlag
            }
        )
    }

    pub fn vp_length(
        &self,
        verifier: AccountHash
    ) -> u64 {
        let vp_length_key = format!("VPRequest_length_{}", verifier);
        let vp_length_value: Option<u64> = self.query_contract(&vp_length_key);
        match vp_length_value {
            None => 0,
            Some(vp_length_value) => vp_length_value
        }
    }

    pub fn vp(
        &self,
        verifier: AccountHash,
        index: u64
    ) -> ([u8;32], [u8;32], AccountHash, u8) {
        let ipfsHash_key = format!("VPRequest_{}_{}_{}", verifier, index, "ipfsHash");
        let ipfsHashResponce_key = format!("VPRequest_{}_{}_{}", verifier, index, "ipfsHashResponce");
        let holder_key = format!("VPRequest_{}_{}_{}", verifier, index, "holder");
        let status_key = format!("VPRequest_{}_{}_{}", verifier, index, "status");
    
        let ipfsHash_value: Option<[u8;32]> = self.query_contract(&ipfsHash_key);
        let ipfsHashResponce_value: Option<[u8;32]> = self.query_contract(&ipfsHashResponce_key);
        let holder_value: Option<AccountHash> = self.query_contract(&holder_key);
        let status_value: Option<u8> = self.query_contract(&status_key);

        let ipfsHash_value = match ipfsHash_value {
            None => [0u8;32],
            Some(ipfsHash_value) => ipfsHash_value
        };
        
        let ipfsHashResponce_value = match ipfsHashResponce_value {
            None => [0u8;32],
            Some(ipfsHashResponce_value) => ipfsHashResponce_value
        };

        let holder_value = match holder_value {
            None => AccountHash::new([0u8;32]),
            Some(holder_value) => holder_value
        };

        let status_value = match status_value {
            None => 0u8,
            Some(status_value) => status_value
        };

        (ipfsHash_value, ipfsHashResponce_value, holder_value, status_value)
    }

    pub fn sendVPRequest(
        &mut self, 
        sender: AccountHash,
        ipfsHash: [u8;32],
        holder: AccountHash
    ) {
        self.call(
            sender, 
            "sendVPRequest", 
            runtime_args! {
                "ipfsHash" => ipfsHash,
                "holder" => holder
            }
        )
    }

    pub fn changeVPRequestStatus(
        &mut self,
        sender: AccountHash,
        verifier: AccountHash,
        index: u64,
        newStatus: u8,
        ipfsHashResponce: [u8;32]
    ) {
        self.call(
            sender, 
            "changeVPRequestStatus",
            runtime_args! {
                "verifier" => verifier,
                "index" => index,
                "newStatus" => newStatus,
                "ipfsHashResponce" => ipfsHashResponce
            }
        )
    }
   
}
