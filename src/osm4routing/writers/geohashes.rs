use geohashrust::BinaryHash;
use polars::prelude::*;

pub fn create_geohashes_csv(geohashes: Vec<String>, geohash_precision: u8) {
    let max_allowed_features_in_geohash = 1000;

    let mut final_geohashes: Vec<String> = Vec::new();
    let mut temp_geohashes: Vec<String> = geohashes;

    for i in 0 + 1..geohash_precision as usize {
        let sliced_geohashes: Vec<&str> =
            temp_geohashes.iter().map(|geohash| &geohash[..i]).collect();
        let geohash_df =
            df!("geohash" => temp_geohashes.clone(), "sliced_geohash" => sliced_geohashes).unwrap();

        let grouped_geohash_df = geohash_df
            .lazy()
            .groupby([col("sliced_geohash")])
            .agg([
                col("geohash").count().alias("count"),
                col("geohash").reverse().alias("geohashes"),
            ])
            .collect()
            .unwrap();

        let geohashes_over_max_allowed_features_df = grouped_geohash_df
            .clone()
            .lazy()
            .filter(col("count").gt(lit(max_allowed_features_in_geohash)))
            .collect()
            .unwrap();

        let geohashes_over_max_allowed_features_vec: Vec<String> =
            geohashes_over_max_allowed_features_df
                .column("geohashes")
                .unwrap()
                .explode()
                .unwrap()
                .utf8()
                .unwrap()
                .into_no_null_iter()
                .collect::<Vec<&str>>()
                .iter()
                .map(|geohash| geohash.to_string())
                .collect();

        temp_geohashes = geohashes_over_max_allowed_features_vec;

        let geohashes_under_max_allowed_features_df = grouped_geohash_df
            .clone()
            .lazy()
            .filter(col("count").lt(lit(max_allowed_features_in_geohash + 1)))
            .collect()
            .unwrap();

        let mut geohashes_under_max_allowed_features_vec: Vec<String> =
            geohashes_under_max_allowed_features_df
                .column("sliced_geohash")
                .unwrap()
                .utf8()
                .unwrap()
                .into_no_null_iter()
                .collect::<Vec<&str>>()
                .iter()
                .map(|geohash| geohash.to_string())
                .collect();

        final_geohashes.append(&mut geohashes_under_max_allowed_features_vec);
    }

    // remove duplicated geohashes
    temp_geohashes.sort_unstable();
    temp_geohashes.dedup();
    final_geohashes.append(&mut temp_geohashes);

    let geohashes_path = std::path::Path::new("geohashes.csv");
    let mut geohashes_csv = csv::Writer::from_path(geohashes_path).unwrap();
    geohashes_csv
        .serialize(vec!["geohash", "min_lon", "min_lat", "max_lon", "max_lat"])
        .expect("CSV: unable to write geohash header");

    for final_geohash in final_geohashes {
        let bh = BinaryHash::from_string(final_geohash.as_str());
        let bbox = bh.decode();
        geohashes_csv
            .serialize((
                final_geohash,
                bbox.min_lon,
                bbox.min_lat,
                bbox.max_lon,
                bbox.max_lat,
            ))
            .expect("CSV: unable to write geohash");
    }
}
