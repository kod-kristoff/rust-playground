use crate::domain::models::Chart;

pub fn print_chart(chart: &Chart, positions: &[i32], cutoff: Option<usize>) {
    let cutoff: usize = cutoff.unwrap_or(8);
    println!("Chart size: {} edges", chart.chartsize());
    for (k, edgeset) in chart.chart.iter().enumerate() {
        if edgeset.len() > 0 && (positions.contains(&(k as i32)) || positions.contains(&(k as i32 - chart.chart.len() as i32))) {
            println!("{} edges ending in position {}:", edgeset.len(), k);
            let mut sorted_edgeset = edgeset.to_vec();
            sorted_edgeset.sort();
            for (n, edge) in sorted_edgeset.iter().enumerate() {
                if cutoff > 0 && n >= cutoff {
                    println!("    ...");
                    break;
                }
                println!("    {}", edge);
            }
        }
    }
}
