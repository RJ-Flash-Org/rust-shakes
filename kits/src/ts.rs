//! Time-series kit (MVP: moving average).

use foundry::Result;
use rax::Array;

/// Simple moving average.
pub fn moving_average(values: &Array<f64>, window: usize) -> Result<Array<f64>> {
    if window == 0 || values.ndim() != 1 || values.len() < window {
        return Err(foundry::Error::InvalidArgument("bad moving_average args".into()));
    }
    let n = values.len() - window + 1;
    let mut out = Array::zeros([n]);
    for i in 0..n {
        let mut s = 0.0;
        for j in 0..window {
            s += values.get_flat(i + j)?;
        }
        out.set_flat(i, s / window as f64)?;
    }
    Ok(out)
}
