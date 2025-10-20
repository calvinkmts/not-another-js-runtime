use runtime_core::RuntimeHandle;

#[tokio::test]
async fn it_runs_inline_js() {
    let mut runtime = RuntimeHandle::new_with_loader().unwrap();
    runtime
        .js_runtime_mut()
        .execute_script("<inline>", r#"console.log("ok")"#)
        .unwrap();
}
