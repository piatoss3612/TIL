use candid::{CandidType, Principal};
use ic_cdk::{caller, notify, update};
use serde::Deserialize;
use std::{cell::RefCell, collections::BTreeMap};

type SubscriberStore = BTreeMap<Principal, Subscriber>;

thread_local! {
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[update]
fn subscribe(subscriber: Subscriber) {
    let subscriber_principal_id = caller();
    SUBSCRIBERS.with(|store| {
        store
            .borrow_mut()
            .insert(subscriber_principal_id, subscriber);
    });
}

#[update]
async fn publish(counter: Counter) {
    SUBSCRIBERS.with(|store| {
        for (k, v) in store.borrow().iter() {
            if v.topic == counter.topic {
                let _call_result: Result<(), _> = notify(*k, "update_count", (&counter,));
            }
        }
    })
}
