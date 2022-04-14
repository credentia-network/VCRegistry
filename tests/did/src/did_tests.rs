
#[cfg(test)]
mod didTestFixture;

#[cfg(test)]
mod tests {

   use crate::didTestFixture::{DIDTestFixture};

    #[test]
    fn should_install() {
        let fixture = DIDTestFixture::install_contract();
        
    }

}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
