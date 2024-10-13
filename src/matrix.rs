use crate::widget::Widget;

pub(crate) type Matrix = [[u8; 9]; 34];

///
/// Encode a 9x34 array of booleans to a 39 byte (one bit per pixel) array
/// [0][0] starts in top left corner
///
pub fn encode(arr: [[bool; 9]; 34]) -> [u8; 39] {
    let mut out_arr: [u8; 39] = [0; 39];
    let mut index = 0;
    let mut byte_offs = 0;

    #[allow(clippy::needless_range_loop)]
    for i in 0..34 {
        for j in 0..9 {
            let newval = if arr[i][j] { 0x01 } else { 0x00 };
            out_arr[index] |= newval << byte_offs;

            index = if byte_offs >= 7 { index + 1 } else { index };
            byte_offs = if byte_offs >= 7 { 0 } else { byte_offs + 1 };
        }
    }

    out_arr
}

///
/// Switch a 2D array's rows and columns
///
pub fn transpose(arr: Matrix) -> [[u8; 34]; 9] {
    let mut out = [[0; 34]; 9];

    #[allow(clippy::needless_range_loop)]
    for i in 0..34 {
        for j in 0..9 {
            out[j][i] = arr[i][j];
        }
    }

    out
}

///
/// Overlay a smaller matrix on a larger matrix with a given position
///
pub fn emplace<W>(orig: Matrix, widget: &mut W, x: usize, y: usize) -> Matrix
where
    W: Widget + ?Sized,
{
    // debug_assert!(x as usize + widget.width < 9 && y as usize + widget.height < 34);
    let mut out: [[u8; 9]; 34] = orig;

    for i in 0..widget.get_shape().y {
        for j in 0..widget.get_shape().x {
            out[i + y][j + x] = widget.get_matrix()[j + (widget.get_shape().x * i)];
        }
    }

    out
}
