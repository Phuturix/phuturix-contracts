use scrypto::prelude::*;

mod state;
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
    }

    impl Phuturex {
        //When deploy
        pub fn instantiate_phuturex(
            token: Bucket,
            custom_fee: Decimal,
        ) -> (Global<phuturex::Phuturex>, FungibleBucket){
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

            let component = Self {
                pool: Vault::with_bucket(token),
                fee: custom_fee,
                auth_badge: auth_badge.resource_manager(),
                positions: HashMap::new(),
                position_counter: 0,
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
            borrowed_size: Decimal,
            collateral_size: Decimal,
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
                borrowed_size,
                collateral_size,
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
                },
                None => {
                    info!("No open position found for account: {:?}", account_address);
                    None
                }
            }
        }

        pub fn read_positions(&self) {
            info!("Number of positions: {}", self.positions.keys().len());
        }
    }
}
