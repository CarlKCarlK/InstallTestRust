use bed_reader::sample_bed_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bed_reader::{assert_eq_nan, Bed, ReadOptions};
    use ndarray as nd;

    let file_name = sample_bed_file("small.bed")?;
    let mut bed = Bed::new(file_name)?;
    let val = ReadOptions::builder().f64().read(&mut bed)?;

    assert_eq_nan(
        &val,
        &nd::array![
            [1.0, 0.0, f64::NAN, 0.0],
            [2.0, 0.0, f64::NAN, 2.0],
            [0.0, 1.0, 2.0, 0.0]
        ],
    );

    use ndarray::s;

    let file_name = sample_bed_file("some_missing.bed")?;
    let mut bed = Bed::new(&file_name)?;
    let val = ReadOptions::builder()
        .iid_index(s![..;2])
        .sid_index(20..30)
        .f64()
        .read(&mut bed)?;

    assert!(val.dim() == (50, 10));

    use std::collections::HashSet;

    let mut bed = Bed::new(&file_name)?;
    println!("{:?}", bed.iid()?.slice(s![..5])); // Outputs ndarray: ["iid_0", "iid_1", "iid_2", "iid_3", "iid_4"]
    println!("{:?}", bed.sid()?.slice(s![..5])); // Outputs ndarray: ["sid_0", "sid_1", "sid_2", "sid_3", "sid_4"]
    println!("{:?}", bed.chromosome()?.iter().collect::<HashSet<_>>());
    // Outputs: {"12", "10", "4", "8", "19", "21", "9", "15", "6", "16", "13", "7", "17", "18", "1", "22", "11", "2", "20", "3", "5", "14"}
    let val = ReadOptions::builder()
        .sid_index(bed.chromosome()?.map(|elem| elem == "5"))
        .f64()
        .read(&mut bed)?;

    assert!(val.dim() == (100, 6));

    Ok(())
}
