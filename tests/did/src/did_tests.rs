
#[cfg(test)]
mod didTestFixture;

#[cfg(test)]
mod tests {

use crate::didTestFixture::{DIDTestFixture};

    #[test]
    fn should_change_owner() {
        let mut fixture = DIDTestFixture::install_contract();

        let owner_before = fixture.owner(fixture.identity);
        println!("Owner before: {}", owner_before);

        fixture.changeOwner(fixture.identity, fixture.identity, fixture.owner);

        let owner_after = fixture.owner(fixture.identity);
        println!("Owner after:  {}", owner_after);

        assert!(fixture.owner == owner_after, "actual owner doesnt match with expected owner")
    }

    
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
