#![cfg(unix)]
#![cfg(feature = "signal")]
#![warn(rust_2018_idioms)]
#![feature(async_await)]

mod support;
use support::*;

#[tokio::test]
async fn twice() {
    let kind = SignalKind::user_defined1();
    let mut sig = signal(kind).expect("failed to get signal");

    for _ in 0..2 {
        send_signal(libc::SIGUSR1);

        let (item, sig_next) = with_timeout(sig.into_future()).await;
        assert_eq!(item, Some(()));

        sig = sig_next;
    }
}
