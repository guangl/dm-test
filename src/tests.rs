// 此文件由 `cargo run -- gen-tests` 自动生成
// 请勿手动编辑

#[cfg(test)]
mod sqllogic_tests {
    use crate::{DatabaseConfig, SqlLogicTestRunner};
    use std::path::Path;

    // 辅助宏：为每个测试文件生成独立的测试函数
    macro_rules! sqllogic_test {
        ($name:ident, $path:expr) => {
            #[tokio::test]
            async fn $name() -> Result<(), Box<dyn std::error::Error>> {
                let config = DatabaseConfig::from_env();
                let test_file = $path;

                if !Path::new(test_file).exists() {
                    println!("⚠️ 跳过不存在的测试文件: {}", test_file);
                    return Ok(());
                }

                println!("🧪 运行测试: {}", test_file);
                SqlLogicTestRunner::run_file(&config, test_file).await?;
                println!("✅ {} 测试通过", test_file);
                Ok(())
            }
        };
    }

    // tests/datatype/bfile/bfile_basic.test
    sqllogic_test!(test_datatype_bfile_bfile_basic, "tests/datatype/bfile/bfile_basic.test");

    // tests/datatype/bfile/bfile_null.test
    sqllogic_test!(test_datatype_bfile_bfile_null, "tests/datatype/bfile/bfile_null.test");

    // tests/datatype/bigint/01_bigint_basic.test
    sqllogic_test!(test_datatype_bigint_01_bigint_basic, "tests/datatype/bigint/01_bigint_basic.test");

    // tests/datatype/bigint/02_bigint_arithmetic.test
    sqllogic_test!(test_datatype_bigint_02_bigint_arithmetic, "tests/datatype/bigint/02_bigint_arithmetic.test");

    // tests/datatype/bigint/03_bigint_aggregate.test
    sqllogic_test!(test_datatype_bigint_03_bigint_aggregate, "tests/datatype/bigint/03_bigint_aggregate.test");

    // tests/datatype/bigint/04_bigint_null.test
    sqllogic_test!(test_datatype_bigint_04_bigint_null, "tests/datatype/bigint/04_bigint_null.test");

    // tests/datatype/bigint/05_bigint_comparison.test
    sqllogic_test!(test_datatype_bigint_05_bigint_comparison, "tests/datatype/bigint/05_bigint_comparison.test");

    // tests/datatype/bigint/06_bigint_ordering.test
    sqllogic_test!(test_datatype_bigint_06_bigint_ordering, "tests/datatype/bigint/06_bigint_ordering.test");

    // tests/datatype/bit/01_bit_basic.test
    sqllogic_test!(test_datatype_bit_01_bit_basic, "tests/datatype/bit/01_bit_basic.test");

    // tests/datatype/bit/02_bit_operations.test
    sqllogic_test!(test_datatype_bit_02_bit_operations, "tests/datatype/bit/02_bit_operations.test");

    // tests/datatype/bit/03_bit_null.test
    sqllogic_test!(test_datatype_bit_03_bit_null, "tests/datatype/bit/03_bit_null.test");

    // tests/datatype/bit/04_bit_default.test
    sqllogic_test!(test_datatype_bit_04_bit_default, "tests/datatype/bit/04_bit_default.test");

    // tests/datatype/blob/blob_basic.test
    sqllogic_test!(test_datatype_blob_blob_basic, "tests/datatype/blob/blob_basic.test");

    // tests/datatype/blob/blob_binary_data.test
    sqllogic_test!(test_datatype_blob_blob_binary_data, "tests/datatype/blob/blob_binary_data.test");

    // tests/datatype/blob/blob_large_data.test
    sqllogic_test!(test_datatype_blob_blob_large_data, "tests/datatype/blob/blob_large_data.test");

    // tests/datatype/blob/blob_null.test
    sqllogic_test!(test_datatype_blob_blob_null, "tests/datatype/blob/blob_null.test");

    // tests/datatype/blob/blob_operations.test
    sqllogic_test!(test_datatype_blob_blob_operations, "tests/datatype/blob/blob_operations.test");

    // tests/datatype/blob/blob_update.test
    sqllogic_test!(test_datatype_blob_blob_update, "tests/datatype/blob/blob_update.test");

    // tests/datatype/char/01_char_basic.test
    sqllogic_test!(test_datatype_char_01_char_basic, "tests/datatype/char/01_char_basic.test");

    // tests/datatype/char/02_char_length_variations.test
    sqllogic_test!(test_datatype_char_02_char_length_variations, "tests/datatype/char/02_char_length_variations.test");

    // tests/datatype/char/03_char_padding.test
    sqllogic_test!(test_datatype_char_03_char_padding, "tests/datatype/char/03_char_padding.test");

    // tests/datatype/char/04_char_null_handling.test
    sqllogic_test!(test_datatype_char_04_char_null_handling, "tests/datatype/char/04_char_null_handling.test");

    // tests/datatype/char/05_char_default_values.test
    sqllogic_test!(test_datatype_char_05_char_default_values, "tests/datatype/char/05_char_default_values.test");

    // tests/datatype/char/06_char_comparison.test
    sqllogic_test!(test_datatype_char_06_char_comparison, "tests/datatype/char/06_char_comparison.test");

    // tests/datatype/char/07_char_ordering.test
    sqllogic_test!(test_datatype_char_07_char_ordering, "tests/datatype/char/07_char_ordering.test");

    // tests/datatype/char/08_char_update.test
    sqllogic_test!(test_datatype_char_08_char_update, "tests/datatype/char/08_char_update.test");

    // tests/datatype/char/09_char_delete.test
    sqllogic_test!(test_datatype_char_09_char_delete, "tests/datatype/char/09_char_delete.test");

    // tests/datatype/char/10_char_constraints.test
    sqllogic_test!(test_datatype_char_10_char_constraints, "tests/datatype/char/10_char_constraints.test");

    // tests/datatype/char/11_char_index.test
    sqllogic_test!(test_datatype_char_11_char_index, "tests/datatype/char/11_char_index.test");

    // tests/datatype/char/12_char_join.test
    sqllogic_test!(test_datatype_char_12_char_join, "tests/datatype/char/12_char_join.test");

    // tests/datatype/char/13_char_aggregate.test
    sqllogic_test!(test_datatype_char_13_char_aggregate, "tests/datatype/char/13_char_aggregate.test");

    // tests/datatype/character/01_character_basic.test
    sqllogic_test!(test_datatype_character_01_character_basic, "tests/datatype/character/01_character_basic.test");

    // tests/datatype/character/02_character_padding.test
    sqllogic_test!(test_datatype_character_02_character_padding, "tests/datatype/character/02_character_padding.test");

    // tests/datatype/character/03_character_constraints.test
    sqllogic_test!(test_datatype_character_03_character_constraints, "tests/datatype/character/03_character_constraints.test");

    // tests/datatype/character/04_character_aggregate.test
    sqllogic_test!(test_datatype_character_04_character_aggregate, "tests/datatype/character/04_character_aggregate.test");

    // tests/datatype/clob/clob_basic.test
    sqllogic_test!(test_datatype_clob_clob_basic, "tests/datatype/clob/clob_basic.test");

    // tests/datatype/clob/clob_large_data.test
    sqllogic_test!(test_datatype_clob_clob_large_data, "tests/datatype/clob/clob_large_data.test");

    // tests/datatype/clob/clob_null.test
    sqllogic_test!(test_datatype_clob_clob_null, "tests/datatype/clob/clob_null.test");

    // tests/datatype/clob/clob_operations.test
    sqllogic_test!(test_datatype_clob_clob_operations, "tests/datatype/clob/clob_operations.test");

    // tests/datatype/date/date_arithmetic.test
    sqllogic_test!(test_datatype_date_date_arithmetic, "tests/datatype/date/date_arithmetic.test");

    // tests/datatype/date/date_basic.test
    sqllogic_test!(test_datatype_date_date_basic, "tests/datatype/date/date_basic.test");

    // tests/datatype/date/date_boundary.test
    sqllogic_test!(test_datatype_date_date_boundary, "tests/datatype/date/date_boundary.test");

    // tests/datatype/date/date_comparison.test
    sqllogic_test!(test_datatype_date_date_comparison, "tests/datatype/date/date_comparison.test");

    // tests/datatype/date/date_format.test
    sqllogic_test!(test_datatype_date_date_format, "tests/datatype/date/date_format.test");

    // tests/datatype/date/date_functions.test
    sqllogic_test!(test_datatype_date_date_functions, "tests/datatype/date/date_functions.test");

    // tests/datatype/date/date_interval.test
    sqllogic_test!(test_datatype_date_date_interval, "tests/datatype/date/date_interval.test");

    // tests/datatype/date/date_null.test
    sqllogic_test!(test_datatype_date_date_null, "tests/datatype/date/date_null.test");

    // tests/datatype/double/01_double_basic.test
    sqllogic_test!(test_datatype_double_01_double_basic, "tests/datatype/double/01_double_basic.test");

    // tests/datatype/double/02_double_precision.test
    sqllogic_test!(test_datatype_double_02_double_precision, "tests/datatype/double/02_double_precision.test");

    // tests/datatype/double/03_double_arithmetic.test
    sqllogic_test!(test_datatype_double_03_double_arithmetic, "tests/datatype/double/03_double_arithmetic.test");

    // tests/datatype/double/04_double_null.test
    sqllogic_test!(test_datatype_double_04_double_null, "tests/datatype/double/04_double_null.test");

    // tests/datatype/float/01_float_basic.test
    sqllogic_test!(test_datatype_float_01_float_basic, "tests/datatype/float/01_float_basic.test");

    // tests/datatype/float/02_float_arithmetic.test
    sqllogic_test!(test_datatype_float_02_float_arithmetic, "tests/datatype/float/02_float_arithmetic.test");

    // tests/datatype/float/03_float_precision.test
    sqllogic_test!(test_datatype_float_03_float_precision, "tests/datatype/float/03_float_precision.test");

    // tests/datatype/float/04_float_null.test
    sqllogic_test!(test_datatype_float_04_float_null, "tests/datatype/float/04_float_null.test");

    // tests/datatype/float/05_float_comparison.test
    sqllogic_test!(test_datatype_float_05_float_comparison, "tests/datatype/float/05_float_comparison.test");

    // tests/datatype/float/06_float_aggregate.test
    sqllogic_test!(test_datatype_float_06_float_aggregate, "tests/datatype/float/06_float_aggregate.test");

    // tests/datatype/image/image_basic.test
    sqllogic_test!(test_datatype_image_image_basic, "tests/datatype/image/image_basic.test");

    // tests/datatype/image/image_binary_data.test
    sqllogic_test!(test_datatype_image_image_binary_data, "tests/datatype/image/image_binary_data.test");

    // tests/datatype/image/image_large_data.test
    sqllogic_test!(test_datatype_image_image_large_data, "tests/datatype/image/image_large_data.test");

    // tests/datatype/image/image_null.test
    sqllogic_test!(test_datatype_image_image_null, "tests/datatype/image/image_null.test");

    // tests/datatype/image/image_update.test
    sqllogic_test!(test_datatype_image_image_update, "tests/datatype/image/image_update.test");

    // tests/datatype/integer/01_integer_basic.test
    sqllogic_test!(test_datatype_integer_01_integer_basic, "tests/datatype/integer/01_integer_basic.test");

    // tests/datatype/integer/02_integer_range.test
    sqllogic_test!(test_datatype_integer_02_integer_range, "tests/datatype/integer/02_integer_range.test");

    // tests/datatype/integer/03_integer_arithmetic.test
    sqllogic_test!(test_datatype_integer_03_integer_arithmetic, "tests/datatype/integer/03_integer_arithmetic.test");

    // tests/datatype/integer/04_integer_null.test
    sqllogic_test!(test_datatype_integer_04_integer_null, "tests/datatype/integer/04_integer_null.test");

    // tests/datatype/integer/05_integer_aggregate.test
    sqllogic_test!(test_datatype_integer_05_integer_aggregate, "tests/datatype/integer/05_integer_aggregate.test");

    // tests/datatype/integer/06_integer_default.test
    sqllogic_test!(test_datatype_integer_06_integer_default, "tests/datatype/integer/06_integer_default.test");

    // tests/datatype/integer/07_integer_comparison.test
    sqllogic_test!(test_datatype_integer_07_integer_comparison, "tests/datatype/integer/07_integer_comparison.test");

    // tests/datatype/integer/08_integer_ordering.test
    sqllogic_test!(test_datatype_integer_08_integer_ordering, "tests/datatype/integer/08_integer_ordering.test");

    // tests/datatype/integer/09_integer_division.test
    sqllogic_test!(test_datatype_integer_09_integer_division, "tests/datatype/integer/09_integer_division.test");

    // tests/datatype/integer/10_integer_modulo.test
    sqllogic_test!(test_datatype_integer_10_integer_modulo, "tests/datatype/integer/10_integer_modulo.test");

    // tests/datatype/interval/interval_arithmetic_operations.test
    sqllogic_test!(test_datatype_interval_interval_arithmetic_operations, "tests/datatype/interval/interval_arithmetic_operations.test");

