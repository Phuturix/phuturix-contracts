use scrypto::prelude::*;

pub mod state;
use state::{Position, PositionState, Side};

pub enum ManifestCustomValue {
    Address(ManifestAddress),
    Bucket(ManifestBucket),
    Proof(ManifestProof),
    Expression(ManifestExpression),
    Blob(ManifestBlobRef),
    Decimal(ManifestDecimal),
    PreciseDecimal(ManifestPreciseDecimal),
    NonFungibleLocalId(ManifestNonFungibleLocalId),
    AddressReservation(ManifestAddressReservation),
}

#[blueprint]
mod phuturex {
    use dummy_oracle::DummyOracle;

    struct Phuturex {
        /// This is the vault where the reserve of Token (eg. Radix), that will be used for making transaction long and short calls from a user
        pool: Vault,
        /// This is a decimal value between 0 and 1 which defines the percentage of fee, that user need to pay, to make a position.
        fee: Decimal,
        ///This badge is the badge that has the authority to Add and windrow the tokens and change the value of the fee
        auth_badge: ResourceManager,
        //These are list of all the position long and short
        positions: HashMap<ComponentAddress, Position>,
        /// Counter for generating unique position IDs
        position_counter: u64,
        // Address of the price oracle component
        price_oracle: Global<DummyOracle>,
    }

    impl Phuturex {
        //When deploy

        pub fn instantiate_phuturex(
            token: Bucket,
            custom_fee: Decimal,
        ) -> (Global<phuturex::Phuturex>, FungibleBucket) {
            assert!(
                custom_fee >= Decimal::zero() && custom_fee <= Decimal::one(),
                "Invalid fee in thousandths"
            );
            assert!(
                !token.is_empty(),
                "You must pass in an initial supply of a token."
            );

            let auth_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
             .metadata(metadata! {
                init {
                    "name" => "Admin Badge",locked;
                    "symbol"  => "AB", locked;
                    "description"  => "This is an admin badge that has the authority to deposit or withdraw tokens in the pool", locked;
                }
            })
            .mint_initial_supply(1);

            let price_oracle: Global<DummyOracle> = DummyOracle::new(dec!(0), dec!(1));

            let component = Self {
                pool: Vault::with_bucket(token),
                fee: custom_fee,
                auth_badge: auth_badge.resource_manager(),
                positions: HashMap::new(),
                position_counter: 0,
                price_oracle: price_oracle,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();

            (component, auth_badge)
        }

        //auth can by done only by authorized address
        pub fn deposit(&mut self, auth: Proof, amount: Bucket) {
            assert!(
                auth.resource_address() == self.auth_badge.address(),
                "Unauthorized access"
            );
            self.pool.put(amount);
        }

        //auth can by done only by authorized address
        pub fn withdraw(&mut self, auth: Proof, amount: Decimal) -> Bucket {
            assert!(
                auth.resource_address() == self.auth_badge.address(),
                "Unauthorized access"
            );
            self.pool.take(amount)
        }

        //auth can by done only by authorized address
        pub fn change_fee(&mut self, auth: Proof, new_fee: Decimal) {
            assert!(
                auth.resource_address() == self.auth_badge.address(),
                "Unauthorized access"
            );
            assert!(
                new_fee >= dec!("0") && new_fee <= dec!("1"),
                "Invalid fee value"
            );
            self.fee = new_fee;
        }

        pub fn add_position(
            &mut self,
            account_address: ComponentAddress,
            side_str: String,
            leverage: Decimal,
            open_price: Decimal,
            order_quantity: Decimal,
        ) {
            let side = match side_str.as_str() {
                "long" => Side::Long,
                "short" => Side::Short,
                _ => Side::Short, // handle invalid side_int!
            };

            let position = Position {
                state: PositionState::Open,
                side,
                leverage,
                open_price,
                order_quantity,
            };
            self.positions.insert(account_address, position);
            self.position_counter += 1;
        }

        pub fn close_position(&mut self, account_address: ComponentAddress) -> Option<Position> {
            match self.positions.remove(&account_address) {
                Some(mut position) => {
                    position.state = PositionState::Closed;
                    // TODO: Add more logic to calculate the profit or lost, update to user's wallet

                    info!("Position closed for account: {:?}", account_address);
                    Some(position)
                }
                None => {
                    info!("No open position found for account: {:?}", account_address);
                    None
                }
            }
        }

        pub fn read_positions(&self) {
            info!("Number of positions: {}", self.positions.keys().len());
        }

        pub fn check_positions(&mut self) {
            let current_price = self.price_oracle.get_price();
            for (address, position) in &self.positions {
                let liq_price = position.calculate_liquidation_price();
                info!(
                    "Liq Price: {} // Current price: {}",
                    liq_price, current_price
                )
            }
        }
    }
}

#[blueprint]
mod dummy_oracle {
    struct DummyOracle {
        min_price: Decimal,
        max_price: Decimal,
    }

    impl DummyOracle {
        // Instantiate the oracle with a price range
        pub fn new(min_price: Decimal, max_price: Decimal) -> Global<dummy_oracle::DummyOracle> {
            Self {
                min_price,
                max_price,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn get_price(&self) -> Decimal {
            let tx_hash: Hash = Runtime::transaction_hash();
            let bytes: &[u8] = &tx_hash.as_ref();
            let mut array = [0u8; 16];
            array.copy_from_slice(&bytes[..16]);
            let num_from_tx_hash = u128::from_be_bytes(array);

            let num_to_100 = Decimal::from(num_from_tx_hash % 100u128);
            let price_range = self.max_price - self.min_price;
            let price = self.min_price + price_range * (num_to_100 / dec!(100));
            info!("Price: {}", price);
            price
        }
    }
}
