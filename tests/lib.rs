#[cfg(test)]
mod tests {
    use scrypto::prelude::*;
    use scrypto_test::prelude::*;

    #[test]
    fn test_instantiate_phuturex() {
        // Create a test environment

        let mut ledger = LedgerSimulatorBuilder::new().build();

        let package_address = ledger.compile_and_publish(this_package!());

        let (_, _, account) = ledger.new_account(false);

        let token_address = ledger.create_fungible_resource(
            dec!(1000),  // Initial supply
            18,          // Divisibility
            account      // Account to receive the initial supply
        );

        let manifest = ManifestBuilder::new()
            .call_function(
                package_address,
                "phuturex",
                "instantiate_phuturex",
                manifest_args!(token_address, dec!("0.001")),
            )
            .build();
        let receipt = ledger.execute_manifest(manifest, vec![]);

        // Assert
        let result = receipt.expect_commit_success();
        assert_eq!(result.new_component_addresses().len(), 1);
        assert_eq!(result.new_resource_addresses().len(), 1);
    }
}