    // tests/datatype/interval/interval_day_basic.test
    sqllogic_test!(test_datatype_interval_interval_day_basic, "tests/datatype/interval/interval_day_basic.test");

    // tests/datatype/interval/interval_day_date_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_day_date_arithmetic, "tests/datatype/interval/interval_day_date_arithmetic.test");

    // tests/datatype/interval/interval_day_hour_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_day_hour_arithmetic, "tests/datatype/interval/interval_day_hour_arithmetic.test");

    // tests/datatype/interval/interval_day_hour_basic.test
    sqllogic_test!(test_datatype_interval_interval_day_hour_basic, "tests/datatype/interval/interval_day_hour_basic.test");

    // tests/datatype/interval/interval_day_minute_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_day_minute_arithmetic, "tests/datatype/interval/interval_day_minute_arithmetic.test");

    // tests/datatype/interval/interval_day_minute_basic.test
    sqllogic_test!(test_datatype_interval_interval_day_minute_basic, "tests/datatype/interval/interval_day_minute_basic.test");

    // tests/datatype/interval/interval_day_second_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_day_second_arithmetic, "tests/datatype/interval/interval_day_second_arithmetic.test");

    // tests/datatype/interval/interval_day_second_basic.test
    sqllogic_test!(test_datatype_interval_interval_day_second_basic, "tests/datatype/interval/interval_day_second_basic.test");

    // tests/datatype/interval/interval_hour_basic.test
    sqllogic_test!(test_datatype_interval_interval_hour_basic, "tests/datatype/interval/interval_hour_basic.test");

    // tests/datatype/interval/interval_hour_minute_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_hour_minute_arithmetic, "tests/datatype/interval/interval_hour_minute_arithmetic.test");

    // tests/datatype/interval/interval_hour_minute_basic.test
    sqllogic_test!(test_datatype_interval_interval_hour_minute_basic, "tests/datatype/interval/interval_hour_minute_basic.test");

    // tests/datatype/interval/interval_hour_second_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_hour_second_arithmetic, "tests/datatype/interval/interval_hour_second_arithmetic.test");

    // tests/datatype/interval/interval_hour_second_basic.test
    sqllogic_test!(test_datatype_interval_interval_hour_second_basic, "tests/datatype/interval/interval_hour_second_basic.test");

    // tests/datatype/interval/interval_hour_time_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_hour_time_arithmetic, "tests/datatype/interval/interval_hour_time_arithmetic.test");

    // tests/datatype/interval/interval_minute_basic.test
    sqllogic_test!(test_datatype_interval_interval_minute_basic, "tests/datatype/interval/interval_minute_basic.test");

    // tests/datatype/interval/interval_minute_second_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_minute_second_arithmetic, "tests/datatype/interval/interval_minute_second_arithmetic.test");

    // tests/datatype/interval/interval_minute_second_basic.test
    sqllogic_test!(test_datatype_interval_interval_minute_second_basic, "tests/datatype/interval/interval_minute_second_basic.test");

    // tests/datatype/interval/interval_minute_time_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_minute_time_arithmetic, "tests/datatype/interval/interval_minute_time_arithmetic.test");

    // tests/datatype/interval/interval_month_basic.test
    sqllogic_test!(test_datatype_interval_interval_month_basic, "tests/datatype/interval/interval_month_basic.test");

    // tests/datatype/interval/interval_month_date_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_month_date_arithmetic, "tests/datatype/interval/interval_month_date_arithmetic.test");

    // tests/datatype/interval/interval_multiply_divide.test
    sqllogic_test!(test_datatype_interval_interval_multiply_divide, "tests/datatype/interval/interval_multiply_divide.test");

    // tests/datatype/interval/interval_second_basic.test
    sqllogic_test!(test_datatype_interval_interval_second_basic, "tests/datatype/interval/interval_second_basic.test");

    // tests/datatype/interval/interval_second_time_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_second_time_arithmetic, "tests/datatype/interval/interval_second_time_arithmetic.test");

    // tests/datatype/interval/interval_year_basic.test
    sqllogic_test!(test_datatype_interval_interval_year_basic, "tests/datatype/interval/interval_year_basic.test");

    // tests/datatype/interval/interval_year_date_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_year_date_arithmetic, "tests/datatype/interval/interval_year_date_arithmetic.test");

    // tests/datatype/interval/interval_year_month_arithmetic.test
    sqllogic_test!(test_datatype_interval_interval_year_month_arithmetic, "tests/datatype/interval/interval_year_month_arithmetic.test");

    // tests/datatype/interval/interval_year_month_basic.test
    sqllogic_test!(test_datatype_interval_interval_year_month_basic, "tests/datatype/interval/interval_year_month_basic.test");

    // tests/datatype/json/json_array.test
    sqllogic_test!(test_datatype_json_json_array, "tests/datatype/json/json_array.test");

    // tests/datatype/json/json_basic.test
    sqllogic_test!(test_datatype_json_json_basic, "tests/datatype/json/json_basic.test");

    // tests/datatype/json/json_nested.test
    sqllogic_test!(test_datatype_json_json_nested, "tests/datatype/json/json_nested.test");

    // tests/datatype/json/json_null.test
    sqllogic_test!(test_datatype_json_json_null, "tests/datatype/json/json_null.test");

    // tests/datatype/json/json_types.test
    sqllogic_test!(test_datatype_json_json_types, "tests/datatype/json/json_types.test");

    // tests/datatype/json/json_update.test
    sqllogic_test!(test_datatype_json_json_update, "tests/datatype/json/json_update.test");

    // tests/datatype/jsonb/jsonb_array.test
    sqllogic_test!(test_datatype_jsonb_jsonb_array, "tests/datatype/jsonb/jsonb_array.test");

    // tests/datatype/jsonb/jsonb_basic.test
    sqllogic_test!(test_datatype_jsonb_jsonb_basic, "tests/datatype/jsonb/jsonb_basic.test");

    // tests/datatype/jsonb/jsonb_nested.test
    sqllogic_test!(test_datatype_jsonb_jsonb_nested, "tests/datatype/jsonb/jsonb_nested.test");

    // tests/datatype/jsonb/jsonb_null.test
    sqllogic_test!(test_datatype_jsonb_jsonb_null, "tests/datatype/jsonb/jsonb_null.test");

    // tests/datatype/jsonb/jsonb_operations.test
    sqllogic_test!(test_datatype_jsonb_jsonb_operations, "tests/datatype/jsonb/jsonb_operations.test");

    // tests/datatype/jsonb/jsonb_update.test
    sqllogic_test!(test_datatype_jsonb_jsonb_update, "tests/datatype/jsonb/jsonb_update.test");

    // tests/datatype/long/long_basic.test
    sqllogic_test!(test_datatype_long_long_basic, "tests/datatype/long/long_basic.test");

    // tests/datatype/long/longvarchar_basic.test
    sqllogic_test!(test_datatype_long_longvarchar_basic, "tests/datatype/long/longvarchar_basic.test");

    // tests/datatype/longvarbinary/longvarbinary_basic.test
    sqllogic_test!(test_datatype_longvarbinary_longvarbinary_basic, "tests/datatype/longvarbinary/longvarbinary_basic.test");

    // tests/datatype/longvarbinary/longvarbinary_null.test
    sqllogic_test!(test_datatype_longvarbinary_longvarbinary_null, "tests/datatype/longvarbinary/longvarbinary_null.test");

    // tests/datatype/numeric/01_numeric_basic.test
    sqllogic_test!(test_datatype_numeric_01_numeric_basic, "tests/datatype/numeric/01_numeric_basic.test");

    // tests/datatype/numeric/02_numeric_precision.test
    sqllogic_test!(test_datatype_numeric_02_numeric_precision, "tests/datatype/numeric/02_numeric_precision.test");

    // tests/datatype/numeric/03_numeric_arithmetic.test
    sqllogic_test!(test_datatype_numeric_03_numeric_arithmetic, "tests/datatype/numeric/03_numeric_arithmetic.test");

    // tests/datatype/numeric/04_numeric_null.test
    sqllogic_test!(test_datatype_numeric_04_numeric_null, "tests/datatype/numeric/04_numeric_null.test");

    // tests/datatype/numeric/05_numeric_default.test
    sqllogic_test!(test_datatype_numeric_05_numeric_default, "tests/datatype/numeric/05_numeric_default.test");

    // tests/datatype/numeric/06_numeric_comparison.test
    sqllogic_test!(test_datatype_numeric_06_numeric_comparison, "tests/datatype/numeric/06_numeric_comparison.test");

    // tests/datatype/numeric/07_numeric_aggregate.test
    sqllogic_test!(test_datatype_numeric_07_numeric_aggregate, "tests/datatype/numeric/07_numeric_aggregate.test");

    // tests/datatype/numeric/08_numeric_ordering.test
    sqllogic_test!(test_datatype_numeric_08_numeric_ordering, "tests/datatype/numeric/08_numeric_ordering.test");

    // tests/datatype/numeric/09_numeric_rounding.test
    sqllogic_test!(test_datatype_numeric_09_numeric_rounding, "tests/datatype/numeric/09_numeric_rounding.test");

    // tests/datatype/numeric/10_numeric_negative.test
    sqllogic_test!(test_datatype_numeric_10_numeric_negative, "tests/datatype/numeric/10_numeric_negative.test");

    // tests/datatype/numeric/11_numeric_constraints.test
    sqllogic_test!(test_datatype_numeric_11_numeric_constraints, "tests/datatype/numeric/11_numeric_constraints.test");

    // tests/datatype/numeric/12_numeric_update.test
    sqllogic_test!(test_datatype_numeric_12_numeric_update, "tests/datatype/numeric/12_numeric_update.test");

    // tests/datatype/numeric/13_numeric_index.test
    sqllogic_test!(test_datatype_numeric_13_numeric_index, "tests/datatype/numeric/13_numeric_index.test");

    // tests/datatype/numeric/14_numeric_division.test
    sqllogic_test!(test_datatype_numeric_14_numeric_division, "tests/datatype/numeric/14_numeric_division.test");

    // tests/datatype/numeric/15_numeric_scale_zero.test
    sqllogic_test!(test_datatype_numeric_15_numeric_scale_zero, "tests/datatype/numeric/15_numeric_scale_zero.test");

    // tests/datatype/real/01_real_basic.test
    sqllogic_test!(test_datatype_real_01_real_basic, "tests/datatype/real/01_real_basic.test");

    // tests/datatype/real/02_real_arithmetic.test
    sqllogic_test!(test_datatype_real_02_real_arithmetic, "tests/datatype/real/02_real_arithmetic.test");

    // tests/datatype/real/03_real_null.test
    sqllogic_test!(test_datatype_real_03_real_null, "tests/datatype/real/03_real_null.test");

    // tests/datatype/rowid/01_rowid_basic.test
    sqllogic_test!(test_datatype_rowid_01_rowid_basic, "tests/datatype/rowid/01_rowid_basic.test");

    // tests/datatype/rowid/02_rowid_select.test
    sqllogic_test!(test_datatype_rowid_02_rowid_select, "tests/datatype/rowid/02_rowid_select.test");

    // tests/datatype/rowid/03_rowid_update.test
    sqllogic_test!(test_datatype_rowid_03_rowid_update, "tests/datatype/rowid/03_rowid_update.test");

    // tests/datatype/rowid/04_rowid_delete.test
    sqllogic_test!(test_datatype_rowid_04_rowid_delete, "tests/datatype/rowid/04_rowid_delete.test");

    // tests/datatype/rowid/05_rowid_uniqueness.test
    sqllogic_test!(test_datatype_rowid_05_rowid_uniqueness, "tests/datatype/rowid/05_rowid_uniqueness.test");

    // tests/datatype/rowid/06_rowid_ordering.test
    sqllogic_test!(test_datatype_rowid_06_rowid_ordering, "tests/datatype/rowid/06_rowid_ordering.test");

    // tests/datatype/rowid/07_rowid_join.test
    sqllogic_test!(test_datatype_rowid_07_rowid_join, "tests/datatype/rowid/07_rowid_join.test");

    // tests/datatype/rowid/08_rowid_index.test
    sqllogic_test!(test_datatype_rowid_08_rowid_index, "tests/datatype/rowid/08_rowid_index.test");

    // tests/datatype/rowid/09_rowid_null.test
    sqllogic_test!(test_datatype_rowid_09_rowid_null, "tests/datatype/rowid/09_rowid_null.test");

    // tests/datatype/rowid/10_rowid_stability.test
    sqllogic_test!(test_datatype_rowid_10_rowid_stability, "tests/datatype/rowid/10_rowid_stability.test");

    // tests/datatype/rowid/11_rowid_comparison.test
    sqllogic_test!(test_datatype_rowid_11_rowid_comparison, "tests/datatype/rowid/11_rowid_comparison.test");

    // tests/datatype/rowid/12_rowid_multi_table.test
    sqllogic_test!(test_datatype_rowid_12_rowid_multi_table, "tests/datatype/rowid/12_rowid_multi_table.test");

    // tests/datatype/rowid/13_rowid_persistence.test
    sqllogic_test!(test_datatype_rowid_13_rowid_persistence, "tests/datatype/rowid/13_rowid_persistence.test");

