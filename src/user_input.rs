use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Kamil Utku Mavi, <kamilutkumavi0@gmail.com>")]
pub struct Args{
    #[arg(long, short = 'S', default_value_t=0)]
    pub saat: i32,
    #[arg(long, short = 'd', default_value_t=0)]
    pub dakika: i32,
    #[arg(long, short = 's', default_value_t=0)]
    pub saniye: i32,
    #[arg(long, short = 'm', default_value_t=0)]
    pub milisaniye: i32,
    #[arg(long, short = 'b', default_value_t=2)]
    pub bolum: i32,
}

pub fn pars_args() -> Args {
    Args::parse() 
}
