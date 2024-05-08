use urlshortener::{ client::UrlShortener, providers::Provider };
use std::io::{self, Write};
use colored::Colorize; 
use std::fs::File;
use std::io::{BufRead, BufReader};

fn url_google() {
    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate("https://google.com/", &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}

fn url_snapchat() {
    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate("https://snapchat.com/", &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}

fn url_facebook() {
    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate("https://facebook.com/", &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}

fn url_instagram() {
    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate("https://instagram.com/", &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}

fn url_x() {
    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate("https://x.com/", &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}

fn url_user() {
    
    print!("Lütfen bir URL giriniz: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Giriş başarısız.");

    println!("{} {}", "Link".red(), "oluşturuluyor..".red());

    let url_s = UrlShortener::new().unwrap();
    let short_url = url_s.generate(choice, &Provider::IsGd);
    assert!(short_url.is_ok());
    println!("{:?}", short_url);

}


fn url_list_user() {
    print!("Lütfen birden fazla URL giriniz: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Giriş başarısız.");

    println!("{} {}", "Linkler".red(), "oluşturuluyor..".red());

    let url_s = UrlShortener::new().unwrap();
    let urls: Vec<&str> = choice.trim().split_whitespace().collect();

    for url in urls {
        let short_url = url_s.generate(url.to_string(), &Provider::IsGd);
        match short_url {
            Ok(short) => println!("{} -> {}", url, short),
            Err(err) => eprintln!("{} -> Hata: {}", url, err),
        }
    }
}

fn url_file_user() {
    let shortener = UrlShortener::new().expect("UrlShortener oluşturulamadı");

    let file = File::open("src/urlist.txt").expect("Dosya bulunamadı");
    let reader = BufReader::new(file);
    
    println!("{} {}", "urlist.txt içinde bulunan URL'ler".red(), "oluşturuluyor..".red());

    for line in reader.lines() {
        let url = line.expect("Satır okunamadı");


        let shortened_url = shortener.generate(url.clone(), &Provider::IsGd);
        match shortened_url {
            Ok(short) => println!("{} -> {}", url, short),
            Err(err) => eprintln!("{} -> Hata: {}", url, err),
        }
    }
}


fn main() {
    // Banner
    println!("+----------------------+");
    println!("|   İstediğiniz URL'yi |");
    println!("|        Seçin         |");
    println!("+----------------------+");
    println!("| 1. {}{}            |", "Google".white(), "".white());
    println!("| 2. {}{}         |", "Instagram".magenta(), "".magenta());
    println!("| 3. {}{}          |", "Snapchat".yellow(), "".yellow());
    println!("| 4. {}{}          |", "Facebook".blue(), "".blue());
    println!("| 5. {}{}       |", "X (Twitter)".green(), "".green());
    println!("| 6. {}{}         |", "Başka URL".cyan(), "".cyan());
    println!("| 7. {}{}  |", "Birden fazla URL".white(), "".white());
    println!("| 8. {}{}  |", "Liste içinde URL".yellow(), "".yellow());
    println!("+----------------------+");

    print!("Lütfen bir seçenek numarası girin: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Giriş başarısız.");


    match choice.trim() {
        "1" => url_google(), 
        "2" => url_instagram(),
        "3" => url_snapchat(),
        "4" => url_facebook(),
        "5" => url_x(),
        "6" => url_user(),
        "7" => url_list_user(),
        "8" => url_file_user(),
        _ => println!("Geçersiz seçenek! Lütfen doğru bir seçenek numarası girin."),
    }

}
