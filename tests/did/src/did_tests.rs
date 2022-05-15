#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

#[cfg(test)]
mod didTestFixture;

#[cfg(test)]
mod tests {

use crate::didTestFixture::{DIDTestFixture};

    #[test]
    fn should_change_owner() {
        println!("");
        let mut fixture = DIDTestFixture::install_contract();

        let owner_before = fixture.owner(fixture.identity);
        println!("Owner before: {}", owner_before);

        fixture.changeOwner(fixture.identity, fixture.identity, fixture.owner);

        let owner_after = fixture.owner(fixture.identity);
        println!("Owner after:  {}", owner_after);

        assert!(fixture.owner == owner_after, "actual owner doesnt match with expected owner")
    }

    #[test]
    fn should_add_delegate() {
        println!("");
        let mut fixture = DIDTestFixture::install_contract();

        let identity = fixture.identity;

        let delegate_length_before = fixture.delegate_length(fixture.identity);
        println!("delegate length before: {}", delegate_length_before);
        
        let delegate_before = fixture.delegate(identity, delegate_length_before);
        println!("Delegate before: {:?}", delegate_before);

        let delegateKey = String::from("KEY"); 
        let delegateValue = String::from("VALUE");
        let expire: u64 = 9999999999999999;

        fixture.addDelegate(identity, identity, delegateKey, delegateValue, expire);


        let delegate_length_after = fixture.delegate_length(fixture.identity);
        println!("delegate length after:  {}", delegate_length_after);

        let delegate_after = fixture.delegate(identity, delegate_length_before);
        println!("Delegate after: {:?}", delegate_after);
        
        assert!(delegate_length_before + 1 == delegate_length_after," should be added delegate");
        assert!(delegate_after.0 != String::from("") && delegate_after.1 != String::from("") && delegate_after.2 != 0, "should be non zero delegate");

        fixture.revokeDelegate(identity, identity, 0);

        let delegate_after_revoke = fixture.delegate(identity, delegate_length_before);
        println!("Delegate after revoke: {:?}", delegate_after_revoke);

        assert!(delegate_after_revoke.2 == 0, "delegate is valid")
    }

    #[test]
    fn should_add_attribute() {
        println!("");
        let mut fixture = DIDTestFixture::install_contract();    
        
        let identity = fixture.identity;

        let attribute_length_before = fixture.attribute_length(identity);
        println!("attribute length before: {}", attribute_length_before);

        let attribute_before = fixture.attrubute(identity, attribute_length_before);
        println!("attribute before: {:?}", attribute_before);

        let attributeKey = String::from("KEY");
        let attributeValue = String::from("VALUE");
        let expire: u64 = 9999999999999999;

        fixture.setAttribute(identity, identity, attributeKey, attributeValue, expire);
    
        let attribute_length_after = fixture.attribute_length(identity);
        println!("attribute length after:  {}", attribute_length_after);

        let attribute_after = fixture.attrubute(identity, attribute_length_before);
        println!("attribute before: {:?}", attribute_after);

        assert!(attribute_length_before + 1 == attribute_length_after," should be added delegate");
        assert!(attribute_after.0 != String::from("") && attribute_after.1 != String::from("") && attribute_after.2 != 0, "should be non zero delegate");
        
        fixture.revokeAttribute(identity, identity, 0);

        let attribute_after_revoke = fixture.attrubute(identity, attribute_length_before);
        println!("attribute before: {:?}", attribute_after_revoke);
        
        assert!(attribute_after_revoke.2 == 0, "attribute is valid")

    }


}

fn main() {
    panic!("Execute \"make test\"");
}
