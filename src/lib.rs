#[cfg(feature = "views")] #[macro_use] extern crate dependent_view;
#[cfg(feature="views")] use dependent_view::rc::DependentRc;

use std::default::Default;
use std::rc::{Weak, Rc};
use std::convert::{AsRef, AsMut};


/// Trait representing a component that can be observed
/// # Note
/// It is not advisable to implement this trait directly. Instead, include
/// the observable mixin as a field in your struct, and implement `AsMut<ObservableMixin<M>>`
pub trait Observable<M:Clone> {
    fn register(&mut self, observer: Weak<Observer<M>>);
    fn notify_observers(&mut self, event: M);
}


/// Trait representing an observable object
pub trait Observer<M:Clone> {
    /// Optional field used to uniquely identify an observer - used to remove observers 
    /// if not provided, the only way to de-register an observer is to drop the underlying
    /// observer
    fn id(&self) -> Option<usize> { None }

    /// Used by the observable to notify the observer of changes
    fn notify(&self, event: M);
}


pub struct ObservableMixin<M:Clone> {
    observers: Vec<Weak<Observer<M>>>
}

impl<M:Clone> Default for ObservableMixin<M> {
    fn default() -> ObservableMixin<M> {
        ObservableMixin {
            observers: Vec::new()
        }
    }
}

impl <M:Clone, T: AsMut<ObservableMixin<M>>> Observable<M> for T {
    fn register(&mut self, observer: Weak<Observer<M>>) {
        self.as_mut().observers.push(observer);
    }

    fn notify_observers(&mut self, event: M) {
        self.as_mut().observers.retain(|observer| {
            if let Some(observer) = observer.upgrade() {
                observer.notify(event.clone());
                true
            } else {
                false
            }
        });
    }
}


#[cfg(feature="views")] 
impl<M: Clone> ObservableMixin<M> {
    pub fn register_dependable<T: Observer<M> + 'static>(&mut self, dependable: &mut DependentRc<T>) {
       self.observers.push(to_view!(dependable)); 
    }

    pub fn deregister_dependable<T: Observer<M> + 'static>(&mut self, dependable: &DependentRc<T>) {
        if let Some(id) = dependable.id() {
            // if the thing you are trying to remove has an id
            self.observers.retain(|observer| {
                if let Some(observer) = observer.upgrade() {
                    if let Some(o_id) = observer.id() {
                       // if the id's are the same, drop them, otherwise keep
                       o_id != id
                    } else {
                        // if the thing being observed doesn't have
                        // an id, it can't be removed this way - only
                        // when the underlying rc is dropped
                        true 
                    }
                } else {
                    // if it has died, remove it anyway
                    false
                }
            });
        }
    }
}

