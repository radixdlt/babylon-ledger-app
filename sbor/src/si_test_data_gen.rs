/// Generate test vectors as binary files for use by tests.
#[cfg(test)]
pub mod tests {
    use crate::si_test_data::tests::*;
    use std::io::Write;

    struct Blob {
        pub name: &'static str,
        pub data: &'static [u8],
    }

    const BLOBS: &[Blob] = &[
        Blob {
            name: "checked_childless_subintent",
            data: &SI_CHECKED_CHILDLESS_SUBINTENT,
        },
        Blob {
            name: "subintent_vector_0",
            data: &SI_VECTOR_0,
        },
        Blob {
            name: "subintent_vector_1",
            data: &SI_VECTOR_1,
        },
        Blob {
            name: "subintent_vector_2",
            data: &SI_VECTOR_2,
        },
        Blob {
            name: "subintent_vector_3",
            data: &SI_VECTOR_3,
        },
        Blob {
            name: "subintent_vector_4",
            data: &SI_VECTOR_4,
        },
        Blob {
            name: "subintent_vector_5",
            data: &SI_VECTOR_5,
        },
        Blob {
            name: "subintent_vector_6",
            data: &SI_VECTOR_6,
        },
    ];

    #[test]
    pub fn generate_binaries() {
        for blob in BLOBS {
            let mut file =
                std::fs::File::create(format!("../ragger_tests/data/{}.si", blob.name)).unwrap();
            file.write_all(blob.data).unwrap();
        }
    }
}
