use crate::types::FieldElement;

pub const MAINNET: FieldElement = FieldElement::from_mont([
    17696389056366564951,
    18446744073709551615,
    18446744073709551615,
    502562008147966918,
]);

#[deprecated = "The Goerli testnet has been shutdown"]
pub const TESTNET: FieldElement = FieldElement::from_mont([
    3753493103916128178,
    18446744073709548950,
    18446744073709551615,
    398700013197595345,
]);

#[deprecated = "The Goerli testnet has been shutdown"]
pub const TESTNET2: FieldElement = FieldElement::from_mont([
    1663542769632127759,
    18446744073708869172,
    18446744073709551615,
    33650220878420990,
]);

pub const SEPOLIA: FieldElement = FieldElement::from_mont([
    1555806712078248243,
    18446744073708869172,
    18446744073709551615,
    507980251676163170,
]);

#[cfg(test)]
mod test {
    use crate::utils::cairo_short_string_to_felt;

    use super::*;

    #[test]
    #[allow(deprecated)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_chain_ids() {
        for (text, felt) in [
            ("SN_MAIN", MAINNET),
            ("SN_GOERLI", TESTNET),
            ("SN_GOERLI2", TESTNET2),
            ("SN_SEPOLIA", SEPOLIA),
        ]
        .into_iter()
        {
            assert_eq!(cairo_short_string_to_felt(text).unwrap(), felt);
        }
    }
}
