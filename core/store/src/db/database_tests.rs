#![cfg(test)]
//! Set of tests over the 'Database' interface, that we can run over multiple implementations
//! to make sure that they are working correctly.
use crate::db::{DBTransaction, TestDB};
use crate::{DBCol, NodeStorage};

/// Tests the behavior of the iterators. Iterators don't really work over cold storage, so we're not testing it here.
#[test]
fn test_db_iter() {
    let (_tmp_dir, opener) = NodeStorage::test_opener();
    let store = opener.open().unwrap().get_hot_store();
    let test_db = TestDB::new();
    for db in [&*test_db as _, store.database()] {
        let mut transaction = DBTransaction::new();
        transaction.insert(DBCol::Block, "a".into(), "val_a".into());
        transaction.insert(DBCol::Block, "aa".into(), "val_aa".into());
        transaction.insert(DBCol::Block, "aa1".into(), "val_aa1".into());
        transaction.insert(DBCol::Block, "bb1".into(), "val_bb1".into());
        transaction.insert(DBCol::Block, "cc1".into(), "val_cc1".into());
        db.write(transaction).unwrap();

        let keys: Vec<_> = db
            .iter(DBCol::Block)
            .map(|data| String::from_utf8(data.unwrap().0.to_vec()).unwrap())
            .collect();
        assert_eq!(keys, vec!["a", "aa", "aa1", "bb1", "cc1"]);

        let keys: Vec<_> = db
            .iter_range(DBCol::Block, Some("aa".as_bytes()), Some("bb1".as_bytes()))
            .map(|data| String::from_utf8(data.unwrap().0.to_vec()).unwrap())
            .collect();
        assert_eq!(keys, vec!["aa", "aa1"]);
    }
}
