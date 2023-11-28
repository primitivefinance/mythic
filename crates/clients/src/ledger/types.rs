use std::fmt;
// All available instructions for ledger ethereum app
#[repr(u8)]
pub enum Instruction {
    GetPublicKey = 0x02,
    Sign = 0x04, // sign transaction
    GetAppConfiguration = 0x06,
    SignPersonalMessage = 0x08,
    ProvideErc20TokenInformation = 0x0A,
    SignEip712Message = 0x0C,
    GetEth2PublicKey = 0x0E,
    SetEth2WithdrawalIndex = 0x10,
    SetExternalPlugin = 0x12,
    ProvideNftInformation = 0x14,
    SetPlugin = 0x16,
    PerformPrivacyOperation = 0x18,
    Eip712StructDef = 0x1A,
    Eip712StructImpl = 0x1C,
    Eip712Filtering = 0x1E,
    EnsGetChallenge = 0x20,
    EnsProvideInfo = 0x22,
}

// P1 and P2
pub const P1_FIRST: u8 = 0x00;
pub const P1_NON_CONFIRM: u8 = 0x00;
pub const P1_MORE: u8 = 0x80;
pub const P2_NO_CHAINCODE: u8 = 0x00;

#[derive(Debug)]
pub enum LedgerClienError {
    LedgerError(coins_ledger::LedgerError),
    CommandError(String),
    // ... other error variants ...
}
impl From<coins_ledger::LedgerError> for LedgerClienError {
    fn from(err: coins_ledger::LedgerError) -> LedgerClienError {
        LedgerClienError::LedgerError(err)
    }
}

#[derive(Clone, Debug)]
/// Ledger wallet type
pub enum DerivationType {
    /// Ledger Live-generated HD path
    LedgerLive(usize),
    /// Legacy generated HD Path
    Legacy(usize),
    /// Any other path
    Other(String),
}

impl fmt::Display for DerivationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                DerivationType::Legacy(index) => format!("m/44'/60'/0'/{index}"), /* i believe this is most common */
                DerivationType::LedgerLive(index) => format!("m/44'/60'/{index}'/0/0"),
                DerivationType::Other(inner) => inner.to_owned(),
            }
        )
    }
}