    // tests/datatype/rowid/14_rowid_format.test
    sqllogic_test!(test_datatype_rowid_14_rowid_format, "tests/datatype/rowid/14_rowid_format.test");

    // tests/datatype/rowid/15_rowid_aggregate.test
    sqllogic_test!(test_datatype_rowid_15_rowid_aggregate, "tests/datatype/rowid/15_rowid_aggregate.test");

    // tests/datatype/rowid/16_rowid_subquery.test
    sqllogic_test!(test_datatype_rowid_16_rowid_subquery, "tests/datatype/rowid/16_rowid_subquery.test");

    // tests/datatype/rowid/17_rowid_distinct.test
    sqllogic_test!(test_datatype_rowid_17_rowid_distinct, "tests/datatype/rowid/17_rowid_distinct.test");

    // tests/datatype/rowid/18_rowid_transaction.test
    sqllogic_test!(test_datatype_rowid_18_rowid_transaction, "tests/datatype/rowid/18_rowid_transaction.test");

    // tests/datatype/rowid/19_rowid_performance.test
    sqllogic_test!(test_datatype_rowid_19_rowid_performance, "tests/datatype/rowid/19_rowid_performance.test");

    // tests/datatype/rowid/20_rowid_limits.test
    sqllogic_test!(test_datatype_rowid_20_rowid_limits, "tests/datatype/rowid/20_rowid_limits.test");

    // tests/datatype/smallint/01_smallint_basic.test
    sqllogic_test!(test_datatype_smallint_01_smallint_basic, "tests/datatype/smallint/01_smallint_basic.test");

    // tests/datatype/smallint/02_smallint_arithmetic.test
    sqllogic_test!(test_datatype_smallint_02_smallint_arithmetic, "tests/datatype/smallint/02_smallint_arithmetic.test");

    // tests/datatype/smallint/03_smallint_null.test
    sqllogic_test!(test_datatype_smallint_03_smallint_null, "tests/datatype/smallint/03_smallint_null.test");

    // tests/datatype/smallint/04_smallint_aggregate.test
    sqllogic_test!(test_datatype_smallint_04_smallint_aggregate, "tests/datatype/smallint/04_smallint_aggregate.test");

    // tests/datatype/text/text_basic.test
    sqllogic_test!(test_datatype_text_text_basic, "tests/datatype/text/text_basic.test");

    // tests/datatype/text/text_concat.test
    sqllogic_test!(test_datatype_text_text_concat, "tests/datatype/text/text_concat.test");

    // tests/datatype/text/text_functions.test
    sqllogic_test!(test_datatype_text_text_functions, "tests/datatype/text/text_functions.test");

    // tests/datatype/text/text_large_data.test
    sqllogic_test!(test_datatype_text_text_large_data, "tests/datatype/text/text_large_data.test");

    // tests/datatype/text/text_null.test
    sqllogic_test!(test_datatype_text_text_null, "tests/datatype/text/text_null.test");

    // tests/datatype/text/text_update.test
    sqllogic_test!(test_datatype_text_text_update, "tests/datatype/text/text_update.test");

    // tests/datatype/time/time_arithmetic.test
    sqllogic_test!(test_datatype_time_time_arithmetic, "tests/datatype/time/time_arithmetic.test");

    // tests/datatype/time/time_basic.test
    sqllogic_test!(test_datatype_time_time_basic, "tests/datatype/time/time_basic.test");

    // tests/datatype/time/time_boundary.test
    sqllogic_test!(test_datatype_time_time_boundary, "tests/datatype/time/time_boundary.test");

    // tests/datatype/time/time_comparison.test
    sqllogic_test!(test_datatype_time_time_comparison, "tests/datatype/time/time_comparison.test");

    // tests/datatype/time/time_null.test
    sqllogic_test!(test_datatype_time_time_null, "tests/datatype/time/time_null.test");

    // tests/datatype/time/time_precision.test
    sqllogic_test!(test_datatype_time_time_precision, "tests/datatype/time/time_precision.test");

    // tests/datatype/time_with_timezone/time_tz_basic.test
    sqllogic_test!(test_datatype_time_with_timezone_time_tz_basic, "tests/datatype/time_with_timezone/time_tz_basic.test");

    // tests/datatype/time_with_timezone/time_tz_comparison.test
    sqllogic_test!(test_datatype_time_with_timezone_time_tz_comparison, "tests/datatype/time_with_timezone/time_tz_comparison.test");

    // tests/datatype/time_with_timezone/time_tz_null.test
    sqllogic_test!(test_datatype_time_with_timezone_time_tz_null, "tests/datatype/time_with_timezone/time_tz_null.test");

    // tests/datatype/time_with_timezone/time_tz_range.test
    sqllogic_test!(test_datatype_time_with_timezone_time_tz_range, "tests/datatype/time_with_timezone/time_tz_range.test");

    // tests/datatype/timestamp/timestamp_arithmetic.test
    sqllogic_test!(test_datatype_timestamp_timestamp_arithmetic, "tests/datatype/timestamp/timestamp_arithmetic.test");

    // tests/datatype/timestamp/timestamp_basic.test
    sqllogic_test!(test_datatype_timestamp_timestamp_basic, "tests/datatype/timestamp/timestamp_basic.test");

    // tests/datatype/timestamp/timestamp_boundary.test
    sqllogic_test!(test_datatype_timestamp_timestamp_boundary, "tests/datatype/timestamp/timestamp_boundary.test");

    // tests/datatype/timestamp/timestamp_comparison.test
    sqllogic_test!(test_datatype_timestamp_timestamp_comparison, "tests/datatype/timestamp/timestamp_comparison.test");

    // tests/datatype/timestamp/timestamp_null.test
    sqllogic_test!(test_datatype_timestamp_timestamp_null, "tests/datatype/timestamp/timestamp_null.test");

    // tests/datatype/timestamp/timestamp_precision.test
    sqllogic_test!(test_datatype_timestamp_timestamp_precision, "tests/datatype/timestamp/timestamp_precision.test");

    // tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_basic.test
    sqllogic_test!(test_datatype_timestamp_with_local_timezone_timestamp_local_tz_basic, "tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_basic.test");

    // tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_comparison.test
    sqllogic_test!(test_datatype_timestamp_with_local_timezone_timestamp_local_tz_comparison, "tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_comparison.test");

    // tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_handling.test
    sqllogic_test!(test_datatype_timestamp_with_local_timezone_timestamp_local_tz_handling, "tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_handling.test");

    // tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_null.test
    sqllogic_test!(test_datatype_timestamp_with_local_timezone_timestamp_local_tz_null, "tests/datatype/timestamp_with_local_timezone/timestamp_local_tz_null.test");

    // tests/datatype/timestamp_with_timezone/timestamp_tz_basic.test
    sqllogic_test!(test_datatype_timestamp_with_timezone_timestamp_tz_basic, "tests/datatype/timestamp_with_timezone/timestamp_tz_basic.test");

    // tests/datatype/timestamp_with_timezone/timestamp_tz_comparison.test
    sqllogic_test!(test_datatype_timestamp_with_timezone_timestamp_tz_comparison, "tests/datatype/timestamp_with_timezone/timestamp_tz_comparison.test");

    // tests/datatype/timestamp_with_timezone/timestamp_tz_conversion.test
    sqllogic_test!(test_datatype_timestamp_with_timezone_timestamp_tz_conversion, "tests/datatype/timestamp_with_timezone/timestamp_tz_conversion.test");

    // tests/datatype/timestamp_with_timezone/timestamp_tz_null.test
    sqllogic_test!(test_datatype_timestamp_with_timezone_timestamp_tz_null, "tests/datatype/timestamp_with_timezone/timestamp_tz_null.test");

    // tests/datatype/tinyint/01_tinyint_basic.test
    sqllogic_test!(test_datatype_tinyint_01_tinyint_basic, "tests/datatype/tinyint/01_tinyint_basic.test");

    // tests/datatype/tinyint/02_tinyint_range.test
    sqllogic_test!(test_datatype_tinyint_02_tinyint_range, "tests/datatype/tinyint/02_tinyint_range.test");

    // tests/datatype/tinyint/03_tinyint_arithmetic.test
    sqllogic_test!(test_datatype_tinyint_03_tinyint_arithmetic, "tests/datatype/tinyint/03_tinyint_arithmetic.test");

    // tests/datatype/tinyint/04_tinyint_null.test
    sqllogic_test!(test_datatype_tinyint_04_tinyint_null, "tests/datatype/tinyint/04_tinyint_null.test");

    // tests/datatype/varchar/01_varchar_basic.test
    sqllogic_test!(test_datatype_varchar_01_varchar_basic, "tests/datatype/varchar/01_varchar_basic.test");

    // tests/datatype/varchar/02_varchar_no_padding.test
    sqllogic_test!(test_datatype_varchar_02_varchar_no_padding, "tests/datatype/varchar/02_varchar_no_padding.test");

    // tests/datatype/varchar/03_varchar_max_length.test
    sqllogic_test!(test_datatype_varchar_03_varchar_max_length, "tests/datatype/varchar/03_varchar_max_length.test");

    // tests/datatype/varchar/04_varchar_null.test
    sqllogic_test!(test_datatype_varchar_04_varchar_null, "tests/datatype/varchar/04_varchar_null.test");

    // tests/datatype/varchar/05_varchar_default.test
    sqllogic_test!(test_datatype_varchar_05_varchar_default, "tests/datatype/varchar/05_varchar_default.test");

    // tests/datatype/varchar/06_varchar_comparison.test
    sqllogic_test!(test_datatype_varchar_06_varchar_comparison, "tests/datatype/varchar/06_varchar_comparison.test");

    // tests/datatype/varchar/07_varchar_ordering.test
    sqllogic_test!(test_datatype_varchar_07_varchar_ordering, "tests/datatype/varchar/07_varchar_ordering.test");

    // tests/datatype/varchar/08_varchar_update.test
    sqllogic_test!(test_datatype_varchar_08_varchar_update, "tests/datatype/varchar/08_varchar_update.test");

    // tests/datatype/varchar/09_varchar_delete.test
    sqllogic_test!(test_datatype_varchar_09_varchar_delete, "tests/datatype/varchar/09_varchar_delete.test");

    // tests/datatype/varchar/10_varchar_constraints.test
    sqllogic_test!(test_datatype_varchar_10_varchar_constraints, "tests/datatype/varchar/10_varchar_constraints.test");

    // tests/datatype/varchar/11_varchar_index.test
    sqllogic_test!(test_datatype_varchar_11_varchar_index, "tests/datatype/varchar/11_varchar_index.test");

    // tests/datatype/varchar/12_varchar_aggregate.test
    sqllogic_test!(test_datatype_varchar_12_varchar_aggregate, "tests/datatype/varchar/12_varchar_aggregate.test");

    // tests/datatype/varchar2/01_varchar2_basic.test
    sqllogic_test!(test_datatype_varchar2_01_varchar2_basic, "tests/datatype/varchar2/01_varchar2_basic.test");

    // tests/datatype/varchar2/02_varchar2_no_padding.test
    sqllogic_test!(test_datatype_varchar2_02_varchar2_no_padding, "tests/datatype/varchar2/02_varchar2_no_padding.test");

    // tests/datatype/varchar2/03_varchar2_max_length.test
    sqllogic_test!(test_datatype_varchar2_03_varchar2_max_length, "tests/datatype/varchar2/03_varchar2_max_length.test");

    // tests/datatype/varchar2/04_varchar2_null.test
    sqllogic_test!(test_datatype_varchar2_04_varchar2_null, "tests/datatype/varchar2/04_varchar2_null.test");

    // tests/datatype/varchar2/05_varchar2_default.test
    sqllogic_test!(test_datatype_varchar2_05_varchar2_default, "tests/datatype/varchar2/05_varchar2_default.test");

    // tests/datatype/varchar2/06_varchar2_comparison.test
    sqllogic_test!(test_datatype_varchar2_06_varchar2_comparison, "tests/datatype/varchar2/06_varchar2_comparison.test");

    // tests/datatype/varchar2/07_varchar2_update.test
    sqllogic_test!(test_datatype_varchar2_07_varchar2_update, "tests/datatype/varchar2/07_varchar2_update.test");

    // tests/datatype/varchar2/08_varchar2_index.test
    sqllogic_test!(test_datatype_varchar2_08_varchar2_index, "tests/datatype/varchar2/08_varchar2_index.test");

    // tests/express/datetime_expressions/01_date_add_interval_year.test
    sqllogic_test!(test_express_datetime_expressions_01_date_add_interval_year, "tests/express/datetime_expressions/01_date_add_interval_year.test");

    // tests/express/datetime_expressions/02_date_subtract_interval_year.test
    sqllogic_test!(test_express_datetime_expressions_02_date_subtract_interval_year, "tests/express/datetime_expressions/02_date_subtract_interval_year.test");

    // tests/express/datetime_expressions/03_interval_year_add_date.test
    sqllogic_test!(test_express_datetime_expressions_03_interval_year_add_date, "tests/express/datetime_expressions/03_interval_year_add_date.test");

