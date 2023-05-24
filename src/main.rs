use clap::Parser;
use osm4routing::ProfileType;

/// Extract a road or railway network from Openstreetmap as a graph ready for routing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input path (<source.osm.pbf>)
    #[arg(short, long)]
    input: String,

    /// Profile type
    #[arg(short, long)]
    profile: String,
}

fn main() {
    let args = Args::parse();

    let profile_type = match args.profile.as_str() {
        "railway" => ProfileType::Railway,
        "road" => ProfileType::Road,
        _ => panic!("specified profile isn't implemented"),
    };

    match osm4routing::read(args.input.as_str(), profile_type) {
        Ok((nodes, edges)) => osm4routing::write(nodes, edges, profile_type),
        Err(error) => println!("Error: {}", error),
    }
}
