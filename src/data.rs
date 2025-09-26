use crate::errors::KrakenError;
use miette::{ Result};

pub mod make;

pub enum TransactionKind {
    DEPOSIT = 0,
    WITHDRAW = 1,
    DISPUTE = 2,
    RESOLVE = 3,
    CHARGEBACK = 4
}

impl Into<u8> for TransactionKind {
    fn into(self) -> usize {
        match self {
            TransactionKind::DEPOSIT => 0,
            TransactionKind::WITHDRAW => 1,
            TransactionKind::DISPUTE => 2,
            TransactionKind::RESOLVE => 3,
            TransactionKind::CHARGEBACK => 4,
        }
    }
}

/// Attempt to match an int in u8 form to the type of Transaction. [0-4] is small enough to fit
/// into a u8, so no need to waste memory on a larger datatype.
impl TryFrom<u8> for TransactionKind {
    type Error = KrakenError;

    fn try_from(v: u8) -> Result<Self, KrakenError> {
        match v {
            0 => Ok(TransactionKind::DEPOSIT),
            1 => Ok(TransactionKind::WITHDRAW),
            2 => Ok(TransactionKind::DISPUTE),
            3 => Ok(TransactionKind::RESOLVE),
            4 => Ok(TransactionKind::CHARGEBACK),
            _ => Err(KrakenError::EnumError)
        }
    }
}

/// Match against a String-proper. Don't manipulate it or convert to lower-case, as we're assuming
/// everything is formatted correctly. Converting to lowercase allocates a bunch and wastes
/// time/memory.
impl TryFrom<String> for TransactionKind {
    type Error = KrakenError;
    fn try_from(v: String) -> Result<Self, KrakenError> {
        match v.as_str() {
            "deposit" => Ok(TransactionKind::DEPOSIT),
            "withdraw" => Ok(TransactionKind::WITHDRAW),
            "dispute" => Ok(TransactionKind::DISPUTE),
            "resolve" => Ok(TransactionKind::RESOLVE),
            "chargeback" => Ok(TransactionKind::CHARGEBACK),
            _ => Err(KrakenError::EnumError)
        }
    }
}

/// Match against a str ref. Don't convert to lower-case as we assume the input is clean.
impl TryFrom<&str> for TransactionKind {
    type Error = KrakenError;
    fn try_from(v: &str) -> Result<Self, KrakenError> {
        match v {
            "deposit" => Ok(TransactionKind::DEPOSIT),
            "withdraw" => Ok(TransactionKind::WITHDRAW),
            "dispute" => Ok(TransactionKind::DISPUTE),
            "resolve" => Ok(TransactionKind::RESOLVE),
            "chargeback" => Ok(TransactionKind::CHARGEBACK),
            _ => Err(KrakenError::EnumError)
        }
    }
}

pub trait Transaction {

    fn get_kind(&self) -> u8; // Get type of transaction.
    fn get_tx(&self) -> u32; // Get transaction ID.
    fn get_amount(&self) -> Option<f64>; // Get the amount of the transaction. Some types will be None.
    fn get_client(&self) -> u16; // Get ID of the client associated with the tx.
}

pub struct DepositTransaction {
    kind: u8,
    tx: u32,
    amount: f64,
    client: u16
}

impl Into<DisputeTransaction> for DepositTransaction {
    fn into(self) -> DisputeTransaction {
        DisputeTransaction {
            kind: self.kind,
            tx: self.tx,
            client: self.client,
        }
    }
}

impl Transaction for DepositTransaction {
    fn get_kind(&self) -> u8 {
        self.kind
    }

    fn get_tx(&self) -> u32 {
        self.tx
    }

    fn get_amount(&self) -> Option<f64> {
        Some(self.amount)
    }

    fn get_client(&self) -> u16 {
        self.client
    }
}

pub struct WithdrawTransaction {
    kind: u8,
    tx: u32,
    amount: f64,
    client: u16
}

impl Into<DisputeTransaction> for WithdrawTransaction {
    fn into(self) -> DisputeTransaction {
        DisputeTransaction {
            kind: self.kind,
            tx: self.tx,
            client: self.client,
        }
    }
}

impl Transaction for WithdrawTransaction {
    fn get_kind(&self) -> u8 {
        self.kind
    }

    fn get_tx(&self) -> u32 {
        self.tx
    }

    fn get_amount(&self) -> Option<f64> {
        Some(self.amount)
    }

    fn get_client(&self) -> u16 {
        self.client
    }
}

pub struct DisputeTransaction {
    kind: u8,
    tx: u32,
    client: u16
}

impl Into<ResolveTransaction> for DisputeTransaction {
    fn into(self) -> ResolveTransaction {
        ResolveTransaction {
            kind: self.kind,
            tx: self.tx,
            client: self.client,
        }
    }
}

impl Into<ChargebackTransaction> for DisputeTransaction {
    fn into(self) -> ChargebackTransaction {
        ChargebackTransaction {
            kind: self.kind,
            tx: self.tx,
            client: self.client,
        }
    }
}

impl Transaction for DisputeTransaction {
    fn get_kind(&self) -> u8 {
        self.kind
    }
    fn get_tx(&self) -> u32 {
        self.tx
    }

    fn get_amount(&self) -> Option<f64> {
        None
    }

    fn get_client(&self) -> u16 {
        self.client
    }
}

pub struct ChargebackTransaction {
    kind: u8,
    tx: u32,
    client: u16
}

impl Transaction for ChargebackTransaction {
    fn get_kind(&self) -> u8 {
        self.kind
    }

    fn get_tx(&self) -> u32 {
        self.tx
    }

    fn get_amount(&self) -> Option<f64> {
        None
    }

    fn get_client(&self) -> u16 {
        self.client
    }
}

pub struct ResolveTransaction {
    kind: u8,
    tx: u32,
    client: u16
}

impl Transaction for ResolveTransaction {
    fn get_kind(&self) -> u8 {
        self.kind
    }

    fn get_tx(&self) -> u32 {
        self.tx
    }

    fn get_amount(&self) -> Option<f64> {
        None
    }

    fn get_client(&self) -> u16 {
        self.client
    }
}