mod server {pub mod server;}
mod client { pub mod client;}

fn main() {
    println!("Клиент: 1, Сервер: 2");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Ошибка при чтении ввода пользователя");
    let choice: usize = input.trim().parse().expect("Пожалуйста, введите номер");
    if choice == 2 {
        let _ = server::server::server();
    } else {
        let _ = client::client::client();
    }
}
