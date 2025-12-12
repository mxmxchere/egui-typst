fn main() {
    use kyte::{Compose, Delta, Transform};

    let before = Delta::new().insert("Hello World".to_owned(), ());

    let alice = Delta::new().retain(5, ()).insert(",".to_owned(), ());
    let bob = Delta::new().retain(11, ()).insert("!".to_owned(), ());

    let after = before
        .compose(alice.clone())
        .compose(alice.clone().transform(bob.clone(), true));
    //let s: String = after.into();
    //before.compose(bob).compose(bob.transform(alice, false)),
    /*   let d = Delta::new().insert("Hello World".to_owned(), ());

        let alice = Delta::new().retain(5, ()).insert(",".to_owned(), ());
        let bob = Delta::new().retain(11, ()).insert("!".to_owned(), ());

        d.compose(alice).compose(alice.transform(bob, true));

        //println!("{:?}", d);
    */
}
