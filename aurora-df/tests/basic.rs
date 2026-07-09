use aurora_df::{DataFrame, Expr, Series, filter};
use rax::Array;

#[test]
fn dataframe_filter_and_select() {
    let a = Series::new("a", Array::from_vec(vec![1.0, 2.0, 3.0, 4.0]));
    let b = Series::new("b", Array::from_vec(vec![10.0, 20.0, 30.0, 40.0]));
    let df = DataFrame::new(vec![a, b]).unwrap();
    let f = Expr::col("a").gt(2.0);
    let filtered = filter(&df, f).unwrap();
    assert_eq!(filtered.shape(), (2, 2));
    assert_eq!(filtered.get("a").unwrap().as_slice(), &[3.0, 4.0]);
}
