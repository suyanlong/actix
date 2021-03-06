extern crate actix;
extern crate futures;
extern crate tokio_core;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use futures::{future, Future};
use tokio_core::reactor::Timeout;
use actix::prelude::*;

struct Die;

impl ResponseType for Die {
    type Item = ();
    type Error = ();
}


struct MyActor(Arc<AtomicUsize>, Arc<AtomicUsize>);

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<MyActor>) {
        let n = self.0.load(Ordering::Relaxed);
        self.0.store(n+1, Ordering::Relaxed);
    }
}
impl actix::Supervised for MyActor {
    fn restarting(&mut self, _: &mut actix::Context<MyActor>) {
        let n = self.1.load(Ordering::Relaxed);
        self.1.store(n+1, Ordering::Relaxed);
    }
}

impl actix::Handler<Die> for MyActor {
    type Result = ();

    fn handle(&mut self, _: Die, ctx: &mut actix::Context<MyActor>) {
        ctx.stop();
    }
}


#[test]
fn test_supervisor() {
    let sys = System::new("test");

    let starts = Arc::new(AtomicUsize::new(0));
    let restarts = Arc::new(AtomicUsize::new(0));
    let starts2 = Arc::clone(&starts);
    let restarts2 = Arc::clone(&restarts);

    let (addr, _) = actix::Supervisor::start(false, move|_| MyActor(starts2, restarts2));

    addr.send(Die);

    Arbiter::handle().spawn(
        Timeout::new(Duration::new(0, 100), Arbiter::handle()).unwrap()
            .then(|_| {
                Arbiter::system().send(actix::msgs::SystemExit(0));
                future::result(Ok(()))
            })
    );

    sys.run();
    assert_eq!(starts.load(Ordering::Relaxed), 2);
    assert_eq!(restarts.load(Ordering::Relaxed), 1);
}

#[test]
fn test_supervisor_lazy() {
    let sys = System::new("test");

    let starts = Arc::new(AtomicUsize::new(0));
    let restarts = Arc::new(AtomicUsize::new(0));
    let starts2 = Arc::clone(&starts);
    let restarts2 = Arc::clone(&restarts);

    let (addr, _) = actix::Supervisor::start(true, move|_| MyActor(starts2, restarts2));

    // ref to supervisor, otherwise it would exit
    let _super_addr = addr.clone();

    let starts3 = Arc::clone(&starts);
    Arbiter::handle().spawn_fn(move || {
        assert_eq!(starts3.load(Ordering::Relaxed), 0);
        addr.send(Die);

        Timeout::new(Duration::new(0, 100), Arbiter::handle()).unwrap()
            .then(|_| {
                Arbiter::system().send(actix::msgs::SystemExit(0));
                future::result(Ok(()))
            })
    });

    sys.run();
    assert_eq!(starts.load(Ordering::Relaxed), 2);
    assert_eq!(restarts.load(Ordering::Relaxed), 1);
}

#[test]
fn test_supervisor_upgrade_address() {
    let sys = System::new("test");

    let starts = Arc::new(AtomicUsize::new(0));
    let restarts = Arc::new(AtomicUsize::new(0));
    let starts2 = Arc::clone(&starts);
    let restarts2 = Arc::clone(&restarts);

    // lazy supervisor
    let (addr, _) = actix::Supervisor::start(true, move|_| MyActor(starts2, restarts2));

    Arbiter::handle().spawn_fn(move || {
        // upgrade address to SyncAddress
        Arbiter::handle().spawn(addr.upgrade().then(|res| {
            res.unwrap().send(Die);
            future::result(Ok(()))
        }));

        Timeout::new(Duration::new(0, 100), Arbiter::handle()).unwrap()
            .then(|_| {
                Arbiter::system().send(actix::msgs::SystemExit(0));
                future::result(Ok(()))
            })
    });

    sys.run();
    assert_eq!(starts.load(Ordering::Relaxed), 1);
    assert_eq!(restarts.load(Ordering::Relaxed), 0);
}
