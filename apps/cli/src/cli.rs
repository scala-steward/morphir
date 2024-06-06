use crate::cli_args::RunArgs;
use crate::js_extensions::morphir_js;

pub fn run_js(args: &RunArgs) {
    let file = args.file.to_owned();
    std::thread::spawn(move || {
        let file_path = file.to_str();
        file_path.map(|path| {
            println!("Running JavaScript file: {}", path);

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            if let Err(error) = runtime.block_on(morphir_js(path)) {
                eprintln!("error: {error}");
            }
        });
    })
    .join()
    .unwrap();
}

pub fn run_js_file(args: &RunArgs) {
    let file = args.file.to_owned();
    let handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        let file_path = file.to_str();

        match file_path {
            Some(path) => {
                println!("Running JavaScript file: {}", path);
                if let Err(error) = handle.block_on(morphir_js(path)) {
                    eprintln!("error: {error}");
                }
            }
            None => {
                eprintln!("error: invalid file path");
            }
        }
    })
    .join()
    .unwrap();
}
