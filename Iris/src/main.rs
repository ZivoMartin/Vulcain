mod system;
mod text_file;
mod interpreteur;
mod type_gestion;
mod view;


use interpreteur::Interpreteur;
use text_file::file_exists;
use crate::text_file::TextFile;
use crate::view::View;
use std::process::{exit, ExitCode};
use std::env;

static OK: u8 = 0;
static ERROR: u8 = 1;
struct RequestParameters {
    request: String,
    json: bool,
    file_sql: String,
    ide: bool
}

impl RequestParameters {
    fn new() -> RequestParameters {
        RequestParameters{
            request: String::new(),
            json: false,
            file_sql: String::new(),
            ide: false
        }
    }
}

/// Parameters:
///     -ide: Start the Iris ide
///     -j $file_name.json : export the result in the file_name.json file
///     -d "SQL REQUEST" : performs the query
///     -f $file_name.sql : execute the content of the .sql file.
fn main() -> ExitCode{
    let args: Vec<String> = env::args().collect();
    let mut req = RequestParameters::new();
    let mut iter = args.iter().skip(1);
    while let Some(elt) = iter.next() {
        match &elt as &str {
            "-j" => req.json = true,
            "-ide" => req.ide = true,
            "-f" => {
                if let Some(path) = iter.next() {
                    req.file_sql = path.to_string();
                }else{
                    eprintln!("You didn't precise the file path with the '-f' parameter.");
                    return ExitCode::from(ERROR)
                }
            }
            "-d" => {
                if let Some(request) = iter.next() {
                   req.request = request.to_string()
                }
            }
            _ => {
                eprintln!("Unknow parameter: {}", elt);
                return ExitCode::from(ERROR)
            }
        }
    }
    let mut interpreteur = Interpreteur::new();
    interpreteur.sqlrequest(req.request, req.json).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(OK as i32)
    });

    if req.file_sql != "" {
        if file_exists(&req.file_sql){
            if req.file_sql.ends_with(".sql") || req.file_sql.ends_with(".txt"){
                let mut sql_file = TextFile::new(req.file_sql.to_string());
                let mut f_text = sql_file.get_text();
                f_text = f_text.replace("\n", "");
                let mut all_request: Vec<&str> = f_text.split(";").collect();
                all_request.pop();
                for request in all_request{
                    interpreteur.sqlrequest(request.to_string(), req.json).unwrap_or_else(|e| {
                        eprintln!("{e}");
                        eprintln!("During the execution of the file {}", req.file_sql);
                        exit(OK as i32)
                    });
                }
                println!("The execution of the file {} has been a success", req.file_sql);
            }else{
                eprintln!("The file {} is not a sql file.", req.file_sql);
            }
        }else{
            eprintln!("Couldn't open the file {}", req.file_sql)
        } 
    }

    if req.ide {
        let mut view = View::new(interpreteur).unwrap();
        view.start().unwrap();
    }
    
    ExitCode::from(OK)
}


