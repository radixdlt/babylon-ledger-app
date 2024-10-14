#[cfg(test)]
pub mod tests {
    use crate::si_test_data::tests::*;
    use std::io::Write;

    struct Blob {
        pub name: &'static str,
        pub data: &'static [u8],
    }

    const BLOBS: &[Blob] = &[Blob {
        name: "checked_childless_subintent",
        data: &SI_CHECKED_CHILDLESS_SUBINTENT,
    }];

    #[test]
    pub fn generate_binaries() {
        for blob in BLOBS {
            let mut file = std::fs::File::create(format!("../test/data/{}.si", blob.name)).unwrap();
            file.write_all(blob.data).unwrap();
        }
    }
}
