use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::time;

use std::time::{Duration, SystemTime};

const DASHBOARD_BUFFER_CHARACTERISTIC_UUID: u128 = 0x0001000150bf48a29d8a835aaa2fb179;
const DASHBOARD_WRITE_CHARACTERISTIC_UUID: u128 = 0x0001000250bf48a29d8a835aaa2fb179;

pub async fn search(central: Adapter) -> Option<Peripheral> {
    println!("Searching for bluetooth peripherals!");
    // start scanning for devices
    central.start_scan(ScanFilter::default()).await.unwrap();
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(5)).await;

    find_display(&central).await
}

async fn find_display(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Dashboard"))
        {
            return Some(p);
        }
    }
    None
}

pub async fn display_image(image: &[[u8; 32]]) {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    let per = search(central).await.expect("Did not find peripheral");

    // connect to the device
    per.connect().await.unwrap();

    // discover services and characteristics
    per.discover_services().await.unwrap();

    println!("Writing to display");

    transmit_to_peripheral(&per, image).await;
}

async fn transmit_to_peripheral(per: &Peripheral, data: &[[u8; 32]]) {
    let chars = per.characteristics();

    let buffer_char = chars
        .iter()
        .find(|c| c.uuid.as_u128() == DASHBOARD_BUFFER_CHARACTERISTIC_UUID)
        .unwrap();

    let write_char = chars
        .iter()
        .find(|c| c.uuid.as_u128() == DASHBOARD_WRITE_CHARACTERISTIC_UUID)
        .unwrap();

    let len = data.len();
    println!("Data chunk amount: {len}");
    let hundreth = len / 100;
    let now = SystemTime::now();
    for (i, packet) in data.iter().enumerate() {
        let write_type = if (i % 50) == 0 {
            WriteType::WithResponse
        } else {
            WriteType::WithoutResponse
        };
        per.write(buffer_char, packet, write_type).await.unwrap();

        if i % hundreth == 0 {
            println!("At {}%", i / hundreth);
            // if i / hundreth >= 5 {
            //     break;
            // }
        }
    }
    let duration = now.elapsed().unwrap();
    println!("Completed transmission of data");
    println!("Time taken: {}ms", duration.as_millis().to_string());

    per.write(write_char, &[1u8], WriteType::WithResponse)
        .await
        .unwrap();

    per.disconnect().await.unwrap();

    println!("Written to Display");
}
