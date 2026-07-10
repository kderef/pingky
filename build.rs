use winres::WindowsResource;

fn main() {
    static_vcruntime::metabuild();

    let mut res = WindowsResource::new();

    res.set_icon("icon.ico");

    res.compile().unwrap();
}
