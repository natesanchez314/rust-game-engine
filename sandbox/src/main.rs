use capy::run;

struct SandboxApp {

}

impl App for SandboxApp {
    
}

fn main() {
    pollster::block_on(run());
}
