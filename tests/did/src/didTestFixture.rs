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

const CONTRACT_WASM: &str = "did.wasm";
const CONTRACT_KEY_NAME: &str = "CasperDIDRegistry";

pub struct DIDTestFixture {
    pub context: TestContext,
    pub identity: AccountHash,
    pub owner: AccountHash,
}

impl DIDTestFixture {

    pub fn install_contract() -> DIDTestFixture {
        let identity = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let owner = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(identity.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(owner.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_WASM);
        let session_args = runtime_args! {};

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(identity.to_account_hash())
            .with_authorization_keys(&[identity.to_account_hash()])
            .build();

        context.run(session);
        DIDTestFixture {
            context,
            identity: identity.to_account_hash(),
            owner: owner.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.identity)
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
            .query(self.identity, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
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

    pub fn owner(&self, identity: AccountHash) -> AccountHash {
        let owner_key = format!("owner_{}", identity.to_string());
        let owner_value : Option<AccountHash> = self.query_contract(&owner_key);
        match owner_value {
            // if return value is None, that mean that no value stored by this key
            None => AccountHash::new([0u8;32]),
            Some(owner_value) => owner_value
        }
    }

    pub fn changeOwner(
        &mut self, 
        sender : AccountHash, 
        identity : AccountHash, 
        newOwner : AccountHash
    ) {
        self.call(
            sender, 
            "changeOwner",
            runtime_args! {
                "identity" => identity,
                "newOwner" => newOwner
            }
        )
    }

    pub fn delegate_length(&self, identity: AccountHash) -> u64 {
        let delegate_length_key = format!("{}_delegateLength",identity);
        let delegate_length : Option<u64> = self.query_contract(&delegate_length_key);
        match delegate_length {
            // if return value is None, that mean that no value stored by this key
            None => 0,
            Some(delegate_length) => delegate_length
        }
    }

    pub fn delegate(&self, identity: AccountHash, index: u64) -> (String,String,u64) {
        let delegate_key = format!("{}_delegate_{}",identity,index);
        let delegate : Option<(String,String,u64)> = self.query_contract(&delegate_key);
        match delegate {
            // if return value is None, that mean that no value stored by this key
            None => (String::from(""), String::from(""), 0),
            Some(delegate) => delegate
        }
    }

    pub fn addDelegate(
        &mut self,
        sender : AccountHash, 
        identity : AccountHash, 
        delegateKey : String,
        delegateValue : String,
        expire : u64
    ) {
        self.call(
            sender, 
            "addDelegate",
            runtime_args! {
                "identity" => identity,
                "delegateKey" => delegateKey,
                "delegateValue" => delegateValue,
                "expire" => expire
            }
        )
    }

    pub fn revokeDelegate(
        &mut self, 
        sender : AccountHash, 
        identity : AccountHash, 
        index : u64
    ) {
        self.call(
            sender, 
            "revokeDelegate",
            runtime_args! {
                "identity" => identity,
                "index" => index
            }
        )
    }

    pub fn attribute_length(&self, identity: AccountHash) -> u64 {
        let attribute_length_key = format!("{}_attributeLength",identity);
        let attribute_length : Option<u64> = self.query_contract(&attribute_length_key);
        match attribute_length {
            // if return value is None, that mean that no value stored by this key
            None => 0,
            Some(attribute_length) => attribute_length
        }
    }

    pub fn attrubute(&self, identity: AccountHash, index: u64) -> (String,String,u64) {
        let attrubute_key = format!("{}_attribute_{}", identity, index);
        let attrubute : Option<(String,String,u64)> = self.query_contract(&attrubute_key);
        match attrubute {
            // if return value is None, that mean that no value stored by this key
            None => (String::from(""), String::from(""), 0),
            Some(attrubute) => attrubute
        }
    }

    pub fn setAttribute(
        &mut self,
        sender : AccountHash, 
        identity : AccountHash, 
        attributeKey : String,
        attributeValue : String,
        expire : u64
    ) {
        self.call(
            sender, 
            "setAttribute",
            runtime_args! {
                "identity" => identity,
                "attributeKey" => attributeKey,
                "attributeValue" => attributeValue,
                "expire" => expire
            }
        )
    }

    pub fn revokeAttribute(
        &mut self,
        sender : AccountHash, 
        identity: AccountHash,
        index : u64
    ) {
        self.call(
            sender, 
            "revokeAttribute",
            runtime_args! {
                "identity" => identity,
                "index" => index
            }
        )
    }

}
