//can be used to read and write to ports, look in docs.rs for details. 
use cpuio::Port;
//access to registers? x86_64 crate is probably needed but have to find which  part is relevant
use x86_64::registers;


//initializing variables to be read from RTC
fn main(){
    let mut second: u64 = 0;
    let mut minute: u64 = 0;
    let mut hour: u64 = 0;
    let mut day: u64 = 0;
    let mut month: u64 = 0;
    let mut year: u64 = 0;
}

