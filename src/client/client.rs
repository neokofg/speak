use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{StreamConfig};
use tokio::sync::{mpsc};
use tokio::net::UdpSocket;
use std::error::Error;
use std::io::{self, Write};

fn list_devices_and_select() -> Option<cpal::Device> {
    let host = cpal::default_host();
    let input_devices = host.input_devices().expect("Ошибка при получении списка входных устройств");
    println!("Выберите входное устройство:");

    let device_list: Vec<_> = input_devices.map(|d| d.name().unwrap()).collect();
    for (index, name) in device_list.iter().enumerate() {
        println!("{}: {}", index + 1, name);
    }

    print!("Введите номер устройства: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка при чтении ввода пользователя");

    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= device_list.len() {
            return host.input_devices().ok()?.nth(choice - 1);
        }
    }
    eprintln!("Неверный ввод");
    None
}
#[tokio::main]
pub async fn client() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);
    let device = list_devices_and_select().expect("Не удалось выбрать устройство");
    let config: StreamConfig = device.default_input_config()?.into();

    let err_fn = |err| eprintln!("Ошибка захвата аудио: {}", err);

    let stream = device.build_input_stream(&config, move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut buf = Vec::new();
        for &sample in data.iter() {
            buf.extend_from_slice(&sample.to_le_bytes());
        }
        println!("Захват аудио: {} байт", buf.len());
        if let Err(e) = tx.blocking_send(buf) {
            eprintln!("Не удалось отправить данные: {}", e);
        }
    }, err_fn, None)?;

    stream.play()?;
    println!("Запись началась...");

    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    socket.connect("127.0.0.1:8080").await?;
    println!("Сокет подключен к серверу");

    let mut buf = [0u8; 4096];
    loop {
        tokio::select! {
            result = socket.recv(&mut buf) => {
                match result {
                    Ok(size) => {
                        println!("Получено {} байт", size);
                    },
                    Err(e) => eprintln!("Ошибка приема данных: {}", e),
                }
            }
            Some(data) = rx.recv() => {
                let chunk_size = 512;
                for chunk in data.chunks(chunk_size) {
                    if let Err(e) = socket.send(chunk).await {
                        eprintln!("Ошибка отправки аудио: {}", e);
                    } else {
                        println!("Аудио успешно отправлено: {} байт", chunk.len());
                    }
                }
            }
        }
    }

    Ok(())
}