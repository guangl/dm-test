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

}