    // tests/express/datetime_expressions/04_date_add_interval_month.test
    sqllogic_test!(test_express_datetime_expressions_04_date_add_interval_month, "tests/express/datetime_expressions/04_date_add_interval_month.test");

    // tests/express/datetime_expressions/05_date_subtract_interval_month.test
    sqllogic_test!(test_express_datetime_expressions_05_date_subtract_interval_month, "tests/express/datetime_expressions/05_date_subtract_interval_month.test");

    // tests/express/datetime_expressions/06_date_add_interval_year_to_month.test
    sqllogic_test!(test_express_datetime_expressions_06_date_add_interval_year_to_month, "tests/express/datetime_expressions/06_date_add_interval_year_to_month.test");

    // tests/express/datetime_expressions/07_date_add_interval_day.test
    sqllogic_test!(test_express_datetime_expressions_07_date_add_interval_day, "tests/express/datetime_expressions/07_date_add_interval_day.test");

    // tests/express/datetime_expressions/08_date_subtract_interval_day.test
    sqllogic_test!(test_express_datetime_expressions_08_date_subtract_interval_day, "tests/express/datetime_expressions/08_date_subtract_interval_day.test");

    // tests/express/datetime_expressions/09_time_add_interval_hour.test
    sqllogic_test!(test_express_datetime_expressions_09_time_add_interval_hour, "tests/express/datetime_expressions/09_time_add_interval_hour.test");

    // tests/express/datetime_expressions/10_time_subtract_interval_hour.test
    sqllogic_test!(test_express_datetime_expressions_10_time_subtract_interval_hour, "tests/express/datetime_expressions/10_time_subtract_interval_hour.test");

    // tests/express/datetime_expressions/11_time_add_interval_minute.test
    sqllogic_test!(test_express_datetime_expressions_11_time_add_interval_minute, "tests/express/datetime_expressions/11_time_add_interval_minute.test");

    // tests/express/datetime_expressions/12_time_subtract_interval_minute.test
    sqllogic_test!(test_express_datetime_expressions_12_time_subtract_interval_minute, "tests/express/datetime_expressions/12_time_subtract_interval_minute.test");

    // tests/express/datetime_expressions/13_time_add_interval_second.test
    sqllogic_test!(test_express_datetime_expressions_13_time_add_interval_second, "tests/express/datetime_expressions/13_time_add_interval_second.test");

    // tests/express/datetime_expressions/14_time_add_interval_day_to_hour.test
    sqllogic_test!(test_express_datetime_expressions_14_time_add_interval_day_to_hour, "tests/express/datetime_expressions/14_time_add_interval_day_to_hour.test");

    // tests/express/datetime_expressions/15_timestamp_add_interval_hour.test
    sqllogic_test!(test_express_datetime_expressions_15_timestamp_add_interval_hour, "tests/express/datetime_expressions/15_timestamp_add_interval_hour.test");

    // tests/express/datetime_expressions/16_timestamp_subtract_interval_hour.test
    sqllogic_test!(test_express_datetime_expressions_16_timestamp_subtract_interval_hour, "tests/express/datetime_expressions/16_timestamp_subtract_interval_hour.test");

    // tests/express/datetime_expressions/17_timestamp_add_interval_day.test
    sqllogic_test!(test_express_datetime_expressions_17_timestamp_add_interval_day, "tests/express/datetime_expressions/17_timestamp_add_interval_day.test");

    // tests/express/datetime_expressions/18_timestamp_add_interval_year.test
    sqllogic_test!(test_express_datetime_expressions_18_timestamp_add_interval_year, "tests/express/datetime_expressions/18_timestamp_add_interval_year.test");

    // tests/express/datetime_expressions/19_timestamp_add_interval_month.test
    sqllogic_test!(test_express_datetime_expressions_19_timestamp_add_interval_month, "tests/express/datetime_expressions/19_timestamp_add_interval_month.test");

    // tests/express/datetime_expressions/20_timestamp_add_interval_day_to_second.test
    sqllogic_test!(test_express_datetime_expressions_20_timestamp_add_interval_day_to_second, "tests/express/datetime_expressions/20_timestamp_add_interval_day_to_second.test");

    // tests/express/datetime_expressions/21_date_add_numeric.test
    sqllogic_test!(test_express_datetime_expressions_21_date_add_numeric, "tests/express/datetime_expressions/21_date_add_numeric.test");

    // tests/express/datetime_expressions/22_date_subtract_numeric.test
    sqllogic_test!(test_express_datetime_expressions_22_date_subtract_numeric, "tests/express/datetime_expressions/22_date_subtract_numeric.test");

    // tests/express/datetime_expressions/23_numeric_add_date.test
    sqllogic_test!(test_express_datetime_expressions_23_numeric_add_date, "tests/express/datetime_expressions/23_numeric_add_date.test");

    // tests/express/datetime_expressions/24_timestamp_add_numeric.test
    sqllogic_test!(test_express_datetime_expressions_24_timestamp_add_numeric, "tests/express/datetime_expressions/24_timestamp_add_numeric.test");

    // tests/express/datetime_expressions/25_timestamp_subtract_numeric.test
    sqllogic_test!(test_express_datetime_expressions_25_timestamp_subtract_numeric, "tests/express/datetime_expressions/25_timestamp_subtract_numeric.test");

    // tests/express/datetime_expressions/26_numeric_add_timestamp.test
    sqllogic_test!(test_express_datetime_expressions_26_numeric_add_timestamp, "tests/express/datetime_expressions/26_numeric_add_timestamp.test");

    // tests/express/datetime_expressions/27_timestamp_at_timezone_offset.test
    sqllogic_test!(test_express_datetime_expressions_27_timestamp_at_timezone_offset, "tests/express/datetime_expressions/27_timestamp_at_timezone_offset.test");

    // tests/express/datetime_expressions/28_timestamp_at_timezone_region.test
    sqllogic_test!(test_express_datetime_expressions_28_timestamp_at_timezone_region, "tests/express/datetime_expressions/28_timestamp_at_timezone_region.test");

    // tests/express/datetime_expressions/29_date_add_negative_interval.test
    sqllogic_test!(test_express_datetime_expressions_29_date_add_negative_interval, "tests/express/datetime_expressions/29_date_add_negative_interval.test");

    // tests/express/datetime_expressions/30_date_subtract_negative_interval.test
    sqllogic_test!(test_express_datetime_expressions_30_date_subtract_negative_interval, "tests/express/datetime_expressions/30_date_subtract_negative_interval.test");

    // tests/express/datetime_expressions/31_time_overflow_addition.test
    sqllogic_test!(test_express_datetime_expressions_31_time_overflow_addition, "tests/express/datetime_expressions/31_time_overflow_addition.test");

    // tests/express/datetime_expressions/32_time_underflow_subtraction.test
    sqllogic_test!(test_express_datetime_expressions_32_time_underflow_subtraction, "tests/express/datetime_expressions/32_time_underflow_subtraction.test");

    // tests/express/datetime_expressions/33_timestamp_add_zero_interval.test
    sqllogic_test!(test_express_datetime_expressions_33_timestamp_add_zero_interval, "tests/express/datetime_expressions/33_timestamp_add_zero_interval.test");

    // tests/express/datetime_expressions/34_date_add_zero.test
    sqllogic_test!(test_express_datetime_expressions_34_date_add_zero, "tests/express/datetime_expressions/34_date_add_zero.test");

    // tests/express/datetime_expressions/35_timestamp_cross_month.test
    sqllogic_test!(test_express_datetime_expressions_35_timestamp_cross_month, "tests/express/datetime_expressions/35_timestamp_cross_month.test");

    // tests/express/datetime_expressions/36_timestamp_cross_year.test
    sqllogic_test!(test_express_datetime_expressions_36_timestamp_cross_year, "tests/express/datetime_expressions/36_timestamp_cross_year.test");

    // tests/express/datetime_expressions/37_date_leap_year.test
    sqllogic_test!(test_express_datetime_expressions_37_date_leap_year, "tests/express/datetime_expressions/37_date_leap_year.test");

    // tests/express/datetime_expressions/38_time_cross_hour.test
    sqllogic_test!(test_express_datetime_expressions_38_time_cross_hour, "tests/express/datetime_expressions/38_time_cross_hour.test");

    // tests/express/datetime_expressions/39_time_cross_minute.test
    sqllogic_test!(test_express_datetime_expressions_39_time_cross_minute, "tests/express/datetime_expressions/39_time_cross_minute.test");

    // tests/express/datetime_expressions/40_timestamp_fractional_second.test
    sqllogic_test!(test_express_datetime_expressions_40_timestamp_fractional_second, "tests/express/datetime_expressions/40_timestamp_fractional_second.test");

    // tests/express/datetime_expressions/41_date_add_large_number.test
    sqllogic_test!(test_express_datetime_expressions_41_date_add_large_number, "tests/express/datetime_expressions/41_date_add_large_number.test");

    // tests/express/datetime_expressions/42_date_subtract_large_number.test
    sqllogic_test!(test_express_datetime_expressions_42_date_subtract_large_number, "tests/express/datetime_expressions/42_date_subtract_large_number.test");

    // tests/express/datetime_expressions/43_timestamp_add_fractional_day.test
    sqllogic_test!(test_express_datetime_expressions_43_timestamp_add_fractional_day, "tests/express/datetime_expressions/43_timestamp_add_fractional_day.test");

    // tests/express/datetime_expressions/44_time_midnight_addition.test
    sqllogic_test!(test_express_datetime_expressions_44_time_midnight_addition, "tests/express/datetime_expressions/44_time_midnight_addition.test");

    // tests/express/datetime_expressions/45_time_before_midnight.test
    sqllogic_test!(test_express_datetime_expressions_45_time_before_midnight, "tests/express/datetime_expressions/45_time_before_midnight.test");

    // tests/express/datetime_expressions/46_timestamp_min_date.test
    sqllogic_test!(test_express_datetime_expressions_46_timestamp_min_date, "tests/express/datetime_expressions/46_timestamp_min_date.test");

    // tests/express/datetime_expressions/47_timestamp_max_date.test
    sqllogic_test!(test_express_datetime_expressions_47_timestamp_max_date, "tests/express/datetime_expressions/47_timestamp_max_date.test");

    // tests/express/datetime_expressions/48_date_cross_month.test
    sqllogic_test!(test_express_datetime_expressions_48_date_cross_month, "tests/express/datetime_expressions/48_date_cross_month.test");

    // tests/express/datetime_expressions/49_date_month_end_handling.test
    sqllogic_test!(test_express_datetime_expressions_49_date_month_end_handling, "tests/express/datetime_expressions/49_date_month_end_handling.test");

    // tests/express/datetime_expressions/50_date_leap_year_month.test
    sqllogic_test!(test_express_datetime_expressions_50_date_leap_year_month, "tests/express/datetime_expressions/50_date_leap_year_month.test");

    // tests/express/datetime_expressions/51_timestamp_add_negative_number.test
    sqllogic_test!(test_express_datetime_expressions_51_timestamp_add_negative_number, "tests/express/datetime_expressions/51_timestamp_add_negative_number.test");

    // tests/express/datetime_expressions/52_time_add_day_to_second.test
    sqllogic_test!(test_express_datetime_expressions_52_time_add_day_to_second, "tests/express/datetime_expressions/52_time_add_day_to_second.test");

    // tests/express/datetime_expressions/53_time_add_hour_to_minute.test
    sqllogic_test!(test_express_datetime_expressions_53_time_add_hour_to_minute, "tests/express/datetime_expressions/53_time_add_hour_to_minute.test");

    // tests/express/datetime_expressions/54_time_add_hour_to_second.test
    sqllogic_test!(test_express_datetime_expressions_54_time_add_hour_to_second, "tests/express/datetime_expressions/54_time_add_hour_to_second.test");

    // tests/express/datetime_expressions/55_time_add_minute_to_second.test
    sqllogic_test!(test_express_datetime_expressions_55_time_add_minute_to_second, "tests/express/datetime_expressions/55_time_add_minute_to_second.test");

    // tests/express/datetime_expressions/56_timestamp_add_day_to_minute.test
    sqllogic_test!(test_express_datetime_expressions_56_timestamp_add_day_to_minute, "tests/express/datetime_expressions/56_timestamp_add_day_to_minute.test");

    // tests/express/datetime_expressions/57_timestamp_add_hour_to_minute.test
    sqllogic_test!(test_express_datetime_expressions_57_timestamp_add_hour_to_minute, "tests/express/datetime_expressions/57_timestamp_add_hour_to_minute.test");

    // tests/express/datetime_expressions/58_timestamp_add_hour_to_second.test
    sqllogic_test!(test_express_datetime_expressions_58_timestamp_add_hour_to_second, "tests/express/datetime_expressions/58_timestamp_add_hour_to_second.test");

    // tests/express/datetime_expressions/59_timestamp_add_minute_to_second.test
    sqllogic_test!(test_express_datetime_expressions_59_timestamp_add_minute_to_second, "tests/express/datetime_expressions/59_timestamp_add_minute_to_second.test");

    // tests/express/datetime_expressions/60_timestamp_add_year_to_month.test
    sqllogic_test!(test_express_datetime_expressions_60_timestamp_add_year_to_month, "tests/express/datetime_expressions/60_timestamp_add_year_to_month.test");

