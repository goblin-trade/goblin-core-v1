// use crate::{
//     axis::token::{
//         CustomERC20, TokenDataTriple, token_data_triple, token_list::CustomERC20List,
//         token_marker::TokenData,
//     },
//     input_processor::{GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, global_args},
//     quantities::UnsidedAtoms,
// };

// #[test]
// fn test_deposit_eth() {
//     let custom_erc20_list_inner: [TokenData<CustomERC20>; 0] = [];
//     let custom_erc20_count = custom_erc20_list_inner.len();
//     let custom_erc20_list = CustomERC20List {
//         inner: custom_erc20_list_inner.as_ref(),
//     };
//     let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

//     let global_args = GlobalArgs {
//         flags: HeaderFlags {
//             read_custom_recipient: false,
//             read_msg_value: true,
//             process_dynamic_markets: false,
//             withdraw_eth: false,
//             withdraw_internally: false,
//             custom_erc20_count,
//         },
//         header: Header {
//             eth_out_due_u32: UnsidedAtoms::default(),
//             market_counts: MarketCounts::default(),
//         },
//         refs: HeaderRefs {
//             custom_recipient: None,
//             token_data_triple,
//         },
//     };
// }
