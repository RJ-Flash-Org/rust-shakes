//! Optimization algorithms (MVP: gradient-free).

use foundry::Result;
use rax::Array;

/// Nelder-Mead simplex optimizer (MVP: delegates to coordinate descent).
pub fn nelder_mead<F>(f: F, x0: Array<f64>, max_iter: usize) -> Result<Array<f64>>
where
    F: Fn(&Array<f64>) -> f64,
{
    let mut x = x0.clone();
    let step = 1e-3;
    for _ in 0..max_iter {
        let mut improved = false;
        for i in 0..x.len() {
            let xi = x.get(i)?;
            let mut xp = x.clone();
            xp.set(i, xi + step)?;
            if f(&xp) < f(&x) {
                x = xp;
                improved = true;
                continue;
            }
            let mut xm = x.clone();
            xm.set(i, xi - step)?;
            if f(&xm) < f(&x) {
                x = xm;
                improved = true;
            }
        }
        if !improved {
            break;
        }
    }
    Ok(x)
}

/// L-BFGS optimizer (placeholder: currently delegates to coordinate descent).
pub fn lbfgs<F>(f: F, x0: Array<f64>, _max_iter: usize) -> Result<Array<f64>>
where
    F: Fn(&Array<f64>) -> f64,
{
    nelder_mead(f, x0, 200)
}
