/* All credits to a1k0n for providing the pseudocode of donut.c 
https://www.a1k0n.net/2011/07/20/donut-math.html
*/

use std::f32::consts::PI;
/* Screen width and height */

pub const SCREEN_WIDTH:f32 = 100.0;
pub const SCREEN_HEIGHT:f32 = 100.0;

pub struct Donut {
    r1 : f32, // Radius of the circle
    r2 : f32, // Distance from the axis of rotation y to circle. Changes the shape of the Donut
    k1 : f32, // z`
    k2 : f32, // Distance of the donut from the viewer
    theta_spacing : f32, // Inner circle angle step
    phi_spacing : f32, // rotation around the axis step
}

impl Donut {
    pub fn new(r1: f32, r2: f32, theta_spacing : f32, phi_spacing : f32) -> Self {
        let k2 = 25.0;
        let k1 = (SCREEN_WIDTH)*k2*2.0/(8.0*(r1+r2));
        Donut {r1, r2, k1, k2, theta_spacing, phi_spacing}
    }

    pub fn get_index(&self, x : u32, y : u32) -> usize { // Assume the Donut is always in a square
        (x + SCREEN_WIDTH as u32 * y) as usize
    }    

    pub fn draw(&mut self, a: f32, b: f32) -> Vec<u8> { // A : Angle of rotation around X-axis, B : Angle of rotation around Y-axis
        let cos_a = a.cos();
        let sin_a = a.sin();
        let cos_b = b.cos();
        let sin_b = b.sin();
        let mut output = (0..SCREEN_HEIGHT as u32 * SCREEN_WIDTH as u32).map(|_| 0).collect::<Vec<u8>>();
        let mut z_buffer = (0..SCREEN_HEIGHT as u32 * SCREEN_WIDTH as u32).map(|_| 0f32).collect::<Vec<f32>>();
        let mut theta : f32 = 0.0;
        while theta < 2.0*PI {
            let mut phi :f32 = 0.0;
            while phi < 2.0*PI {
                let circle_x = self.r2 + self.r1*theta.cos();
                let circle_y = self.r1*theta.sin();
                let x = circle_x * (cos_b * phi.cos() + sin_a*sin_b*phi.sin()) - circle_y*cos_a*sin_b;
                let y = circle_x * (sin_b * phi.cos() - sin_a*cos_b*phi.sin()) + circle_y*cos_a*cos_b;
                let z = self.k2 + cos_a*circle_x*phi.sin() + circle_y*sin_a;
                let ooz = 1.0/z; // One over z
                // println!("{}", z);
                let xp = (SCREEN_WIDTH/2.0 + self.k1*ooz*x) as u32;
                let yp = (SCREEN_HEIGHT/2.0 - self.k1*ooz*y) as u32;
                // println!("X: {}, Y: {}", xp, yp);
                let l = phi.cos()*theta.cos()*sin_b - cos_a*theta.cos()*phi.sin() - sin_a*theta.sin() + cos_b*(cos_a*theta.sin() - theta.cos()*sin_a*phi.sin());
                if l > 0.0 {
                    let index = self.get_index(xp, yp);
                    if ooz > z_buffer[index] {
                        z_buffer[index] = ooz;
                        let luminance_index = l*180.0; // l = [0, sqrt 2]. l*180 ~[0, 256]
                        output[index] = luminance_index as u8;
                    }
                }
                phi += self.phi_spacing;
            }
            theta += self.theta_spacing;
        }
        output
    }
}

