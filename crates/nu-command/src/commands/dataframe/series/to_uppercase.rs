use crate::{commands::dataframe::utils::parse_polars_error, prelude::*};
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{
    dataframe::{Column, NuDataFrame},
    Signature, UntaggedValue,
};
use polars::prelude::IntoSeries;

pub struct DataFrame;

impl WholeStreamCommand for DataFrame {
    fn name(&self) -> &str {
        "dataframe to-uppercase"
    }

    fn usage(&self) -> &str {
        "[Series] Uppercase the strings in the column"
    }

    fn signature(&self) -> Signature {
        Signature::build("dataframe to-uppercase")
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        command(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Modifies strings to uppercase",
            example: "[Abc aBc abC] | dataframe to-df | dataframe to-uppercase",
            result: Some(vec![NuDataFrame::try_from_columns(
                vec![Column::new(
                    "0".to_string(),
                    vec![
                        UntaggedValue::string("ABC").into(),
                        UntaggedValue::string("ABC").into(),
                        UntaggedValue::string("ABC").into(),
                    ],
                )],
                &Span::default(),
            )
            .expect("simple df for test should not fail")
            .into_value(Tag::default())]),
        }]
    }
}

fn command(mut args: CommandArgs) -> Result<OutputStream, ShellError> {
    let tag = args.call_info.name_tag.clone();

    let (df, df_tag) = NuDataFrame::try_from_stream(&mut args.input, &tag.span)?;

    let series = df.as_series(&df_tag.span)?;
    let chunked = series.utf8().map_err(|e| {
        parse_polars_error::<&str>(
            &e,
            &df_tag.span,
            Some("The to-uppercase command can only be used with string columns"),
        )
    })?;

    let mut res = chunked.to_uppercase();
    res.rename(series.name());

    let df = NuDataFrame::try_from_series(vec![res.into_series()], &tag.span)?;
    Ok(OutputStream::one(df.into_value(df_tag)))
}

#[cfg(test)]
mod tests {
    use super::DataFrame;
    use super::ShellError;

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        use crate::examples::test_dataframe as test_examples;

        test_examples(DataFrame {})
    }
}