    // tests/express/interval_expressions/01_date_subtract_date_year_to_month.test
    sqllogic_test!(test_express_interval_expressions_01_date_subtract_date_year_to_month, "tests/express/interval_expressions/01_date_subtract_date_year_to_month.test");

    // tests/express/interval_expressions/02_date_subtract_date_year.test
    sqllogic_test!(test_express_interval_expressions_02_date_subtract_date_year, "tests/express/interval_expressions/02_date_subtract_date_year.test");

    // tests/express/interval_expressions/03_date_subtract_date_month.test
    sqllogic_test!(test_express_interval_expressions_03_date_subtract_date_month, "tests/express/interval_expressions/03_date_subtract_date_month.test");

    // tests/express/interval_expressions/04_date_subtract_date_day.test
    sqllogic_test!(test_express_interval_expressions_04_date_subtract_date_day, "tests/express/interval_expressions/04_date_subtract_date_day.test");

    // tests/express/interval_expressions/05_time_subtract_time_hour.test
    sqllogic_test!(test_express_interval_expressions_05_time_subtract_time_hour, "tests/express/interval_expressions/05_time_subtract_time_hour.test");

    // tests/express/interval_expressions/06_time_subtract_time_minute.test
    sqllogic_test!(test_express_interval_expressions_06_time_subtract_time_minute, "tests/express/interval_expressions/06_time_subtract_time_minute.test");

    // tests/express/interval_expressions/07_time_subtract_time_second.test
    sqllogic_test!(test_express_interval_expressions_07_time_subtract_time_second, "tests/express/interval_expressions/07_time_subtract_time_second.test");

    // tests/express/interval_expressions/08_time_subtract_time_hour_to_minute.test
    sqllogic_test!(test_express_interval_expressions_08_time_subtract_time_hour_to_minute, "tests/express/interval_expressions/08_time_subtract_time_hour_to_minute.test");

    // tests/express/interval_expressions/09_time_subtract_time_hour_to_second.test
    sqllogic_test!(test_express_interval_expressions_09_time_subtract_time_hour_to_second, "tests/express/interval_expressions/09_time_subtract_time_hour_to_second.test");

    // tests/express/interval_expressions/10_time_subtract_time_minute_to_second.test
    sqllogic_test!(test_express_interval_expressions_10_time_subtract_time_minute_to_second, "tests/express/interval_expressions/10_time_subtract_time_minute_to_second.test");

    // tests/express/interval_expressions/11_timestamp_subtract_timestamp_hour.test
    sqllogic_test!(test_express_interval_expressions_11_timestamp_subtract_timestamp_hour, "tests/express/interval_expressions/11_timestamp_subtract_timestamp_hour.test");

    // tests/express/interval_expressions/12_timestamp_subtract_timestamp_day.test
    sqllogic_test!(test_express_interval_expressions_12_timestamp_subtract_timestamp_day, "tests/express/interval_expressions/12_timestamp_subtract_timestamp_day.test");

    // tests/express/interval_expressions/13_timestamp_subtract_timestamp_day_to_second.test
    sqllogic_test!(test_express_interval_expressions_13_timestamp_subtract_timestamp_day_to_second, "tests/express/interval_expressions/13_timestamp_subtract_timestamp_day_to_second.test");

    // tests/express/interval_expressions/14_timestamp_subtract_timestamp_year_to_month.test
    sqllogic_test!(test_express_interval_expressions_14_timestamp_subtract_timestamp_year_to_month, "tests/express/interval_expressions/14_timestamp_subtract_timestamp_year_to_month.test");

    // tests/express/interval_expressions/15_year_month_interval_add.test
    sqllogic_test!(test_express_interval_expressions_15_year_month_interval_add, "tests/express/interval_expressions/15_year_month_interval_add.test");

    // tests/express/interval_expressions/16_year_month_interval_subtract.test
    sqllogic_test!(test_express_interval_expressions_16_year_month_interval_subtract, "tests/express/interval_expressions/16_year_month_interval_subtract.test");

    // tests/express/interval_expressions/17_year_interval_add_year_interval.test
    sqllogic_test!(test_express_interval_expressions_17_year_interval_add_year_interval, "tests/express/interval_expressions/17_year_interval_add_year_interval.test");

    // tests/express/interval_expressions/18_month_interval_add_month_interval.test
    sqllogic_test!(test_express_interval_expressions_18_month_interval_add_month_interval, "tests/express/interval_expressions/18_month_interval_add_month_interval.test");

    // tests/express/interval_expressions/19_day_time_interval_add.test
    sqllogic_test!(test_express_interval_expressions_19_day_time_interval_add, "tests/express/interval_expressions/19_day_time_interval_add.test");

    // tests/express/interval_expressions/20_day_time_interval_subtract.test
    sqllogic_test!(test_express_interval_expressions_20_day_time_interval_subtract, "tests/express/interval_expressions/20_day_time_interval_subtract.test");

    // tests/express/interval_expressions/21_day_interval_add_day_interval.test
    sqllogic_test!(test_express_interval_expressions_21_day_interval_add_day_interval, "tests/express/interval_expressions/21_day_interval_add_day_interval.test");

    // tests/express/interval_expressions/22_hour_interval_add_hour_interval.test
    sqllogic_test!(test_express_interval_expressions_22_hour_interval_add_hour_interval, "tests/express/interval_expressions/22_hour_interval_add_hour_interval.test");

    // tests/express/interval_expressions/23_minute_interval_add_minute_interval.test
    sqllogic_test!(test_express_interval_expressions_23_minute_interval_add_minute_interval, "tests/express/interval_expressions/23_minute_interval_add_minute_interval.test");

    // tests/express/interval_expressions/24_second_interval_add_second_interval.test
    sqllogic_test!(test_express_interval_expressions_24_second_interval_add_second_interval, "tests/express/interval_expressions/24_second_interval_add_second_interval.test");

    // tests/express/interval_expressions/25_interval_multiply_number.test
    sqllogic_test!(test_express_interval_expressions_25_interval_multiply_number, "tests/express/interval_expressions/25_interval_multiply_number.test");

    // tests/express/interval_expressions/26_interval_divide_number.test
    sqllogic_test!(test_express_interval_expressions_26_interval_divide_number, "tests/express/interval_expressions/26_interval_divide_number.test");

    // tests/express/interval_expressions/27_number_multiply_interval.test
    sqllogic_test!(test_express_interval_expressions_27_number_multiply_interval, "tests/express/interval_expressions/27_number_multiply_interval.test");

    // tests/express/interval_expressions/28_day_to_second_interval_multiply.test
    sqllogic_test!(test_express_interval_expressions_28_day_to_second_interval_multiply, "tests/express/interval_expressions/28_day_to_second_interval_multiply.test");

    // tests/express/interval_expressions/29_year_to_month_multiply.test
    sqllogic_test!(test_express_interval_expressions_29_year_to_month_multiply, "tests/express/interval_expressions/29_year_to_month_multiply.test");

    // tests/express/interval_expressions/30_hour_to_second_divide.test
    sqllogic_test!(test_express_interval_expressions_30_hour_to_second_divide, "tests/express/interval_expressions/30_hour_to_second_divide.test");

    // tests/express/interval_expressions/31_interval_add_different_precision.test
    sqllogic_test!(test_express_interval_expressions_31_interval_add_different_precision, "tests/express/interval_expressions/31_interval_add_different_precision.test");

    // tests/express/interval_expressions/32_interval_subtract_different_precision.test
    sqllogic_test!(test_express_interval_expressions_32_interval_subtract_different_precision, "tests/express/interval_expressions/32_interval_subtract_different_precision.test");

    // tests/express/interval_expressions/33_day_to_hour_add_hour_to_minute.test
    sqllogic_test!(test_express_interval_expressions_33_day_to_hour_add_hour_to_minute, "tests/express/interval_expressions/33_day_to_hour_add_hour_to_minute.test");

    // tests/express/interval_expressions/34_hour_to_minute_add_minute_to_second.test
    sqllogic_test!(test_express_interval_expressions_34_hour_to_minute_add_minute_to_second, "tests/express/interval_expressions/34_hour_to_minute_add_minute_to_second.test");

    // tests/express/interval_expressions/35_interval_multiply_zero.test
    sqllogic_test!(test_express_interval_expressions_35_interval_multiply_zero, "tests/express/interval_expressions/35_interval_multiply_zero.test");

    // tests/express/interval_expressions/36_interval_divide_one.test
    sqllogic_test!(test_express_interval_expressions_36_interval_divide_one, "tests/express/interval_expressions/36_interval_divide_one.test");

    // tests/express/interval_expressions/37_negative_interval_add.test
    sqllogic_test!(test_express_interval_expressions_37_negative_interval_add, "tests/express/interval_expressions/37_negative_interval_add.test");

    // tests/express/interval_expressions/38_negative_interval_subtract.test
    sqllogic_test!(test_express_interval_expressions_38_negative_interval_subtract, "tests/express/interval_expressions/38_negative_interval_subtract.test");

    // tests/express/interval_expressions/39_interval_multiply_negative.test
    sqllogic_test!(test_express_interval_expressions_39_interval_multiply_negative, "tests/express/interval_expressions/39_interval_multiply_negative.test");

    // tests/express/interval_expressions/40_interval_divide_negative.test
    sqllogic_test!(test_express_interval_expressions_40_interval_divide_negative, "tests/express/interval_expressions/40_interval_divide_negative.test");

    // tests/express/interval_expressions/41_large_year_interval_add.test
    sqllogic_test!(test_express_interval_expressions_41_large_year_interval_add, "tests/express/interval_expressions/41_large_year_interval_add.test");

    // tests/express/interval_expressions/42_large_month_interval_subtract.test
    sqllogic_test!(test_express_interval_expressions_42_large_month_interval_subtract, "tests/express/interval_expressions/42_large_month_interval_subtract.test");

    // tests/express/interval_expressions/43_day_to_minute_add_day_to_second.test
    sqllogic_test!(test_express_interval_expressions_43_day_to_minute_add_day_to_second, "tests/express/interval_expressions/43_day_to_minute_add_day_to_second.test");

    // tests/express/interval_expressions/44_fractional_multiply.test
    sqllogic_test!(test_express_interval_expressions_44_fractional_multiply, "tests/express/interval_expressions/44_fractional_multiply.test");

    // tests/express/interval_expressions/45_fractional_divide.test
    sqllogic_test!(test_express_interval_expressions_45_fractional_divide, "tests/express/interval_expressions/45_fractional_divide.test");

    // tests/express/interval_expressions/46_year_month_interval_overflow.test
    sqllogic_test!(test_express_interval_expressions_46_year_month_interval_overflow, "tests/express/interval_expressions/46_year_month_interval_overflow.test");

    // tests/express/interval_expressions/47_day_time_interval_hour_overflow.test
    sqllogic_test!(test_express_interval_expressions_47_day_time_interval_hour_overflow, "tests/express/interval_expressions/47_day_time_interval_hour_overflow.test");

    // tests/express/interval_expressions/48_minute_overflow_to_hour.test
    sqllogic_test!(test_express_interval_expressions_48_minute_overflow_to_hour, "tests/express/interval_expressions/48_minute_overflow_to_hour.test");

    // tests/express/interval_expressions/49_second_overflow_to_minute.test
    sqllogic_test!(test_express_interval_expressions_49_second_overflow_to_minute, "tests/express/interval_expressions/49_second_overflow_to_minute.test");

    // tests/express/interval_expressions/50_zero_interval_add.test
    sqllogic_test!(test_express_interval_expressions_50_zero_interval_add, "tests/express/interval_expressions/50_zero_interval_add.test");

    // tests/express/interval_expressions/51_interval_subtract_itself.test
    sqllogic_test!(test_express_interval_expressions_51_interval_subtract_itself, "tests/express/interval_expressions/51_interval_subtract_itself.test");

    // tests/express/interval_expressions/52_year_month_negative_result.test
    sqllogic_test!(test_express_interval_expressions_52_year_month_negative_result, "tests/express/interval_expressions/52_year_month_negative_result.test");

    // tests/express/interval_expressions/53_day_time_negative_result.test
    sqllogic_test!(test_express_interval_expressions_53_day_time_negative_result, "tests/express/interval_expressions/53_day_time_negative_result.test");

    // tests/express/interval_expressions/54_complex_day_to_second_add.test
    sqllogic_test!(test_express_interval_expressions_54_complex_day_to_second_add, "tests/express/interval_expressions/54_complex_day_to_second_add.test");

    // tests/express/interval_expressions/55_complex_day_to_second_subtract.test
    sqllogic_test!(test_express_interval_expressions_55_complex_day_to_second_subtract, "tests/express/interval_expressions/55_complex_day_to_second_subtract.test");

    // tests/express/interval_expressions/56_year_to_month_complex.test
    sqllogic_test!(test_express_interval_expressions_56_year_to_month_complex, "tests/express/interval_expressions/56_year_to_month_complex.test");

    // tests/express/interval_expressions/57_day_interval_multiply_large.test
    sqllogic_test!(test_express_interval_expressions_57_day_interval_multiply_large, "tests/express/interval_expressions/57_day_interval_multiply_large.test");

