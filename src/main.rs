
use std::{thread, time};

extern crate stopwatch;
use stopwatch::{Stopwatch};

fn main() {
    let mut hundos :u32 = 0;
    let timing :bool = true;
    let limit :u32 = 10000;

    while timing {
        clock(hundos);    
        if hundos == limit {
            break;
        }
        hundos += 1;
    }
}

/*this uses u32 integers so as long as you don't run it for 49 days straight
or travel backward in time there should be no overflow issues*/
fn time_converter(hundos: u32) -> (u32, u32, u32) {
    let mut secos = hundos / 100;
    let minos = secos / 60;
    secos = secos % 60;
    let hundos = hundos % 100;
    return (minos, secos, hundos)
}

/*implements timer by counting hundredths
of a second in 10 ms intervals ("hundos")*/
fn clock(hundos :u32) {
    let sw = Stopwatch::start_new();
        
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    
    assert!(now.elapsed() >= ten_millis);

    /*ingeniously converts hundos to minutes and decimal seconds,
    stores astonishing results in tuple "mino_seco_hundo_tuplo"*/
    let mino_seco_hundo_tuplo = time_converter(hundos);
    println!("{0}:{1}.{2}", mino_seco_hundo_tuplo.0,
                            mino_seco_hundo_tuplo.1,
                            mino_seco_hundo_tuplo.2);
    println!("That took {}ms", sw.elapsed_ms());
}
