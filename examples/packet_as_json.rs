//! A simple example demonstrating how to dump a packet as a Json
//!

use scalpel;

fn main() {
    let _ = scalpel::register_defaults();
    let dns_packet = vec![
        0x52, 0x54, 0x00, 0xbd, 0x1c, 0x70, 0xfe, 0x54, /* RT...p.T */
        0x00, 0x3e, 0x00, 0x96, 0x08, 0x00, 0x45, 0x00, /* .>....E. */
        0x00, 0xe0, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, /* ....@.@. */
        0xc4, 0x74, 0xc0, 0xa8, 0x7a, 0x01, 0xc0, 0xa8, /* .t..z... */
        0x7a, 0x46, 0x00, 0x35, 0xdb, 0x13, 0x00, 0xcc, /* zF.5.... */
        0x76, 0x76, /* DNS */ 0xf3, 0x03, 0x81, 0x80, 0x00, 0x01, /* vv...... */
        0x00, 0x01, 0x00, 0x04, 0x00, 0x04, 0x03, 0x77, /* .......w */
        0x77, 0x77, 0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, /* ww.googl */
        0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x1c, /* e.com... */
        0x00, 0x01, 0xc0, 0x0c, 0x00, 0x1c, 0x00, 0x01, /* ........ */
        0x00, 0x00, 0x01, 0x2c, 0x00, 0x10, 0x2a, 0x00, /* ...,..*. */
        0x14, 0x50, 0x40, 0x0c, 0x0c, 0x01, 0x00, 0x00, /* .P@..... */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0xc0, 0x10, /* .....i.. */
        0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
        0x00, 0x06, 0x03, 0x6e, 0x73, 0x34, 0xc0, 0x10, /* ...ns4.. */
        0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, /* ........ */
        0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, 0x73, 0x32, /* .....ns2 */
        0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, 0x00, 0x01, /* ........ */
        0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, 0x03, 0x6e, /* .......n */
        0x73, 0x31, 0xc0, 0x10, 0xc0, 0x10, 0x00, 0x02, /* s1...... */
        0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, 0x00, 0x06, /* ........ */
        0x03, 0x6e, 0x73, 0x33, 0xc0, 0x10, 0xc0, 0x6c, /* .ns3...l */
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
        0x00, 0x04, 0xd8, 0xef, 0x20, 0x0a, 0xc0, 0x5a, /* .... ..Z */
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
        0x00, 0x04, 0xd8, 0xef, 0x22, 0x0a, 0xc0, 0x7e, /* ...."..~ */
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
        0x00, 0x04, 0xd8, 0xef, 0x24, 0x0a, 0xc0, 0x48, /* ....$..H */
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0xa3, 0x00, /* ........ */
        0x00, 0x04, 0xd8, 0xef, 0x26, 0x0a, /* ....&. */
    ];
    let p = scalpel::packet::Packet::from_bytes(&dns_packet, scalpel::types::ENCAP_TYPE_ETH);

    println!("{}", serde_json::to_string_pretty(&p.unwrap()).unwrap());

    let sctp_packet = hex::decode(
    "00005096523a0026cb39f4c00800450000a8da490000fa844bf6585206860aad300d189f0b5add68d33d0f7373ab030000100629beaa0000fa000000000000030028d42b4897000000050000000301000202000000180012000800000a43000600080000045600030028d42b4898000000060000000301000202000000180012000800000a42000600080000045600030028d42b4899000000070000000301000202000000180012000800000fa20006000800000456");

    let sctp_packet = sctp_packet.unwrap();
    let p = scalpel::packet::Packet::from_bytes(&sctp_packet, scalpel::types::ENCAP_TYPE_ETH);

    println!("{}", serde_json::to_string_pretty(&p.unwrap()).unwrap());
}
