use std::clone;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    output: f32,

    #[arg(short, long, default_value = "12")]
    input: f32,

    #[arg(long, default_value = "5")]
    ref_min: f32,

    #[arg(long, default_value = "1600")]
    vco_max: f32,

    #[arg(long, default_value = "750")]
    vco_min: f32,

    #[clap(short, long)]
    low_vco: bool,
}

fn main() {
    let opts = Args::parse();

    // let fbdiv_range = 16..=320;
    // let postdiv_range = 1..=7;
    let refdiv_min = 1;
    let refdiv_max = 63;

    let refdiv_range =
        refdiv_min..=std::cmp::min(refdiv_max, (opts.input / opts.ref_min).floor() as i32);

    let mut best = (0_f32, 0_i32, 0_i32, 0_i32, 0_i32);
    let mut best_margin = (opts.output - best.0).abs();

    for refdiv in refdiv_range {
        for fbdiv in (16..=320).rev() {
            let vco = opts.input / (refdiv as f32) * (fbdiv as f32);
            if vco < opts.vco_min || vco > opts.vco_max {
                continue;
            }
            // pd1 is inner loop so that we prefer higher ratios of pd1:pd2
            for pd2 in 1..=7 {
                for pd1 in 1..=7 {
                    let out = vco / (pd1 as f32) / (pd2 as f32);
                    let margin = (opts.output - out).abs();
                    if margin < best_margin {
                        best = (out, fbdiv, pd1, pd2, refdiv);
                        best_margin = margin;
                    }
                }
            }
        }
    }

    println!("Requested: {} MHz", opts.output);
    println!("Achieved: {} MHz", best.0);
    println!("REFDIV: {}", best.4);
    println!(
        "FBDIV: {} (VCO = {} MHz)",
        best.1,
        opts.input / (best.4 as f32) * (best.1 as f32)
    );
    println!("PD1: {}", best.2);
    println!("PD2: {}", best.3);
}
