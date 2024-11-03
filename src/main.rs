use sure_hesapla::user_input::pars_args;
 
fn dakika2milisaniye(dakika: i32) -> i32 {
    saniye2milisaniye(dakika*60)
}

fn saat2milisaniye(saat: i32) -> i32 {
    dakika2milisaniye(saat*24)
}

fn saniye2milisaniye(saniye: i32) -> i32 {
   saniye * 1000
}

fn milisaniye2time(mut milisaniye: i32){
    let mut saniye = 0;
    let mut dakika = 0;
    let mut saat = 0;
    loop {
        if milisaniye >= 1000 && milisaniye != 0 {
            saniye += 1;
            milisaniye -= 1000; 
        } else {
            break;
        }
    }
    loop {
        if saniye >= 60 && saniye != 0 {
            dakika += 1;
            saniye -= 60;
        } else {
            break;
        }
    }
    loop {
        if dakika >= 24 && dakika != 0 {
            saat += 1;
            dakika -= 24;
        } else {
            break;
        }
    }
    println!("{} saat {} dakika {} saniye {} milisaniye",saat, dakika, saniye, milisaniye);
}
fn main() {
    let my_args = pars_args();
    let mut milisaniye = saat2milisaniye(my_args.saat) 
        + dakika2milisaniye(my_args.dakika) 
        + saniye2milisaniye(my_args.saniye) 
        + my_args.milisaniye;
    milisaniye /= my_args.bolum;
    milisaniye2time(milisaniye);
 }