    // tests/express/interval_expressions/58_hour_interval_divide_small.test
    sqllogic_test!(test_express_interval_expressions_58_hour_interval_divide_small, "tests/express/interval_expressions/58_hour_interval_divide_small.test");

    // tests/express/interval_expressions/59_date_subtract_cross_year.test
    sqllogic_test!(test_express_interval_expressions_59_date_subtract_cross_year, "tests/express/interval_expressions/59_date_subtract_cross_year.test");

    // tests/express/interval_expressions/60_timestamp_subtract_cross_month.test
    sqllogic_test!(test_express_interval_expressions_60_timestamp_subtract_cross_month, "tests/express/interval_expressions/60_timestamp_subtract_cross_month.test");

    // tests/express/numeric_expressions/01_unary_plus_minus.test
    sqllogic_test!(test_express_numeric_expressions_01_unary_plus_minus, "tests/express/numeric_expressions/01_unary_plus_minus.test");

    // tests/express/numeric_expressions/02_unary_minus.test
    sqllogic_test!(test_express_numeric_expressions_02_unary_minus, "tests/express/numeric_expressions/02_unary_minus.test");

    // tests/express/numeric_expressions/03_double_negative.test
    sqllogic_test!(test_express_numeric_expressions_03_double_negative, "tests/express/numeric_expressions/03_double_negative.test");

    // tests/express/numeric_expressions/04_unary_bitwise_not.test
    sqllogic_test!(test_express_numeric_expressions_04_unary_bitwise_not, "tests/express/numeric_expressions/04_unary_bitwise_not.test");

    // tests/express/numeric_expressions/05_bitwise_not_zero.test
    sqllogic_test!(test_express_numeric_expressions_05_bitwise_not_zero, "tests/express/numeric_expressions/05_bitwise_not_zero.test");

    // tests/express/numeric_expressions/06_bitwise_not_negative.test
    sqllogic_test!(test_express_numeric_expressions_06_bitwise_not_negative, "tests/express/numeric_expressions/06_bitwise_not_negative.test");

    // tests/express/numeric_expressions/07_addition_integer.test
    sqllogic_test!(test_express_numeric_expressions_07_addition_integer, "tests/express/numeric_expressions/07_addition_integer.test");

    // tests/express/numeric_expressions/08_subtraction_integer.test
    sqllogic_test!(test_express_numeric_expressions_08_subtraction_integer, "tests/express/numeric_expressions/08_subtraction_integer.test");

    // tests/express/numeric_expressions/09_multiplication_integer.test
    sqllogic_test!(test_express_numeric_expressions_09_multiplication_integer, "tests/express/numeric_expressions/09_multiplication_integer.test");

    // tests/express/numeric_expressions/10_division_integer.test
    sqllogic_test!(test_express_numeric_expressions_10_division_integer, "tests/express/numeric_expressions/10_division_integer.test");

    // tests/express/numeric_expressions/11_modulo_integer.test
    sqllogic_test!(test_express_numeric_expressions_11_modulo_integer, "tests/express/numeric_expressions/11_modulo_integer.test");

    // tests/express/numeric_expressions/12_numeric_addition_precision.test
    sqllogic_test!(test_express_numeric_expressions_12_numeric_addition_precision, "tests/express/numeric_expressions/12_numeric_addition_precision.test");

    // tests/express/numeric_expressions/13_numeric_subtraction_precision.test
    sqllogic_test!(test_express_numeric_expressions_13_numeric_subtraction_precision, "tests/express/numeric_expressions/13_numeric_subtraction_precision.test");

    // tests/express/numeric_expressions/14_numeric_multiplication_precision.test
    sqllogic_test!(test_express_numeric_expressions_14_numeric_multiplication_precision, "tests/express/numeric_expressions/14_numeric_multiplication_precision.test");

    // tests/express/numeric_expressions/15_numeric_division.test
    sqllogic_test!(test_express_numeric_expressions_15_numeric_division, "tests/express/numeric_expressions/15_numeric_division.test");

    // tests/express/numeric_expressions/16_approximate_addition.test
    sqllogic_test!(test_express_numeric_expressions_16_approximate_addition, "tests/express/numeric_expressions/16_approximate_addition.test");

    // tests/express/numeric_expressions/17_approximate_subtraction.test
    sqllogic_test!(test_express_numeric_expressions_17_approximate_subtraction, "tests/express/numeric_expressions/17_approximate_subtraction.test");

    // tests/express/numeric_expressions/18_approximate_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_18_approximate_multiplication, "tests/express/numeric_expressions/18_approximate_multiplication.test");

    // tests/express/numeric_expressions/19_approximate_division.test
    sqllogic_test!(test_express_numeric_expressions_19_approximate_division, "tests/express/numeric_expressions/19_approximate_division.test");

    // tests/express/numeric_expressions/20_bitwise_and.test
    sqllogic_test!(test_express_numeric_expressions_20_bitwise_and, "tests/express/numeric_expressions/20_bitwise_and.test");

    // tests/express/numeric_expressions/21_bitwise_or.test
    sqllogic_test!(test_express_numeric_expressions_21_bitwise_or, "tests/express/numeric_expressions/21_bitwise_or.test");

    // tests/express/numeric_expressions/22_bitwise_xor.test
    sqllogic_test!(test_express_numeric_expressions_22_bitwise_xor, "tests/express/numeric_expressions/22_bitwise_xor.test");

    // tests/express/numeric_expressions/23_left_shift.test
    sqllogic_test!(test_express_numeric_expressions_23_left_shift, "tests/express/numeric_expressions/23_left_shift.test");

    // tests/express/numeric_expressions/24_right_shift.test
    sqllogic_test!(test_express_numeric_expressions_24_right_shift, "tests/express/numeric_expressions/24_right_shift.test");

    // tests/express/numeric_expressions/25_shift_smallint_result_type.test
    sqllogic_test!(test_express_numeric_expressions_25_shift_smallint_result_type, "tests/express/numeric_expressions/25_shift_smallint_result_type.test");

    // tests/express/numeric_expressions/26_shift_int_result_type.test
    sqllogic_test!(test_express_numeric_expressions_26_shift_int_result_type, "tests/express/numeric_expressions/26_shift_int_result_type.test");

    // tests/express/numeric_expressions/27_shift_bigint_result_type.test
    sqllogic_test!(test_express_numeric_expressions_27_shift_bigint_result_type, "tests/express/numeric_expressions/27_shift_bigint_result_type.test");

    // tests/express/numeric_expressions/28_shift_mixed_bigint_result.test
    sqllogic_test!(test_express_numeric_expressions_28_shift_mixed_bigint_result, "tests/express/numeric_expressions/28_shift_mixed_bigint_result.test");

    // tests/express/numeric_expressions/29_shift_numeric_rounding.test
    sqllogic_test!(test_express_numeric_expressions_29_shift_numeric_rounding, "tests/express/numeric_expressions/29_shift_numeric_rounding.test");

    // tests/express/numeric_expressions/30_shift_int_numeric.test
    sqllogic_test!(test_express_numeric_expressions_30_shift_int_numeric, "tests/express/numeric_expressions/30_shift_int_numeric.test");

    // tests/express/numeric_expressions/31_integer_type_promotion.test
    sqllogic_test!(test_express_numeric_expressions_31_integer_type_promotion, "tests/express/numeric_expressions/31_integer_type_promotion.test");

    // tests/express/numeric_expressions/32_integer_numeric_precision.test
    sqllogic_test!(test_express_numeric_expressions_32_integer_numeric_precision, "tests/express/numeric_expressions/32_integer_numeric_precision.test");

    // tests/express/numeric_expressions/33_smallint_numeric_precision.test
    sqllogic_test!(test_express_numeric_expressions_33_smallint_numeric_precision, "tests/express/numeric_expressions/33_smallint_numeric_precision.test");

    // tests/express/numeric_expressions/34_bigint_numeric_precision.test
    sqllogic_test!(test_express_numeric_expressions_34_bigint_numeric_precision, "tests/express/numeric_expressions/34_bigint_numeric_precision.test");

    // tests/express/numeric_expressions/35_scale_rounding.test
    sqllogic_test!(test_express_numeric_expressions_35_scale_rounding, "tests/express/numeric_expressions/35_scale_rounding.test");

    // tests/express/numeric_expressions/36_precedence_unary_plus.test
    sqllogic_test!(test_express_numeric_expressions_36_precedence_unary_plus, "tests/express/numeric_expressions/36_precedence_unary_plus.test");

    // tests/express/numeric_expressions/37_precedence_multiply_add.test
    sqllogic_test!(test_express_numeric_expressions_37_precedence_multiply_add, "tests/express/numeric_expressions/37_precedence_multiply_add.test");

    // tests/express/numeric_expressions/38_precedence_divide_subtract.test
    sqllogic_test!(test_express_numeric_expressions_38_precedence_divide_subtract, "tests/express/numeric_expressions/38_precedence_divide_subtract.test");

    // tests/express/numeric_expressions/39_precedence_parentheses.test
    sqllogic_test!(test_express_numeric_expressions_39_precedence_parentheses, "tests/express/numeric_expressions/39_precedence_parentheses.test");

    // tests/express/numeric_expressions/40_precedence_shift_and.test
    sqllogic_test!(test_express_numeric_expressions_40_precedence_shift_and, "tests/express/numeric_expressions/40_precedence_shift_and.test");

    // tests/express/numeric_expressions/41_precedence_and_xor.test
    sqllogic_test!(test_express_numeric_expressions_41_precedence_and_xor, "tests/express/numeric_expressions/41_precedence_and_xor.test");

    // tests/express/numeric_expressions/42_precedence_xor_or.test
    sqllogic_test!(test_express_numeric_expressions_42_precedence_xor_or, "tests/express/numeric_expressions/42_precedence_xor_or.test");

    // tests/express/numeric_expressions/43_divide_by_zero.test
    sqllogic_test!(test_express_numeric_expressions_43_divide_by_zero, "tests/express/numeric_expressions/43_divide_by_zero.test");

    // tests/express/numeric_expressions/44_integer_divide_by_zero.test
    sqllogic_test!(test_express_numeric_expressions_44_integer_divide_by_zero, "tests/express/numeric_expressions/44_integer_divide_by_zero.test");

    // tests/express/numeric_expressions/45_modulo_by_zero.test
    sqllogic_test!(test_express_numeric_expressions_45_modulo_by_zero, "tests/express/numeric_expressions/45_modulo_by_zero.test");

    // tests/express/numeric_expressions/46_null_addition.test
    sqllogic_test!(test_express_numeric_expressions_46_null_addition, "tests/express/numeric_expressions/46_null_addition.test");

    // tests/express/numeric_expressions/47_null_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_47_null_multiplication, "tests/express/numeric_expressions/47_null_multiplication.test");

    // tests/express/numeric_expressions/48_null_bitwise_and.test
    sqllogic_test!(test_express_numeric_expressions_48_null_bitwise_and, "tests/express/numeric_expressions/48_null_bitwise_and.test");

    // tests/express/numeric_expressions/49_negative_modulo.test
    sqllogic_test!(test_express_numeric_expressions_49_negative_modulo, "tests/express/numeric_expressions/49_negative_modulo.test");

    // tests/express/numeric_expressions/50_negative_bitwise_and.test
    sqllogic_test!(test_express_numeric_expressions_50_negative_bitwise_and, "tests/express/numeric_expressions/50_negative_bitwise_and.test");

    // tests/express/numeric_expressions/51_negative_left_shift.test
    sqllogic_test!(test_express_numeric_expressions_51_negative_left_shift, "tests/express/numeric_expressions/51_negative_left_shift.test");

    // tests/express/numeric_expressions/52_negative_right_shift.test
    sqllogic_test!(test_express_numeric_expressions_52_negative_right_shift, "tests/express/numeric_expressions/52_negative_right_shift.test");

    // tests/express/numeric_expressions/53_tinyint_overflow.test
    sqllogic_test!(test_express_numeric_expressions_53_tinyint_overflow, "tests/express/numeric_expressions/53_tinyint_overflow.test");

    // tests/express/numeric_expressions/54_smallint_overflow.test
    sqllogic_test!(test_express_numeric_expressions_54_smallint_overflow, "tests/express/numeric_expressions/54_smallint_overflow.test");

    // tests/express/numeric_expressions/55_integer_overflow.test
    sqllogic_test!(test_express_numeric_expressions_55_integer_overflow, "tests/express/numeric_expressions/55_integer_overflow.test");

    // tests/express/numeric_expressions/56_zero_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_56_zero_multiplication, "tests/express/numeric_expressions/56_zero_multiplication.test");

    // tests/express/numeric_expressions/57_one_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_57_one_multiplication, "tests/express/numeric_expressions/57_one_multiplication.test");

    // tests/express/numeric_expressions/58_zero_addition.test
    sqllogic_test!(test_express_numeric_expressions_58_zero_addition, "tests/express/numeric_expressions/58_zero_addition.test");

    // tests/express/numeric_expressions/59_self_subtraction.test
    sqllogic_test!(test_express_numeric_expressions_59_self_subtraction, "tests/express/numeric_expressions/59_self_subtraction.test");

