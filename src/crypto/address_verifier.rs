use nanos_sdk::io::Comm;
use sbor::bech32::address::Address;
use sbor::bech32::encoder::Bech32;
use sbor::bech32::network::NetworkId;
use sbor::static_vec::StaticVec;
use crate::sign::tx_state::info_message;
use crate::ui::multipage_validator::MultipageValidator;

pub fn verify_address(address: Address, network_id: NetworkId, comm: &mut Comm) {
    let mut vec = StaticVec::<u8, { Bech32::MAX_LEN }>::new(0);
    address.format(&mut vec, network_id);

    info_message(b"Address:", vec.as_slice());

    let rc = MultipageValidator::new(&[&"Address Correct?"], &[&"Yes"], &[&"No"]).ask();

    if rc {
        comm.append(&[0x01])
    } else {
        comm.append(&[0x00])
    }
    comm.append(vec.as_slice());
}
