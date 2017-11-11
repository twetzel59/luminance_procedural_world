//! General 3D game math.

use luminance::linear::M44;

/// Creates a luminance 4x4 matrix.
/// When specifying a matrix normally, the rows and columns
/// are transposed from how they are on paper. This is confusing.
/// Also, nested arrays are used to represent the columns.
/// This macro simplifies declaring a matrix.
///
/// Simply plug in 16 values as one would on paper.
/// # Example
/// ```
/// let identity = mat4 [
///     1., 0., 0., 0.,
///     0., 1., 0., 0.,
///     0., 0., 1., 0.,
///     0., 0., 0., 1.,
/// ];
/// ```
#[macro_export]
macro_rules! mat4 {
    ($m00:expr, $m10:expr, $m20:expr, $m30:expr,
     $m01:expr, $m11:expr, $m21:expr, $m31:expr,
     $m02:expr, $m12:expr, $m22:expr, $m32:expr,
     $m03:expr, $m13:expr, $m23:expr, $m33:expr)
        => (mat4![$m00, $m10, $m20, $m30,
                  $m01, $m11, $m21, $m31,
                  $m02, $m12, $m22, $m32,
                  $m03, $m13, $m23, $m33, ]);
     
    ($m00:expr, $m10:expr, $m20:expr, $m30:expr,
     $m01:expr, $m11:expr, $m21:expr, $m31:expr,
     $m02:expr, $m12:expr, $m22:expr, $m32:expr,
     $m03:expr, $m13:expr, $m23:expr, $m33:expr ,)
        => ([[$m00, $m01, $m02, $m03],
             [$m10, $m11, $m12, $m13],
             [$m20, $m21, $m22, $m23],
             [$m30, $m31, $m32, $m33]]);
}

/// Typeclass for types that can be represented by a 4x4 matrix.
pub trait ToMatrix {
    fn to_matrix(&self) -> M44;
}

/// Stores a translation
pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Translation {
    /// Create a new Translation with these values.
    pub fn new(x: f32, y: f32, z: f32) -> Translation {
        Translation {
            x,
            y,
            z,
        }
    }
}

impl ToMatrix for Translation {
    fn to_matrix(&self) -> M44 {
        mat4! [
            1., 0., 0., self.x,
            0., 1., 0., self.y,
            0., 0., 1., self.z,
            0., 0., 0., 1.,
        ]
    }
}
