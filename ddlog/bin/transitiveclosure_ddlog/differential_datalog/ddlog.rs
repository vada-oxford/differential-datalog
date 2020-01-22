use std::fmt::Debug;
use std::iter::Iterator;
use std::ops::Deref;

use crate::callback::Callback;
use crate::program::IdxId;
use crate::program::RelId;
use crate::program::Update;
use crate::program::Val;
use crate::record::UpdCmd;
use crate::valmap::DeltaMap;
use std::collections::btree_set::BTreeSet;

/// Convert to and from values/objects of a DDlog program.
pub trait DDlogConvert: Debug {
    type Value: Debug;

    /// Convert a `RelId` into its symbolic name.
    fn relid2name(rel_id: RelId) -> Option<&'static str>;

    /// Convert a `IdxId` into its symbolic name.
    fn indexid2name(idx_id: IdxId) -> Option<&'static str>;

    /// Convert an `UpdCmd` into an `Update`.
    fn updcmd2upd(upd_cmd: &UpdCmd) -> Result<Update<Self::Value>, String>;
}

/// A trait capturing program instantiation and handling of
/// transactions.
pub trait DDlog: Debug {
    type Convert: DDlogConvert<Value = Self::Value>;
    type Value: Val;

    /// Run the program.
    fn run<F>(workers: usize, do_store: bool, cb: F) -> Result<Self, String>
    where
        Self: Sized,
        F: Callback;

    /// Start a transaction.
    fn transaction_start(&self) -> Result<(), String>;

    /// Commit a transaction previously started using
    /// `transaction_start`, producing a map of deltas.
    fn transaction_commit_dump_changes(&self) -> Result<DeltaMap<Self::Value>, String>;

    /// Commit a transaction previously started using
    /// `transaction_start`.
    fn transaction_commit(&self) -> Result<(), String>;

    /// Roll back a transaction previously started using
    /// `transaction_start`.
    fn transaction_rollback(&self) -> Result<(), String>;

    /// Apply a set of updates.
    fn apply_updates<V, I>(&self, upds: I) -> Result<(), String>
    where
        V: Deref<Target = UpdCmd>,
        I: Iterator<Item = V>;

    /// Apply a set of updates.
    fn apply_valupdates<I>(&self, upds: I) -> Result<(), String>
    where
        I: Iterator<Item = Update<Self::Value>>;

    /// Apply a set of updates directly from the flatbuffer
    /// representation
    #[cfg(feature = "flatbuf")]
    fn apply_updates_from_flatbuf(&self, buf: &[u8]) -> Result<(), String>;

    /// Query index.  Returns all values associated with the given key in the index.
    fn query_index(&self, index: IdxId, key: Self::Value) -> Result<BTreeSet<Self::Value>, String>;

    /// Similar to `query_index`, but extracts query from a flatbuffer.
    #[cfg(feature = "flatbuf")]
    fn query_index_from_flatbuf(&self, buf: &[u8]) -> Result<BTreeSet<Self::Value>, String>;

    /// Dump all values in an index.
    fn dump_index(&self, index: IdxId) -> Result<BTreeSet<Self::Value>, String>;

    /// Stop the program.
    fn stop(&mut self) -> Result<(), String>;
}
