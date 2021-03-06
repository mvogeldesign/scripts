extern crate time;
extern crate byteorder;

mod ntp;
pub mod error;

use std::net::UdpSocket;
use time::Timespec;
use self::error::Error;
use self::ntp::NTPHeader;

const NTP_PORT: u16 = 123;
const UDP_LOCAL: &'static str = "0.0.0.0:35000";

/// `retrieve_ntp_timestamp` retrieves the current time from a NTP server.
///
/// # Arguments
///
/// * `host` - The NTP server (i.e. 0.pool.ntp.org)
// Generate NTP header and receive receive NTP header
pub fn retrieve_ntp_timestamp(host: &str) -> Result<Timespec, Error> {
    let header = NTPHeader::new();
    let message = try!(header.encode());

    let socket = try!(UdpSocket::bind(UDP_LOCAL));

    let host = format!("{host}:{port}", host = host, port = NTP_PORT);
    try!(socket.send_to(&message[..], &host[..]));

    let mut buf = [0u8; 1000];

    let (amt, _) = try!(socket.recv_from(&mut buf));

    drop(socket);

    let header = try!(ntp::NTPHeader::decode(amt, &buf));

    Ok(header.transmit_timestamp.as_timespec())
}

#[test]
fn receive_timestamp() {
    const NTP_SERVER: &'static str = "0.au.pool.ntp.org";

    let t1 = retrieve_ntp_timestamp(NTP_SERVER).unwrap();
    let t2 = retrieve_ntp_timestamp(NTP_SERVER).unwrap();

    assert!(t2 > t1);
}
