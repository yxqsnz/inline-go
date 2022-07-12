macro_rules! inline_go {
    ($($content:tt)*) => {{
        let data = stringify!($($content)*);
        std::fs::write("source.go", data.as_bytes()).unwrap();
        let cmd = std::process::Command::new("go")
                    .args(["run", "source.go"]).output();
        let res = cmd.unwrap();
        let _ = std::fs::remove_file("source.go");
        assert!(res.status.success());
        let mut stdout = String::from_utf8(res.stdout).unwrap();
        let stderr = String::from_utf8(res.stderr).unwrap();
        stdout.push_str(&stderr);
        stdout
    }}
}
fn main() {
    let out = inline_go! {
        package main;

        func main() {
            println("Lmao")
        }
    };
    println!("==> {out}");
}