    // tests/express/numeric_expressions/60_self_bitwise_and.test
    sqllogic_test!(test_express_numeric_expressions_60_self_bitwise_and, "tests/express/numeric_expressions/60_self_bitwise_and.test");

    // tests/express/numeric_expressions/61_self_bitwise_or.test
    sqllogic_test!(test_express_numeric_expressions_61_self_bitwise_or, "tests/express/numeric_expressions/61_self_bitwise_or.test");

    // tests/express/numeric_expressions/62_self_bitwise_xor.test
    sqllogic_test!(test_express_numeric_expressions_62_self_bitwise_xor, "tests/express/numeric_expressions/62_self_bitwise_xor.test");

    // tests/express/numeric_expressions/63_left_shift_zero.test
    sqllogic_test!(test_express_numeric_expressions_63_left_shift_zero, "tests/express/numeric_expressions/63_left_shift_zero.test");

    // tests/express/numeric_expressions/64_right_shift_zero.test
    sqllogic_test!(test_express_numeric_expressions_64_right_shift_zero, "tests/express/numeric_expressions/64_right_shift_zero.test");

    // tests/express/numeric_expressions/65_bitwise_and_zero.test
    sqllogic_test!(test_express_numeric_expressions_65_bitwise_and_zero, "tests/express/numeric_expressions/65_bitwise_and_zero.test");

    // tests/express/numeric_expressions/66_bitwise_or_zero.test
    sqllogic_test!(test_express_numeric_expressions_66_bitwise_or_zero, "tests/express/numeric_expressions/66_bitwise_or_zero.test");

    // tests/express/numeric_expressions/67_bitwise_xor_zero.test
    sqllogic_test!(test_express_numeric_expressions_67_bitwise_xor_zero, "tests/express/numeric_expressions/67_bitwise_xor_zero.test");

    // tests/express/numeric_expressions/68_float_integer_addition.test
    sqllogic_test!(test_express_numeric_expressions_68_float_integer_addition, "tests/express/numeric_expressions/68_float_integer_addition.test");

    // tests/express/numeric_expressions/69_real_integer_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_69_real_integer_multiplication, "tests/express/numeric_expressions/69_real_integer_multiplication.test");

    // tests/express/numeric_expressions/70_decimal_integer_addition.test
    sqllogic_test!(test_express_numeric_expressions_70_decimal_integer_addition, "tests/express/numeric_expressions/70_decimal_integer_addition.test");

    // tests/express/numeric_expressions/71_dec_smallint_subtraction.test
    sqllogic_test!(test_express_numeric_expressions_71_dec_smallint_subtraction, "tests/express/numeric_expressions/71_dec_smallint_subtraction.test");

    // tests/express/numeric_expressions/72_number_bigint_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_72_number_bigint_multiplication, "tests/express/numeric_expressions/72_number_bigint_multiplication.test");

    // tests/express/numeric_expressions/73_complex_expression_1.test
    sqllogic_test!(test_express_numeric_expressions_73_complex_expression_1, "tests/express/numeric_expressions/73_complex_expression_1.test");

    // tests/express/numeric_expressions/74_complex_expression_2.test
    sqllogic_test!(test_express_numeric_expressions_74_complex_expression_2, "tests/express/numeric_expressions/74_complex_expression_2.test");

    // tests/express/numeric_expressions/75_complex_expression_3.test
    sqllogic_test!(test_express_numeric_expressions_75_complex_expression_3, "tests/express/numeric_expressions/75_complex_expression_3.test");

    // tests/express/numeric_expressions/76_decimal_multiplication_precision.test
    sqllogic_test!(test_express_numeric_expressions_76_decimal_multiplication_precision, "tests/express/numeric_expressions/76_decimal_multiplication_precision.test");

    // tests/express/numeric_expressions/77_large_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_77_large_multiplication, "tests/express/numeric_expressions/77_large_multiplication.test");

    // tests/express/numeric_expressions/78_decimal_division_precision.test
    sqllogic_test!(test_express_numeric_expressions_78_decimal_division_precision, "tests/express/numeric_expressions/78_decimal_division_precision.test");

    // tests/express/numeric_expressions/79_positive_modulo_positive.test
    sqllogic_test!(test_express_numeric_expressions_79_positive_modulo_positive, "tests/express/numeric_expressions/79_positive_modulo_positive.test");

    // tests/express/numeric_expressions/80_negative_modulo_positive.test
    sqllogic_test!(test_express_numeric_expressions_80_negative_modulo_positive, "tests/express/numeric_expressions/80_negative_modulo_positive.test");

    // tests/express/numeric_expressions/81_positive_modulo_negative.test
    sqllogic_test!(test_express_numeric_expressions_81_positive_modulo_negative, "tests/express/numeric_expressions/81_positive_modulo_negative.test");

    // tests/express/numeric_expressions/82_negative_modulo_negative.test
    sqllogic_test!(test_express_numeric_expressions_82_negative_modulo_negative, "tests/express/numeric_expressions/82_negative_modulo_negative.test");

    // tests/express/numeric_expressions/83_large_left_shift.test
    sqllogic_test!(test_express_numeric_expressions_83_large_left_shift, "tests/express/numeric_expressions/83_large_left_shift.test");

    // tests/express/numeric_expressions/84_large_right_shift.test
    sqllogic_test!(test_express_numeric_expressions_84_large_right_shift, "tests/express/numeric_expressions/84_large_right_shift.test");

    // tests/express/numeric_expressions/85_bitwise_and_all_ones.test
    sqllogic_test!(test_express_numeric_expressions_85_bitwise_and_all_ones, "tests/express/numeric_expressions/85_bitwise_and_all_ones.test");

    // tests/express/numeric_expressions/86_bitwise_or_all_ones.test
    sqllogic_test!(test_express_numeric_expressions_86_bitwise_or_all_ones, "tests/express/numeric_expressions/86_bitwise_or_all_ones.test");

    // tests/express/numeric_expressions/87_consecutive_addition.test
    sqllogic_test!(test_express_numeric_expressions_87_consecutive_addition, "tests/express/numeric_expressions/87_consecutive_addition.test");

    // tests/express/numeric_expressions/88_consecutive_multiplication.test
    sqllogic_test!(test_express_numeric_expressions_88_consecutive_multiplication, "tests/express/numeric_expressions/88_consecutive_multiplication.test");

    // tests/express/numeric_expressions/89_numeric_double_mixed.test
    sqllogic_test!(test_express_numeric_expressions_89_numeric_double_mixed, "tests/express/numeric_expressions/89_numeric_double_mixed.test");

    // tests/express/numeric_expressions/90_decimal_float_mixed.test
    sqllogic_test!(test_express_numeric_expressions_90_decimal_float_mixed, "tests/express/numeric_expressions/90_decimal_float_mixed.test");

    // tests/express/numeric_expressions/91_shift_basic.test
    sqllogic_test!(test_express_numeric_expressions_91_shift_basic, "tests/express/numeric_expressions/91_shift_basic.test");

    // tests/express/operator_precedence/01_parentheses_highest.test
    sqllogic_test!(test_express_operator_precedence_01_parentheses_highest, "tests/express/operator_precedence/01_parentheses_highest.test");

    // tests/express/operator_precedence/02_multiply_before_add.test
    sqllogic_test!(test_express_operator_precedence_02_multiply_before_add, "tests/express/operator_precedence/02_multiply_before_add.test");

    // tests/express/operator_precedence/03_unary_minus_before_multiply.test
    sqllogic_test!(test_express_operator_precedence_03_unary_minus_before_multiply, "tests/express/operator_precedence/03_unary_minus_before_multiply.test");

    // tests/express/operator_precedence/04_unary_plus_before_add.test
    sqllogic_test!(test_express_operator_precedence_04_unary_plus_before_add, "tests/express/operator_precedence/04_unary_plus_before_add.test");

    // tests/express/operator_precedence/05_bitwise_not_before_multiply.test
    sqllogic_test!(test_express_operator_precedence_05_bitwise_not_before_multiply, "tests/express/operator_precedence/05_bitwise_not_before_multiply.test");

    // tests/express/operator_precedence/06_multiply_divide_left_to_right.test
    sqllogic_test!(test_express_operator_precedence_06_multiply_divide_left_to_right, "tests/express/operator_precedence/06_multiply_divide_left_to_right.test");

    // tests/express/operator_precedence/07_divide_mod_left_to_right.test
    sqllogic_test!(test_express_operator_precedence_07_divide_mod_left_to_right, "tests/express/operator_precedence/07_divide_mod_left_to_right.test");

    // tests/express/operator_precedence/08_multiply_before_addition.test
    sqllogic_test!(test_express_operator_precedence_08_multiply_before_addition, "tests/express/operator_precedence/08_multiply_before_addition.test");

    // tests/express/operator_precedence/09_divide_before_subtract.test
    sqllogic_test!(test_express_operator_precedence_09_divide_before_subtract, "tests/express/operator_precedence/09_divide_before_subtract.test");

    // tests/express/operator_precedence/10_mod_before_add.test
    sqllogic_test!(test_express_operator_precedence_10_mod_before_add, "tests/express/operator_precedence/10_mod_before_add.test");

    // tests/express/operator_precedence/11_add_subtract_left_to_right.test
    sqllogic_test!(test_express_operator_precedence_11_add_subtract_left_to_right, "tests/express/operator_precedence/11_add_subtract_left_to_right.test");

    // tests/express/operator_precedence/12_add_before_left_shift.test
    sqllogic_test!(test_express_operator_precedence_12_add_before_left_shift, "tests/express/operator_precedence/12_add_before_left_shift.test");

    // tests/express/operator_precedence/13_subtract_before_right_shift.test
    sqllogic_test!(test_express_operator_precedence_13_subtract_before_right_shift, "tests/express/operator_precedence/13_subtract_before_right_shift.test");

    // tests/express/operator_precedence/14_shift_left_to_right.test
    sqllogic_test!(test_express_operator_precedence_14_shift_left_to_right, "tests/express/operator_precedence/14_shift_left_to_right.test");

    // tests/express/operator_precedence/15_left_shift_before_and.test
    sqllogic_test!(test_express_operator_precedence_15_left_shift_before_and, "tests/express/operator_precedence/15_left_shift_before_and.test");

    // tests/express/operator_precedence/16_right_shift_before_and.test
    sqllogic_test!(test_express_operator_precedence_16_right_shift_before_and, "tests/express/operator_precedence/16_right_shift_before_and.test");

    // tests/express/operator_precedence/17_and_before_xor.test
    sqllogic_test!(test_express_operator_precedence_17_and_before_xor, "tests/express/operator_precedence/17_and_before_xor.test");

    // tests/express/operator_precedence/18_xor_before_or.test
    sqllogic_test!(test_express_operator_precedence_18_xor_before_or, "tests/express/operator_precedence/18_xor_before_or.test");

    // tests/express/operator_precedence/19_complex_arithmetic.test
    sqllogic_test!(test_express_operator_precedence_19_complex_arithmetic, "tests/express/operator_precedence/19_complex_arithmetic.test");

    // tests/express/operator_precedence/20_complex_bitwise.test
    sqllogic_test!(test_express_operator_precedence_20_complex_bitwise, "tests/express/operator_precedence/20_complex_bitwise.test");

    // tests/express/operator_precedence/21_arithmetic_bitwise_mix.test
    sqllogic_test!(test_express_operator_precedence_21_arithmetic_bitwise_mix, "tests/express/operator_precedence/21_arithmetic_bitwise_mix.test");

    // tests/express/operator_precedence/22_unary_binary_mix.test
    sqllogic_test!(test_express_operator_precedence_22_unary_binary_mix, "tests/express/operator_precedence/22_unary_binary_mix.test");

    // tests/express/operator_precedence/23_nested_parentheses.test
    sqllogic_test!(test_express_operator_precedence_23_nested_parentheses, "tests/express/operator_precedence/23_nested_parentheses.test");

    // tests/express/operator_precedence/24_parentheses_change_multiply.test
    sqllogic_test!(test_express_operator_precedence_24_parentheses_change_multiply, "tests/express/operator_precedence/24_parentheses_change_multiply.test");

    // tests/express/operator_precedence/25_parentheses_change_bitwise.test
    sqllogic_test!(test_express_operator_precedence_25_parentheses_change_bitwise, "tests/express/operator_precedence/25_parentheses_change_bitwise.test");

    // tests/express/operator_precedence/26_unary_minus_multiply.test
    sqllogic_test!(test_express_operator_precedence_26_unary_minus_multiply, "tests/express/operator_precedence/26_unary_minus_multiply.test");

    // tests/express/operator_precedence/27_bitwise_not_add.test
    sqllogic_test!(test_express_operator_precedence_27_bitwise_not_add, "tests/express/operator_precedence/27_bitwise_not_add.test");

    // tests/express/operator_precedence/28_multiply_divide_mod_mix.test
    sqllogic_test!(test_express_operator_precedence_28_multiply_divide_mod_mix, "tests/express/operator_precedence/28_multiply_divide_mod_mix.test");

