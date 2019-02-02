// utils
//-----------------------------------------------------------------------------

// Abs return the absolute value of x.
pub fn abs(x: f32) -> f32 {
    if x < 0.0 {
        return -x;
    }
    if x == 0.0 {
        return 0.0; // return correctly abs(-0)
    }
    return x;
}

//-----------------------------------------------------------------------------
