use core::fmt::Write as _;
use core::ops;

use embedded_io_async::{Read, Write};

type Response = heapless::String<255>;

const BUFFER_SIZE: usize = 25 * 1024;

use crate::{
    info, trace, warn, Day, Duration, Handler, Instant, PartResult, Timer, END_INPUT_TAG,
    START_INPUT_TAG,
};

/// # Panics
pub async fn run<const NOM: u32, const DENOM: u32>(
    (mut rx, mut tx): (impl Read, impl Write),
    timer: &impl Timer<u64, NOM, DENOM>,
    mut handler: impl Handler<u64, NOM, DENOM>,
) -> !
where
    Instant<u64, NOM, DENOM>: ops::Sub<Output = Duration<u64, NOM, DENOM>>,
{
    static RESPONSE: static_cell::StaticCell<Response> = static_cell::StaticCell::new();
    static BUFFER: static_cell::StaticCell<[u8; BUFFER_SIZE]> = static_cell::StaticCell::new();
    
    trace!("run");

    let response = RESPONSE.init_with(|| Response::new());

    let buffer = BUFFER.init_with(|| [0; BUFFER_SIZE]);
    loop {
        let mut length = 0;
        loop {
            if length >= buffer.len() {
                warn!("buffer overflow");
                break;
            }

            match rx.read(&mut buffer[length..]).await {
                Err(_err) => {
                    #[cfg(feature = "log")]
                    warn!("error reading: {_err:?}");
                }
                Ok(0) => {
                    trace!("reading 0 bytes");
                }
                Ok(count) => {
                    debug_assert!(length + count <= buffer.len(), "invalid count");

                    length += count;

                    if let Ok(input) = core::str::from_utf8(&buffer[..length]) {
                        match (input.find(START_INPUT_TAG), input.find(END_INPUT_TAG)) {
                            (Some(start_position), Some(end_position)) => {
                                let Ok(day) =
                                    input[start_position + START_INPUT_TAG.len()..].parse::<Day>()
                                else {
                                    warn!("unsupported day");

                                    handler.unsupported_day();

                                    tx.write_all(b"unsupported day\r\n").await.ok();

                                    break;
                                };

                                let input = input
                                    [start_position + START_INPUT_TAG.len() + 2..end_position]
                                    .trim();

                                info!("[{}] start working on {}", day, day);

                                let mut part_1 = PartResult::new();
                                let mut part_2 = PartResult::new();

                                let start = timer.now();

                                handler.started(day, start);

                                if day.solve_1(&mut part_1, input).is_err() {
                                    warn!("part_1: buffer overflow");
                                    break;
                                }

                                if day.solve_2(&mut part_2, input).is_err() {
                                    warn!("part_2: buffer overflow");
                                    break;
                                }

                                let elapsed = timer.now() - start;

                                handler.ended(day, elapsed, part_1.as_str(), part_2.as_str());

                                info!("[{}] part 1: {}", day, part_1.as_str());

                                response.clear();
                                write!(response, "[{day}] part 1: {part_1}\r\n").ok();
                                tx.write_all(response.as_bytes()).await.ok();

                                info!("[{}] part 2: {}", day, part_2.as_str());

                                response.clear();
                                write!(response, "[{day}] part 2: {part_2}\r\n").ok();
                                tx.write_all(response.as_bytes()).await.ok();

                                info!(
                                    "[{}] elapsed: {}ms ({}us)",
                                    day,
                                    elapsed.to_millis(),
                                    elapsed.to_micros()
                                );

                                response.clear();
                                write!(
                                    response,
                                    "[{day}] elapsed: {}ms ({}us)\r\n",
                                    elapsed.to_millis(),
                                    elapsed.to_micros()
                                )
                                .ok();
                                tx.write_all(response.as_bytes()).await.ok();

                                break;
                            }
                            (None, Some(_)) => {
                                warn!("invalid input");

                                handler.invalid_input();

                                tx.write_all(b"invalid input\r\n").await.ok();

                                break;
                            }
                            _ => {}
                        }
                    } else {
                        warn!("invalid utf8 data");
                        break;
                    }
                }
            }
        }
    }
}