    // tests/express/operator_precedence/29_add_subtract_sequence.test
    sqllogic_test!(test_express_operator_precedence_29_add_subtract_sequence, "tests/express/operator_precedence/29_add_subtract_sequence.test");

    // tests/express/operator_precedence/30_multiply_add_subtract_combo.test
    sqllogic_test!(test_express_operator_precedence_30_multiply_add_subtract_combo, "tests/express/operator_precedence/30_multiply_add_subtract_combo.test");

    // tests/express/operator_precedence/31_shift_and_precedence.test
    sqllogic_test!(test_express_operator_precedence_31_shift_and_precedence, "tests/express/operator_precedence/31_shift_and_precedence.test");

    // tests/express/operator_precedence/32_and_xor_or_combo.test
    sqllogic_test!(test_express_operator_precedence_32_and_xor_or_combo, "tests/express/operator_precedence/32_and_xor_or_combo.test");

    // tests/express/operator_precedence/33_all_operators_precedence.test
    sqllogic_test!(test_express_operator_precedence_33_all_operators_precedence, "tests/express/operator_precedence/33_all_operators_precedence.test");

    // tests/express/operator_precedence/34_parentheses_override_all.test
    sqllogic_test!(test_express_operator_precedence_34_parentheses_override_all, "tests/express/operator_precedence/34_parentheses_override_all.test");

    // tests/express/operator_precedence/35_double_negative.test
    sqllogic_test!(test_express_operator_precedence_35_double_negative, "tests/express/operator_precedence/35_double_negative.test");

    // tests/express/operator_precedence/36_multiple_unary.test
    sqllogic_test!(test_express_operator_precedence_36_multiple_unary, "tests/express/operator_precedence/36_multiple_unary.test");

    // tests/express/operator_precedence/37_divide_subtract.test
    sqllogic_test!(test_express_operator_precedence_37_divide_subtract, "tests/express/operator_precedence/37_divide_subtract.test");

    // tests/express/operator_precedence/38_mod_add.test
    sqllogic_test!(test_express_operator_precedence_38_mod_add, "tests/express/operator_precedence/38_mod_add.test");

    // tests/express/operator_precedence/39_shift_sequence.test
    sqllogic_test!(test_express_operator_precedence_39_shift_sequence, "tests/express/operator_precedence/39_shift_sequence.test");

    // tests/express/operator_precedence/40_and_sequence.test
    sqllogic_test!(test_express_operator_precedence_40_and_sequence, "tests/express/operator_precedence/40_and_sequence.test");

    // tests/express/operator_precedence/41_xor_sequence.test
    sqllogic_test!(test_express_operator_precedence_41_xor_sequence, "tests/express/operator_precedence/41_xor_sequence.test");

    // tests/express/operator_precedence/42_or_sequence.test
    sqllogic_test!(test_express_operator_precedence_42_or_sequence, "tests/express/operator_precedence/42_or_sequence.test");

    // tests/express/operator_precedence/43_complex_grouping_1.test
    sqllogic_test!(test_express_operator_precedence_43_complex_grouping_1, "tests/express/operator_precedence/43_complex_grouping_1.test");

    // tests/express/operator_precedence/44_complex_grouping_2.test
    sqllogic_test!(test_express_operator_precedence_44_complex_grouping_2, "tests/express/operator_precedence/44_complex_grouping_2.test");

    // tests/express/operator_precedence/45_full_mix.test
    sqllogic_test!(test_express_operator_precedence_45_full_mix, "tests/express/operator_precedence/45_full_mix.test");

    // tests/express/operator_precedence/46_unary_right_associative.test
    sqllogic_test!(test_express_operator_precedence_46_unary_right_associative, "tests/express/operator_precedence/46_unary_right_associative.test");

    // tests/express/operator_precedence/47_parentheses_unary_combo.test
    sqllogic_test!(test_express_operator_precedence_47_parentheses_unary_combo, "tests/express/operator_precedence/47_parentheses_unary_combo.test");

    // tests/express/operator_precedence/48_multiply_divide_mod_continuous.test
    sqllogic_test!(test_express_operator_precedence_48_multiply_divide_mod_continuous, "tests/express/operator_precedence/48_multiply_divide_mod_continuous.test");

    // tests/express/operator_precedence/49_long_expression.test
    sqllogic_test!(test_express_operator_precedence_49_long_expression, "tests/express/operator_precedence/49_long_expression.test");

    // tests/express/operator_precedence/50_nested_parentheses_complex.test
    sqllogic_test!(test_express_operator_precedence_50_nested_parentheses_complex, "tests/express/operator_precedence/50_nested_parentheses_complex.test");

    // tests/express/string_expressions/01_basic_concatenation.test
    sqllogic_test!(test_express_string_expressions_01_basic_concatenation, "tests/express/string_expressions/01_basic_concatenation.test");

    // tests/express/string_expressions/02_english_concatenation.test
    sqllogic_test!(test_express_string_expressions_02_english_concatenation, "tests/express/string_expressions/02_english_concatenation.test");

    // tests/express/string_expressions/03_concatenation_with_space.test
    sqllogic_test!(test_express_string_expressions_03_concatenation_with_space, "tests/express/string_expressions/03_concatenation_with_space.test");

    // tests/express/string_expressions/04_empty_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_04_empty_string_concatenation, "tests/express/string_expressions/04_empty_string_concatenation.test");

    // tests/express/string_expressions/05_string_empty_concatenation.test
    sqllogic_test!(test_express_string_expressions_05_string_empty_concatenation, "tests/express/string_expressions/05_string_empty_concatenation.test");

    // tests/express/string_expressions/06_empty_empty_concatenation.test
    sqllogic_test!(test_express_string_expressions_06_empty_empty_concatenation, "tests/express/string_expressions/06_empty_empty_concatenation.test");

    // tests/express/string_expressions/07_null_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_07_null_string_concatenation, "tests/express/string_expressions/07_null_string_concatenation.test");

    // tests/express/string_expressions/08_string_null_concatenation.test
    sqllogic_test!(test_express_string_expressions_08_string_null_concatenation, "tests/express/string_expressions/08_string_null_concatenation.test");

    // tests/express/string_expressions/09_null_null_concatenation.test
    sqllogic_test!(test_express_string_expressions_09_null_null_concatenation, "tests/express/string_expressions/09_null_null_concatenation.test");

    // tests/express/string_expressions/10_multiple_concatenation.test
    sqllogic_test!(test_express_string_expressions_10_multiple_concatenation, "tests/express/string_expressions/10_multiple_concatenation.test");

    // tests/express/string_expressions/11_char_concatenation.test
    sqllogic_test!(test_express_string_expressions_11_char_concatenation, "tests/express/string_expressions/11_char_concatenation.test");

    // tests/express/string_expressions/12_varchar_concatenation.test
    sqllogic_test!(test_express_string_expressions_12_varchar_concatenation, "tests/express/string_expressions/12_varchar_concatenation.test");

    // tests/express/string_expressions/13_varchar2_concatenation.test
    sqllogic_test!(test_express_string_expressions_13_varchar2_concatenation, "tests/express/string_expressions/13_varchar2_concatenation.test");

    // tests/express/string_expressions/14_char_varchar_mixed.test
    sqllogic_test!(test_express_string_expressions_14_char_varchar_mixed, "tests/express/string_expressions/14_char_varchar_mixed.test");

    // tests/express/string_expressions/15_number_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_15_number_string_concatenation, "tests/express/string_expressions/15_number_string_concatenation.test");

    // tests/express/string_expressions/16_decimal_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_16_decimal_string_concatenation, "tests/express/string_expressions/16_decimal_string_concatenation.test");

    // tests/express/string_expressions/17_date_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_17_date_string_concatenation, "tests/express/string_expressions/17_date_string_concatenation.test");

    // tests/express/string_expressions/18_timestamp_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_18_timestamp_string_concatenation, "tests/express/string_expressions/18_timestamp_string_concatenation.test");

    // tests/express/string_expressions/19_special_chars_concatenation.test
    sqllogic_test!(test_express_string_expressions_19_special_chars_concatenation, "tests/express/string_expressions/19_special_chars_concatenation.test");

    // tests/express/string_expressions/20_newline_concatenation.test
    sqllogic_test!(test_express_string_expressions_20_newline_concatenation, "tests/express/string_expressions/20_newline_concatenation.test");

    // tests/express/string_expressions/21_tab_concatenation.test
    sqllogic_test!(test_express_string_expressions_21_tab_concatenation, "tests/express/string_expressions/21_tab_concatenation.test");

    // tests/express/string_expressions/22_single_quote_concatenation.test
    sqllogic_test!(test_express_string_expressions_22_single_quote_concatenation, "tests/express/string_expressions/22_single_quote_concatenation.test");

    // tests/express/string_expressions/23_long_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_23_long_string_concatenation, "tests/express/string_expressions/23_long_string_concatenation.test");

    // tests/express/string_expressions/24_chinese_english_mixed.test
    sqllogic_test!(test_express_string_expressions_24_chinese_english_mixed, "tests/express/string_expressions/24_chinese_english_mixed.test");

    // tests/express/string_expressions/25_numeric_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_25_numeric_string_concatenation, "tests/express/string_expressions/25_numeric_string_concatenation.test");

    // tests/express/string_expressions/26_column_concatenation.test
    sqllogic_test!(test_express_string_expressions_26_column_concatenation, "tests/express/string_expressions/26_column_concatenation.test");

    // tests/express/string_expressions/27_column_literal_concatenation.test
    sqllogic_test!(test_express_string_expressions_27_column_literal_concatenation, "tests/express/string_expressions/27_column_literal_concatenation.test");

    // tests/express/string_expressions/28_multiple_column_concatenation.test
    sqllogic_test!(test_express_string_expressions_28_multiple_column_concatenation, "tests/express/string_expressions/28_multiple_column_concatenation.test");

    // tests/express/string_expressions/29_null_column_concatenation.test
    sqllogic_test!(test_express_string_expressions_29_null_column_concatenation, "tests/express/string_expressions/29_null_column_concatenation.test");

    // tests/express/string_expressions/30_concatenation_length.test
    sqllogic_test!(test_express_string_expressions_30_concatenation_length, "tests/express/string_expressions/30_concatenation_length.test");

    // tests/express/string_expressions/31_space_concatenation.test
    sqllogic_test!(test_express_string_expressions_31_space_concatenation, "tests/express/string_expressions/31_space_concatenation.test");

    // tests/express/string_expressions/32_concatenation_comparison.test
    sqllogic_test!(test_express_string_expressions_32_concatenation_comparison, "tests/express/string_expressions/32_concatenation_comparison.test");

    // tests/express/string_expressions/33_case_sensitive_concatenation.test
    sqllogic_test!(test_express_string_expressions_33_case_sensitive_concatenation, "tests/express/string_expressions/33_case_sensitive_concatenation.test");

    // tests/express/string_expressions/34_concatenation_like_match.test
    sqllogic_test!(test_express_string_expressions_34_concatenation_like_match, "tests/express/string_expressions/34_concatenation_like_match.test");

    // tests/express/string_expressions/35_concatenation_upper.test
    sqllogic_test!(test_express_string_expressions_35_concatenation_upper, "tests/express/string_expressions/35_concatenation_upper.test");

    // tests/express/string_expressions/36_concatenation_lower.test
    sqllogic_test!(test_express_string_expressions_36_concatenation_lower, "tests/express/string_expressions/36_concatenation_lower.test");

    // tests/express/string_expressions/37_concatenation_trim.test
    sqllogic_test!(test_express_string_expressions_37_concatenation_trim, "tests/express/string_expressions/37_concatenation_trim.test");

    // tests/express/string_expressions/38_concatenation_substring.test
    sqllogic_test!(test_express_string_expressions_38_concatenation_substring, "tests/express/string_expressions/38_concatenation_substring.test");

    // tests/express/string_expressions/39_nested_concatenation.test
    sqllogic_test!(test_express_string_expressions_39_nested_concatenation, "tests/express/string_expressions/39_nested_concatenation.test");

    // tests/express/string_expressions/40_concatenation_order.test
    sqllogic_test!(test_express_string_expressions_40_concatenation_order, "tests/express/string_expressions/40_concatenation_order.test");

    // tests/express/string_expressions/41_unicode_concatenation.test
    sqllogic_test!(test_express_string_expressions_41_unicode_concatenation, "tests/express/string_expressions/41_unicode_concatenation.test");

    // tests/express/string_expressions/42_charset_concatenation.test
    sqllogic_test!(test_express_string_expressions_42_charset_concatenation, "tests/express/string_expressions/42_charset_concatenation.test");

    // tests/express/string_expressions/43_boolean_string_concatenation.test
    sqllogic_test!(test_express_string_expressions_43_boolean_string_concatenation, "tests/express/string_expressions/43_boolean_string_concatenation.test");

    // tests/express/string_expressions/44_zero_length_concatenation.test
    sqllogic_test!(test_express_string_expressions_44_zero_length_concatenation, "tests/express/string_expressions/44_zero_length_concatenation.test");

    // tests/express/string_expressions/45_performance_concatenation.test
    sqllogic_test!(test_express_string_expressions_45_performance_concatenation, "tests/express/string_expressions/45_performance_concatenation.test");

}
