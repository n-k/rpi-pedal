use alsa::{pcm::*, Direction, Error, ValueOr, PCM};
use ringbuf::HeapRb;
use std::{thread, time::Duration};

fn main() -> ! {
    let rb = HeapRb::<i16>::new(8192 * 10);
    let (mut prod, mut cons) = rb.split();

    thread::spawn(move || {
        let pcm_in = start_pcm_stream("default:1", Direction::Capture).unwrap();
        let io_in = pcm_in.io_i16().unwrap();
        let mut buf = [0i16; 1024];
        loop {
            io_in.readi(&mut buf).unwrap();
            prod.push_slice(&buf);
        }
    });
    thread::spawn(move || {
        let pcm_out = start_pcm_stream("default:1", Direction::Playback).unwrap();
        let io_out = pcm_out.io_i16().unwrap();
        let mut buf = [0i16; 1024];
        loop {
            cons.pop_slice(&mut buf);
            io_out.writei(&buf).unwrap();
        }
    });

    loop {
        thread::sleep(Duration::from_millis(500));
    }
}

fn start_pcm_stream(device: &str, dir: Direction) -> Result<PCM, Error> {
    let pcm = PCM::new(device, dir, false)?;
    {
        // For this example, we assume 44100Hz, one channel, 16 bit audio.
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(1)?;
        hwp.set_rate(44100, ValueOr::Nearest)?;
        hwp.set_format(Format::s16())?;
        hwp.set_access(Access::RWInterleaved)?;
        pcm.hw_params(&hwp)?;
    }
    pcm.start()?;
    Ok(pcm)
}
