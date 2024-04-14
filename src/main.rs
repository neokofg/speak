use tokio::task::spawn_blocking;
mod server {pub mod server;}
mod client { pub mod client;}
slint::include_modules!();
fn main() {
    // println!("Клиент: 1, Сервер: 2");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Ошибка при чтении ввода пользователя");
    // let choice: usize = input.trim().parse().expect("Пожалуйста, введите номер");
    // if choice == 2 {
    //     let _ = server::server::server();
    // } else {
    //     let _ = client::client::client();
    // }
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    
    runtime.block_on(async {
        spawn_blocking(run_ui)
            .await
            .unwrap()
            .unwrap()
    });
}

fn run_ui() -> Result<(), slint::PlatformError> {
    let ui =  AppWindow::new()?;
    ui.on_run_server(move || {
        server::server::server();
    });
    ui.on_run_client(move || {
        client::client::client();
    });

    ui.run()
}