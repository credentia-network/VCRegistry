#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

#[cfg(test)]
mod demovcregistryTestFixture;

#[cfg(test)]
mod tests {

use crate::demovcregistryTestFixture::{DemovcregistryTestFixture};

    #[test]
    fn should_issueDemoVC() {
        println!("");
        let mut fixture = DemovcregistryTestFixture::install_contract();
        let issuer = fixture.issuer;

        let vc_length_before = fixture.vc_length(issuer);
        println!("vc length before: {}", vc_length_before);
        
        let vc_before = fixture.vc(issuer, vc_length_before);
        println!("vc before: {:?}", vc_before);

        let merkleRoot = [1u8;32];
        let ipfsHash = [2u8;32];
        let schemaHash = [3u8;32];
        let holder = fixture.holder;
        let revocationFlag = true;
        fixture.issueDemoVC(issuer, merkleRoot, ipfsHash, schemaHash, holder, revocationFlag);

        let vc_length_after = fixture.vc_length(issuer);
        println!("vc length after: {}", vc_length_after);
        
        let vc_after = fixture.vc(issuer, vc_length_before);
        println!("vc after: {:?}", vc_after);

        assert!(vc_after.0 == merkleRoot);
        assert!(vc_after.1 == ipfsHash);
        assert!(vc_after.2 == schemaHash);
        assert!(vc_after.3 == holder);
        assert!(vc_after.4 == revocationFlag);

        let revocationFlag = false;
        fixture.changeVCRevocationFlag(issuer, vc_length_before, revocationFlag);

        let vc_after_changeRevocationFlag = fixture.vc(issuer, vc_length_before);
        println!("vc after changeRevocationFlag: {:?}", vc_after_changeRevocationFlag);

        assert!(vc_after_changeRevocationFlag.4 == revocationFlag);

    }

    #[test]
    fn should_sendVP_and_changeVPRequestStatus_to_approve() {
        println!("");
        let mut fixture = DemovcregistryTestFixture::install_contract();
        let verifier = fixture.verifier;
        
        let vp_length_before = fixture.vp_length(verifier);
        println!("vp length before: {}", vp_length_before);

        let vp_before = fixture.vp(verifier, vp_length_before);
        println!("vp before: {:?}", vp_before);

        let ipfsHash = [1u8;32];
        let holder = fixture.holder;
        fixture.sendVPRequest(verifier, ipfsHash, holder);

        let vp_after = fixture.vp(verifier, vp_length_before);
        println!("vp after: {:?}", vp_after);

        assert!(vp_after.0 == ipfsHash, "ipfsHash should be differ");
        assert!(vp_after.1 == [0u8;32], "ipfsHashResponce should be not instantiated");
        assert!(vp_after.2 == holder, "expect another holder");
        assert!(vp_after.3 == 0, "status is not 0");

        let index = vp_length_before;
        let newStatus = 1;
        let ipfsHashResponce = [2u8;32];
        fixture.changeVPRequestStatus(holder, verifier, index, newStatus, ipfsHashResponce);

        let vp_after_changeStatus = fixture.vp(verifier, vp_length_before);
        println!("vp after changeStatus: {:?}", vp_after_changeStatus);
        
        assert!(vp_after_changeStatus.1 == ipfsHashResponce, "should be expected ipfsHashResponce");
        assert!(vp_after_changeStatus.3 == newStatus, "should be input status");

    }

    #[test]
    fn should_sendVP_and_changeVPRequestStatus_to_reject() {
        println!("");
        let mut fixture = DemovcregistryTestFixture::install_contract();
        let verifier = fixture.verifier;
        
        let vp_length_before = fixture.vp_length(verifier);
        println!("vp length before: {}", vp_length_before);

        let vp_before = fixture.vp(verifier, vp_length_before);
        println!("vp before: {:?}", vp_before);

        let ipfsHash = [1u8;32];
        let holder = fixture.holder;
        fixture.sendVPRequest(verifier, ipfsHash, holder);

        let vp_after = fixture.vp(verifier, vp_length_before);
        println!("vp after: {:?}", vp_after);

        assert!(vp_after.0 == ipfsHash, "ipfsHash should be differ");
        assert!(vp_after.1 == [0u8;32], "ipfsHashResponce should be not instantiated");
        assert!(vp_after.2 == holder, "expect another holder");
        assert!(vp_after.3 == 0, "status is not 0");

        let index = vp_length_before;
        let newStatus = 2;
        let ipfsHashResponce = [2u8;32];
        fixture.changeVPRequestStatus(holder, verifier, index, newStatus, ipfsHashResponce);

        let vp_after_changeStatus = fixture.vp(verifier, vp_length_before);
        println!("vp after changeStatus: {:?}", vp_after_changeStatus);
        
        assert!(vp_after_changeStatus.1 == ipfsHashResponce, "should be expected ipfsHashResponce");
        assert!(vp_after_changeStatus.3 == newStatus, "should be input status");

    }

    #[test]
    fn should_sendVP_and_changeVPRequestStatus_to_revert_by_verifier() {
        println!("");
        let mut fixture = DemovcregistryTestFixture::install_contract();
        let verifier = fixture.verifier;
        
        let vp_length_before = fixture.vp_length(verifier);
        println!("vp length before: {}", vp_length_before);

        let vp_before = fixture.vp(verifier, vp_length_before);
        println!("vp before: {:?}", vp_before);

        let ipfsHash = [1u8;32];
        let holder = fixture.holder;
        fixture.sendVPRequest(verifier, ipfsHash, holder);

        let vp_after = fixture.vp(verifier, vp_length_before);
        println!("vp after: {:?}", vp_after);

        assert!(vp_after.0 == ipfsHash, "ipfsHash should be differ");
        assert!(vp_after.1 == [0u8;32], "ipfsHashResponce should be not instantiated");
        assert!(vp_after.2 == holder, "expect another holder");
        assert!(vp_after.3 == 0, "status is not 0");

        let holder = fixture.holder;
        let index = vp_length_before;
        let newStatus = 3;
        let ipfsHashResponce = [2u8;32];
        fixture.changeVPRequestStatus(verifier, verifier, index, newStatus, ipfsHashResponce);

        let vp_after_changeStatus = fixture.vp(verifier, index);
        println!("vp after changeStatus: {:?}", vp_after_changeStatus);
        
        assert!(vp_after_changeStatus.3 == newStatus, "should be input status");

    }
}

fn main() {
    panic!("Execute \"make test\"");
}
