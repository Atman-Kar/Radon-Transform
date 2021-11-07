use pyo3::prelude::*;  
use std::f64::consts::PI;

#[pyclass]
pub struct Tissue {
    // define a tissue image as below
    image_size : usize,
    attenuation : Vec<Vec<f64>>,
}

#[pymethods]
impl Tissue {

    #[new]
    pub fn new(i_size : usize) -> Tissue {
        // constructor - create a new tissue object

        Tissue {
            image_size : i_size,
            attenuation : vec![vec![0.0 ;i_size]; i_size],
        }
    }

    pub fn get_image_size(&mut self) -> usize {
        // return the size of the square image

        self.image_size
    }

    pub fn print_attenuation_matrix(&mut self) {
        // print out the matrix

        for i in 0..self.image_size {
            for j in 0..self.image_size {
                print!("{}  ", self.attenuation[i][j]);
            }
            println!("");
        }
    }

    pub fn insert_attenuation_value(&mut self, attn : f64, coords : (usize, usize)) {
        // insert attenuation value into the matrix     
        // left-top corner of the image is the origin

        let (x, y) : (usize, usize) = coords;
        self.attenuation[x][y] = attn;
    }

    pub fn radon_transform(&mut self) -> Vec<Vec<f64>> {
        /*
            Function to perform the radon transform on the attenuation matrix
            Read more at : https://digitalcommons.colby.edu/cgi/viewcontent.cgi?article=1649&context=honorstheses
        */

        let diag : usize = (((self.image_size - 1) as f64 * 2.0_f64.sqrt()).floor() as usize) + 1;
        let mut r : Vec<Vec<f64>> = vec![vec![0.0 ; 180]; 2*diag + 1];

        for x in 0..self.image_size {
            for y in 0..self.image_size {
                for th_deg in 0..180 {
                    // convert the angle to radians
                    let th_rad : f64 = (th_deg as f64) * PI / 180.0;

                    // calculate perpendicular distance from line given angle theta 
                    let t : i64 = ((x as f64)*th_rad.cos() + (y as f64)*th_rad.sin()).floor() as i64;

                    // adding values along line
                    r[(t + (diag as i64)) as usize][th_deg] += self.attenuation[x][y]; 
                }
            }
        }

        r // return the radon transform matrix
    }

    pub fn backprojection(&mut self, radon_matrix : Vec<Vec<f64>>) -> Vec<Vec<f64>> {

        /* 
            recronstructing the image from several 1D xray slices. 
            See : https://homepages.inf.ed.ac.uk/rbf/CVonline/LOCAL_COPIES/AV0405/HAYDEN/Slice_Reconstruction.html
        */

        let diag : usize = (((self.image_size - 1) as f64 * 2.0_f64.sqrt()).floor() as usize) + 1;
        let mut out : Vec<Vec<f64>> = vec![vec![0.0; self.image_size]; self.image_size];
        for x in 0..self.image_size {
            for y in 0..self.image_size {

                let mut temp : f64 = 0.0;
                for th_deg in 0..180 {

                    // convert to radians
                    let th_rad : f64 = (th_deg as f64) * PI / 180.0;

                    // find the distance 
                    let t : i64 = ((x as f64)*th_rad.cos() + (y as f64)*th_rad.sin()).floor() as i64;

                    // integration / discrete summation
                    temp += radon_matrix[(t + (diag as i64)) as usize][th_deg];
                }

                out[x][y] = temp;
            }
        }

        out // return the backprojected matrix (without filtering)
    }
}

#[pymodule] 
fn radon_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tissue>()?;
    Ok(())
}