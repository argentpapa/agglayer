use agglayer_types::CertificateIndex;

use crate::columns::PER_EPOCH_TRANSACTION_HASH_PER_CERTIFICATE_INDEX;

/// Column family for the transaction hash per certificate index in an epoch.
///
/// ## Column definition
///
/// | key                | value    |
/// | --                 | --       |
/// | `CertificateIndex` | `Hash`   |
pub struct TransactionHashPerCertificateIndexColumn;

impl crate::columns::ColumnSchema for TransactionHashPerCertificateIndexColumn {
    type Key = CertificateIndex;
    type Value = agglayer_types::Hash;

    const COLUMN_FAMILY_NAME: &'static str = PER_EPOCH_TRANSACTION_HASH_PER_CERTIFICATE_INDEX;
}