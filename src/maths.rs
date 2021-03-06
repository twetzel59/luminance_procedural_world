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

/// The identity matrix.
pub const IDENTITY: M44 = mat4! [
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.,
];

/// Typeclass for types that can be represented by a 4x4 matrix.
pub trait ToMatrix {
    /// Get the matrix representing the type's transform.
    fn to_matrix(&self) -> M44;
}

/// Stores a translation.
#[derive(Clone, Debug)]
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
    
    /// Shift the translation by this offset.
    pub fn slide(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
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

/// Stores a rotation. Only rotations about the X and Y axis
/// are preformed.
#[derive(Clone, Debug)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
}

impl Rotation {
    /// Create a new Rotation with these values.
    pub fn new(x: f32, y: f32) -> Rotation {
        Rotation {
            x,
            y,
        }
    }
    
    /// Adjust the rotation by this offset.
    pub fn spin(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

impl ToMatrix for Rotation {
    fn to_matrix(&self) -> M44 {
        let sin = self.x.sin();
        let cos = self.x.cos();
        let rx = mat4! [
            1.,     0.,     0.,     0.,
            0.,     cos,    -sin,   0.,
            0.,     sin,    cos,    0.,
            0.,     0.,     0.,     1.,
        ];
        
        let sin = self.y.sin();
        let cos = self.y.cos();
        let ry = mat4! [
            cos,    0.,     sin,    0.,
            0.,     1.,     0.,     0.,
            -sin,   0.,     cos,    0.,
            0.,     0.,     0.,     1.,
        ];
        
        matrix_mul(&rx, &ry)
    }
}

/// Stores a 3D projection.
#[derive(Clone, Debug)]
pub struct Projection {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Projection {
    /// Create a new Projection with these values.
    /// # Parameters
    /// * `fov`: Field of view **in radians**
    /// * `aspect`: Aspect ratio
    /// * `near` and `far`: Clipping planes
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Projection {
        Projection {
            fov,
            aspect,
            near,
            far
        }
    }
}

impl ToMatrix for Projection {
    fn to_matrix(&self) -> M44 {
        let fov_expr = 1. / (self.fov / 2.).tan();
        let aspect = self.aspect;
        let near = self.near;
        let far = self.far;
        let ndist = far - near;
        let fdist = far + near;
        
        mat4! [
            fov_expr / aspect,  0.,                 0.,                 0.,
            0.,                 fov_expr,           0.,                 0.,
            0.,                 0.,                 -fdist / ndist,     -(2. * far * near) / ndist,
            0.,                 0.,                 -1.,                0.,
        ]
    }
}

/// Multiplies two 4x4 matrices, returning the product.
pub fn matrix_mul(left: &M44, right: &M44) -> M44 {
    let mut result = mat4! [
        0., 0., 0., 0.,
        0., 0., 0., 0.,
        0., 0., 0., 0.,
        0., 0., 0., 0.,
    ];
    
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += left[k][j] * right[i][k];
            }
        }
    }
    
    result
}

/// A 3D plane defined as (A, B, C, D).
#[derive(Clone, Debug)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    /// Create a plane from these coefficients.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane {
            a,
            b,
            c,
            d
        }
    }
    
    /// Normalize the plane.
    pub fn normalize(&mut self) {
        let l = (sq(self.a) + sq(self.b) + sq(self.c)).sqrt();
        
        self.a /= l;
        self.b /= l;
        self.c /= l;
        self.d /= l;
    }
}

type FlatM44 = [f32; 16];

/// Represents the frustum of the camera.
#[derive(Clone, Debug)]
pub struct Frustum {
    planes: [Plane; 6],
}

impl Frustum {
    /// Create the frustum from the projection and view matrices.
    pub fn new(proj: &M44, view: &M44) -> Frustum {
        // http://www.crownandcutlass.com/features/technicaldetails/frustum.html
        // http://www.lighthouse3d.com/tutorials/view-frustum-culling/clip-space-approach-extracting-the-planes/
        
        let mat: FlatM44 = unsafe {
            ::std::mem::transmute(matrix_mul(proj, view))
        };
        
        let mut right  = Plane::new(mat[3]  - mat[0],
                                    mat[7]  - mat[4],
                                    mat[11] - mat[8],
                                    mat[15] - mat[12]);

        let mut left   = Plane::new(mat[3]  + mat[0],
                                    mat[7]  + mat[4],
                                    mat[11] + mat[8],
                                    mat[15] + mat[12]);

        let mut bottom = Plane::new(mat[3]  + mat[1],
                                    mat[7]  + mat[5],
                                    mat[11] + mat[9],
                                    mat[15] + mat[13]);
                                    
        let mut top =    Plane::new(mat[3]  - mat[1],
                                    mat[7]  - mat[5],
                                    mat[11] - mat[9],
                                    mat[15] - mat[13]);
                                    
        let mut far =    Plane::new(mat[3]  - mat[2],
                                    mat[7]  - mat[6],
                                    mat[11] - mat[10],
                                    mat[15] - mat[14]);
        
        let mut near =   Plane::new(mat[3]  + mat[2],
                                    mat[7]  + mat[6],
                                    mat[11] + mat[10],
                                    mat[15] + mat[14]);
        
        right.normalize();
        left.normalize();
        bottom.normalize();
        top.normalize();
        far.normalize();
        near.normalize();
        
        Frustum {
            planes: [
                right,
                left,
                bottom,
                top,
                far,
                near,
            ],
        }        
    }
    
    /// Return a reference to the frustum planes in the order:
    /// * Right
    /// * Left
    /// * Bottom
    /// * Top
    /// * Far
    /// * Near
    pub fn planes(&self) -> &[Plane; 6] {
        &self.planes
    }
}

// Utility
fn sq(x: f32) -> f32 {
    x * x
}
