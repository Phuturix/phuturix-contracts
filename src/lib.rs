use scrypto::prelude::*;

mod state;
use state::{Position, Side};

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
        auth_badge: FungibleVault,
        //These are list of all the position long and short
        positions: HashMap<u64, Position>,
        /// Counter for generating unique position IDs
        position_counter: u64,
    }

    impl Phuturex {
        //When deploy 
        pub fn instantiate_phuturex(token: Bucket, custom_fee: Decimal) ->  Global<phuturex::Phuturex> {
             assert!(
                custom_fee >= Decimal::zero() && custom_fee <= Decimal::one(),
                "Invalid fee in thousandths"
            );
            assert!(
                !token.is_empty(),
                "You must pass in an initial supply of a token."
            );

             let auth_badge = ResourceBuilder::new_fungible(OwnerRole::None)
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
                auth_badge: FungibleVault::with_bucket(auth_badge),
                positions: HashMap::new(),
                position_counter: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();


            component
        }
        //auth can by done only by authorized address
        pub fn deposit(){}
         //auth can by done only by authorized address
        pub fn withdraw(){}
       //auth can by done only by authorized address
        pub fn change_fee(){}

        pub fn add_position() {

        }

        pub fn close_position() {
            
        }

    }
}