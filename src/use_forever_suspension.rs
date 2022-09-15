use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
    use_mut_ref, Callback,
};

#[hook]
pub fn use_forever_suspension() -> SuspensionResult<()> {
    let forever_suspend = use_forever_suspend();

    Err(forever_suspend.emit(()))
}

#[hook]
pub fn use_forever_suspend() -> Callback<(), Suspension> {
    let suspension_handle_ref = use_mut_ref(|| None);

    Callback::from(move |_| -> Suspension {
        let (s, handle) = Suspension::new();
        *suspension_handle_ref.borrow_mut() = Some(handle);
        log::info!("🚀 use_forever_suspend::Suspending");
        s
    })
}
