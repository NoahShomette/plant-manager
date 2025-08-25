//! Core app features and settings

use std::{collections::BTreeMap, ops::Bound};

use chrono::Utc;
use serde::{Deserialize, Serialize};

pub mod plant;

/// An item that contains a date history
#[derive(Debug, Hash, Serialize, Deserialize, Clone, PartialEq)]
pub struct HistoryItem<T> {
    pub item: BTreeMap<i64, T>,
}

impl<T> HistoryItem<T> {
    pub fn new(item: T) -> HistoryItem<T> {
        let mut map = BTreeMap::new();
        map.insert(Utc::now().naive_utc().and_utc().timestamp(), item);
        HistoryItem { item: map }
    }

    pub fn new_with_timestamp(item: T, timestamp: i64) -> HistoryItem<T> {
        let mut map = BTreeMap::new();
        map.insert(timestamp, item);
        HistoryItem { item: map }
    }

    pub fn state(&self) -> Option<(&i64, &T)> {
        self.item
            .range((
                Bound::Unbounded,
                Bound::Included(&Utc::now().naive_utc().and_utc().timestamp()),
            ))
            .next_back()
    }

    // pub fn previous_state(&self) -> Option<(&i64, &T)> {
    //     self.item
    //         .range((
    //             Bound::Unbounded,
    //             Bound::Included(&Utc::now().naive_utc().and_utc().timestamp()),
    //         ))
    //         .into_iter()
    // }
}

/// An item that contains a date history that is infallible. Always has a default
#[derive(Debug, Hash, Serialize, Deserialize, Clone, PartialEq)]
pub struct InfallibleHistoryItem<T: Clone> {
    pub default_timestamp: i64,
    pub default: T,
    pub item: HistoryItem<T>,
}

impl<T: Clone> InfallibleHistoryItem<T> {
    pub fn new(item: T) -> InfallibleHistoryItem<T> {
        let now = Utc::now().naive_utc().and_utc().timestamp();
        let history_item = HistoryItem::new_with_timestamp(item.clone(), now);
        InfallibleHistoryItem {
            item: history_item,
            default_timestamp: now,
            default: item,
        }
    }

    pub fn new_with_timestamp(item: T, timestamp: i64) -> InfallibleHistoryItem<T> {
        let history_item = HistoryItem::new_with_timestamp(item.clone(), timestamp);
        InfallibleHistoryItem {
            item: history_item,
            default_timestamp: timestamp,
            default: item,
        }
    }

    pub fn state(&self) -> (&i64, &T) {
        match self.item.state() {
            Some(state) => state,
            None => (&self.default_timestamp, &self.default),
        }
    }

    // pub fn previous_state(&self) -> Option<(&i64, &T)> {
    //     self.item
    //         .range((
    //             Bound::Unbounded,
    //             Bound::Included(&Utc::now().naive_utc().and_utc().timestamp()),
    //         ))
    //         .into_iter()
    // }
}
