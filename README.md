# observable 
### WARNING - this is an unstable crate that may change at any time. Use at your own risk
Idiomatic Observables for rust.


```rust
#[derive(Debug, Clone)]
pub enum Event ( Example1, Example2, Example3 };

pub struct Game {
  ...
  observable: ObservableMixin<Event>
  ...
}
impl AsMut<ObservableMixin<Event>> for Game { fn as_mut(&mut self) -> &mut
ObservableMixin<Event> { &self.observable }
/// Game is now an observable struct
```

To create an observer, we need a struct that implements `Observer<Event>`
```rust
struct GameListener { }

impl Observer<Event> for GameListener {
  pub fn notify(&self, event: Event) {
        println!("Got event: {:?}", event);
  }
}
```
To add an observer, we can need to provide a weak reference to an observable object:
```rust
    let mut game = Game::new();
    /// we need the clone here to upcast to Rc<Trait>
    let observer : Rc<Observer<Event>> = Rc::new(GameListener{}).clone();
    game.register(Rc::downgrade(&observer));
```
Or, if using the 'views' feature, with my other crate 'dependent_view',
```rust
    let mut game = Game::new();
    let mut observer : DependentRc<GameListener> = DependentRc::new(GameListener{});
    game.as_mut().register_dependable(&mut observer);
    // notice here how we get to keep the level of specificity with regards to our observer
    // rather than having to upcast our only reference
```

And finally, we can send events to our observers:
```rust
    game.notify_observers(Event::Example1);
```

