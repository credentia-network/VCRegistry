
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};

use crate::casper_types::{
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
            .normidentityze()
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

    // pub fn balance_of(&self, account: Key) -> Option<U256> {
    //     let item_key = base64::encode(&account.to_bytes().unwrap());

    //     let key = Key::Hash(self.contract_hash().value());
    //     let value = self
    //         .context
    //         .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
    //         .ok()?;

    //     Some(value.into_t::<U256>().unwrap())
    // }

    pub fn owner(&self, identity: AccountHash) -> AccountHash {
        let owner_key = format!("owner_{}", identity.to_string());
        self.query_contract(&owner_key).unwrap()
    }

    pub fn changeOwner(&self, sender : AccountHash, identity : AccountHash, newOwner : AccountHash) {
        self.call(
            sender, 
            "changeOwner",
            runtime_args! {
                "identity" => identity,
                "newOwner" => newOwner
            }
        )
    }

    // pub fn approve(&mut self, spender: Key, amount: U256, sender: AccountHash) {
    //     self.call(
    //         sender,
    //         consts::APPROVE_ENTRY_POINT_NAME,
    //         runtime_args! {
    //             consts::SPENDER_RUNTIME_ARG_NAME => spender,
    //             consts::AMOUNT_RUNTIME_ARG_NAME => amount
    //         },
    //     );
    // }



}
