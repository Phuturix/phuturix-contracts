use scrypto::prelude::*;

#[blueprint]
mod radiphuturix {
    struct RadiPhuturix {
        /// A vault containing pool reverses of  token 
        vault: FungibleVault,
        /// The token address of a token representing pool units in this pool
        pool_units_resource_manager: ResourceManager,
        /// The amount of fees imposed by the pool on swaps where 0 <= fee <= 1.
        fee: Decimal,
        //These are list of all the position long and short
        positions: HashMap<ResourceAddress, Position>
    }

    impl RadiPhuturix {
        /// Creates a new liquidity pool of the two tokens sent to the pool
        pub fn instantiate_radiswap(
            bucket: FungibleBucket,
            custom_fee: Decimal,
            name: String
        ) -> (Global<RadiPhuturix>, FungibleBucket) {
            // Ensure that none of the buckets are empty and that an appropriate  and the fee is set
            assert!(
                !bucket.is_empty(),
                "You must pass in an initial supply of each token"
            );
            assert!(
                custom_fee >= dec!("0") && custom_fee <= dec!("1"),
                "Invalid fee"
            );

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(RadiPhuturix::blueprint_id());

            // Create the pool units token along with the initial supply specified  
            // by the user.
            let pool_units: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => name, locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                ))
                .mint_initial_supply(100);

            // Create the RadiPhuturix component and globalize it
            let radiphuturix = Self {
                vault: FungibleVault::with_bucket(bucket),
                pool_units_resource_manager: pool_units.resource_manager(),
                fee: custom_fee,
                positions: HashMap::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize();

            // Return the component address as well as the pool units tokens
            (radiphuturix, pool_units)
        }

        /// Swaps token A for B, or vice versa.
        pub fn swap(&mut self, input_tokens: FungibleBucket) {
            println!("{:?}", input_tokens)
            // Getting the vault corresponding to the input tokens and the vault 
            // corresponding to the output tokens based on what the input is.
            // let (input_tokens_vault, output_tokens_vault): (&mut FungibleVault, &mut FungibleVault) =
            //     if input_tokens.resource_address() == 
            //     self.vault_a.resource_address() {
            //         (&mut self.vault_a, &mut self.vault_b)
            //     } else if input_tokens.resource_address() == 
            //     self.vault_b.resource_address() {
            //         (&mut self.vault_b, &mut self.vault_a)
            //     } else {
            //         panic!(
            //         "The given input tokens do not belong to this liquidity pool"
            //         )
            //     };

            // // Calculate the output amount of tokens based on the input amount 
            // // and the pool fees
            // let output_amount: Decimal = (output_tokens_vault.amount()
            //     * (dec!("1") - self.fee)
            //     * input_tokens.amount())
            //     / (input_tokens_vault.amount() + input_tokens.amount() 
            //     * (dec!("1") - self.fee));

            // // Perform the swapping operation
            // input_tokens_vault.put(input_tokens);
            // output_tokens_vault.take(output_amount)
        }
        

    }
}


pub enum Side {
    Long,
    Short
}

//TODO: how will we know the user Address of the position? 
pub struct Position{
    pub side: Side,
    pub open_close_time_position: [String; 2],
    pub leverage: f64,
    pub open_price: f32,
    pub liquidation_price: f32,
    pub borrowed_size: f32,
    pub collateral_size: f32,
    pub fees: f32,
    pub is_active: bool,
}