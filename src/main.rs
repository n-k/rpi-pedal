use alsa::{pcm::*, Direction, ValueOr, PCM};
use ringbuf::HeapRb;
use std::thread;

fn start_pcm_stream(device: &str, dir: Direction) -> anyhow::Result<PCM> {
    let pcm = PCM::new(device, dir, false)?;
    {
        // For this example, we assume 44100Hz, one channel, 16 bit audio.
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(1)?;
        hwp.set_rate(44100, ValueOr::Nearest)?;
        hwp.set_format(Format::u32())?;
        hwp.set_access(Access::RWInterleaved)?;
        pcm.hw_params(&hwp)?;
    }
    pcm.start()?;
    Ok(pcm)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rb = HeapRb::<u32>::new(8192 * 10);
    let (mut prod, mut cons) = rb.split();

    thread::spawn(move || {
        let pcm_in = start_pcm_stream("default:1", Direction::Capture).unwrap();
        let io_in = pcm_in.io_u32().unwrap();
        let mut buf = [0u32; 1024];
        loop {
            io_in.readi(&mut buf).unwrap();
            prod.push_slice(&buf);
        }
    });
    thread::spawn(move || {
        let pcm_out = start_pcm_stream("default:1", Direction::Playback).unwrap();
        let io_out = pcm_out.io_u32().unwrap();
        let mut buf = [0u32; 1024];
        loop {
            cons.pop_slice(&mut buf);
            io_out.writei(&buf).unwrap();
        }
    });

    loop {
        thread::sleep(std::time::Duration::from_millis(500));
    }

    // build and server GATT application
    // GattApplication::new()
    //     .await
    //     .service(CounterService::default())
    //     .await
    //     .advertise()
    //     .serve()
    //     .await?;

    Ok(())
}
