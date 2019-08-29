use std::time::Duration;
use log::debug;
use riker::actors::*;

struct MyActor;

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self,
            _ctx: &Context<String>,
            msg: String,
            _sender: Sender) {

        debug!("Received: {}", msg);
    }
}

// provide factory and props functions
impl MyActor {
    fn actor() -> Self {
        MyActor
    }

    fn props() -> BoxActorProd<MyActor> {
        Props::new(MyActor::actor)
    }
}

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let props = MyActor::props();
    let my_actor = sys.actor_of(props, "my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(500));
}
