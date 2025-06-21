use std::{env, fs::File, io::copy};

fn main()  {
    let response = reqwest::blocking::get(
        "https://github.com/rajnishsubedi0/bhajan_sangraha/releases",
    ).unwrap()
    .text().unwrap().clone();

    let document = scraper::Html::parse_document(&response);
    

    let tags: Vec<String> = document
        .select(&scraper::Selector::parse(
            "div.d-flex div.flex-md-row a.Link--primary",
        ).unwrap())
        .map(|x| x.text().collect::<String>())
        .collect();

    let download_path_current_working_folder=env::current_dir().unwrap();
    let output_directory=format!("{}/output.apk",download_path_current_working_folder.to_str().unwrap().to_string());
    
    let app_link=format!("https://github.com/rajnishsubedi0/bhajan_sangraha/releases/download/{}/app-release.apk",tags[0]);
    let mut app_download_response=reqwest::blocking::get(app_link).unwrap();

    let mut file=File::create(output_directory).unwrap();
    copy(&mut app_download_response,&mut file);
   
    
}