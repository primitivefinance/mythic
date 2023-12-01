pub mod types;
use alloy_primitives::{hex, Address};
use coins_ledger::{
    common::{APDUData, APDUResponseCodes},
    errors::LedgerError,
    transports::LedgerAsync,
    APDUCommand, Ledger,
};
use ethers::types::{Signature, TransactionRequest, U256};

use self::types::*;

#[derive(Debug)]
pub struct LedgerClient {
    ledger: Ledger,
    derivation: DerivationType,
}

impl LedgerClient {
    pub async fn new_connection(derivation: DerivationType) -> Self {
        Self {
            ledger: Ledger::init().await.unwrap(),
            derivation,
        }
    }

    /// Get the account which corresponds to our derivation path
    pub async fn get_address(&self) -> Result<Address, LedgerError> {
        self.get_address_with_path_transport().await
    }

    async fn get_address_with_path_transport(&self) -> Result<Address, LedgerError> {
        let data = APDUData::new(&Self::path_to_bytes(&self.derivation));

        let command = APDUCommand {
            ins: Instruction::GetPublicKey as u8,
            p1: P1_NON_CONFIRM,
            p2: P2_NO_CHAINCODE,
            data,
            response_len: None,
        };

        tracing::debug!("Dispatching get_address request to ethereum app");
        let answer = self.ledger.exchange(&command).await?;
        let result = answer.data().unwrap();

        let address = {
            // extract the address from the response
            let offset = 1 + result[0] as usize;
            let address_str = &result[offset + 1..offset + 1 + result[offset] as usize];
            let mut address = [0; 20];
            address.copy_from_slice(&hex::decode(address_str).unwrap());
            Address::from(address)
        };
        tracing::debug!(?address, "Received address from device");
        Ok(address)
    }

    /// Returns the semver of the Ethereum ledger app
    pub async fn version(&self) -> Result<String, LedgerClienError> {
        let command = APDUCommand {
            ins: Instruction::GetAppConfiguration as u8,
            p1: P1_NON_CONFIRM,
            p2: P2_NO_CHAINCODE,
            data: APDUData::new(&[]),
            response_len: None,
        };

        tracing::debug!("Dispatching get_version");
        let answer = self.ledger.exchange(&command).await?;
        // todo handle this unwrap
        let result = answer.data().unwrap();
        if result.len() < 4 {
            return Err(LedgerClienError::LedgerError(
                coins_ledger::LedgerError::BadRetcode(APDUResponseCodes::OutputBufferTooSmall),
            ));
        }
        let version = format!("{}.{}.{}", result[1], result[2], result[3]);
        tracing::debug!(version, "Retrieved version from device");
        Ok(version)
    }

    /// Signs an Ethereum transaction (requires confirmation on the ledger)
    pub async fn sign_tx(&self, tx: &TransactionRequest) -> Result<Signature, LedgerClienError> {
        let mut payload = Self::path_to_bytes(&self.derivation);
        payload.extend_from_slice(&tx.rlp());
        let signature = self.sign_encoded_tx(&payload).await?;
        Ok(signature)
    }

    pub async fn get_app_configuration(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn sign_personal_message(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn provide_erc20_token_information(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn sign_eip_712_message(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn get_eth2_public_key(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn set_eth2_withdrawal_index(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn provide_nft_information(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn set_plugin(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn perform_privacy_operation(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn eip712_struct_def(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn eip712_struct_impl(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn eip712_filtering(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn ens_get_challenge(&self) -> Result<(), LedgerError> {
        todo!()
    }

    pub async fn ens_provide_info(&self) -> Result<(), LedgerError> {
        todo!()
    }

    // helper which converts a derivation path to bytes
    fn path_to_bytes(derivation: &DerivationType) -> Vec<u8> {
        let derivation = derivation.to_string();
        let elements = derivation.split('/').skip(1).collect::<Vec<_>>();
        let depth = elements.len();

        let mut bytes = vec![depth as u8];
        for derivation_index in elements {
            let hardened = derivation_index.contains('\'');
            let mut index = derivation_index.replace('\'', "").parse::<u32>().unwrap();
            if hardened {
                index |= 0x80000000;
            }

            bytes.extend(index.to_be_bytes());
        }
        bytes
    }
    // Helper to get the currently opened app on the ledger
    async fn get_opened_app(&self) -> Result<String, LedgerClienError> {
        let command = APDUCommand {
            ins: 0x01,
            p1: 0x00,
            p2: 0x00,
            data: APDUData::new(&[]),
            response_len: None,
        };

        let result = self.ledger.exchange(&command).await?;
        let result = result.data().unwrap();
        let len: usize = result[1].into();
        Ok(String::from_utf8(result[2..2 + len].to_vec()).unwrap())
    }

    // Helper function for signing either transaction data, personal messages or
    // EIP712 derived structs
    pub async fn sign_encoded_tx(
        &self,
        encoded_tx: &Vec<u8>,
    ) -> Result<Signature, LedgerClienError> {
        if encoded_tx.is_empty() {
            return Err(LedgerClienError::CommandError(
                ("Payload is empty").to_string(),
            ));
        }
        let command = APDUCommand {
            ins: Instruction::Sign as u8,
            p1: P1_FIRST,
            p2: P2_NO_CHAINCODE,
            data: APDUData::new(encoded_tx),
            response_len: None,
        };

        let result = self.ledger.exchange(&command).await?;
        if result.len() < 65 {
            return Err(LedgerClienError::LedgerError(
                LedgerError::ResponseTooShort(vec![result.len() as u8]),
            ));
        }
        let result = result.data().unwrap();
        // todo handle this unwrap better
        if result.is_empty() {
            if self.get_opened_app().await? == "Ethereum" {
                return Err(LedgerClienError::CommandError("Canceled".to_owned()));
            } else {
                return Err(LedgerClienError::CommandError(
                    "Please open Ethereum app on
        Ledger"
                        .to_owned(),
                ));
            }
        }
        let v: u64 = result[0].into();
        let r = &result[1..33];
        let s = &result[33..65];

        let sig = Signature {
            r: U256::from_big_endian(r),
            s: U256::from_big_endian(s),
            v,
        };
        Ok(sig)
    }
}

/// These tests will fail if you don't have a ledger connected
/// They should probably be ignored in CI and ran manually
mod tests {
    #![allow(unused_imports)]
    use super::*;
    // So I was initially trying to run this in it's own separate test as a unit
    // test but i think it was being run in it's own thread because it was
    // failing Which makes sense because a lock is obtained on the device when
    // being interacted with This means that we have to be careful to run the
    // ledger logic asynchronously
    #[tokio::test]
    async fn ledger_tests() {
        // test connecting: this doesn't require that the user has the ethereum ledger
        // application open
        let ledger = LedgerClient::new_connection(DerivationType::LedgerLive(0)).await;
        println!("Connected to ledger");

        // after this all other commands require that the ethereum ledger application is
        // User enters pin and then opens the ethereum ledger application

        // test getting address
        let address = ledger.get_address().await.unwrap();
        println!("Address: {}", address);

        // test getting version
        let version = ledger.version().await.unwrap();
        println!("Got Ethereum ledger application version {}", version);

        let tx = TransactionRequest::default();

        // This currently correctly prompts the user to review this transaction
        let sig = ledger.sign_tx(&tx).await.unwrap();
        println!("Got signature: {:?}", sig);
        // test signing a transaction
        ledger.ledger.close();
    }
}
