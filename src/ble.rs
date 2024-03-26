//! Serves a Bluetooth GATT application using the callback programming model.

use bluer::{
    adv::Advertisement,
    gatt::local::{
        Application, Characteristic, CharacteristicRead, CharacteristicWrite,
        CharacteristicWriteMethod, Service,
    },
};
use futures::FutureExt;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};

use crate::amp::AmpConfig;

pub async fn app(_amp_config: Arc<RwLock<AmpConfig>>) -> bluer::Result<()> {
    const MANUFACTURER_ID: u16 = 0xf00d;
    const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xFEEDC0DE);
    const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);

    eprintln!("service: {}", &SERVICE_UUID);
    eprintln!("char: {}", &CHARACTERISTIC_UUID);

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some("gatt_server".to_string()),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(le_advertisement).await?;

    println!(
        "Serving GATT service on Bluetooth adapter {}",
        adapter.name()
    );
    // let value = Arc::new(Mutex::new(vec![0x10, 0x01, 0x01, 0x10]));
    // let value_read = value.clone();
    // let value_write = value.clone();
    // let value_notify = value.clone();
    let value_read = _amp_config.clone();
    let value_write = _amp_config.clone();
    // let value_notify = _amp_config.clone();
    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![Characteristic {
                uuid: CHARACTERISTIC_UUID,
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        let value = value_read.clone();
                        async move {
                            let value: &AmpConfig = &*value.read().unwrap();
                            println!("Read request {:?} with value {:?}", &req, &value);
                            let bytes = serde_json::to_vec(value).unwrap();
                            Ok(bytes)
                        }
                        .boxed()
                    }),
                    ..Default::default()
                }),
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Fun(Box::new(move |new_value, req| {
                        let value = value_write.clone();
                        async move {
                            let amp_config: AmpConfig = serde_json::from_slice(&new_value).unwrap();
                            println!("Write request {:?} with value {:x?}", &req, &amp_config);
                            let mut value = value.write().unwrap();
                            *value = amp_config;
                            Ok(())
                        }
                        .boxed()
                    })),
                    ..Default::default()
                }),
                notify: None,
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
