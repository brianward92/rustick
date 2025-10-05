mod publish;
mod server;
mod tick;

fn main() -> std::io::Result<()> {
    publish::publish_ticks_from_default()
}
