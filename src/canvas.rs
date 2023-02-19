use crate::tuple::*;

pub struct Canvas
{
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Tuple>>,
}

impl Canvas
{
    pub fn write_pixel(&mut self, x : usize, y: usize, c: Tuple)
    {
        if x >= self.width || y >= self.height
        {
            return;
        }
        self.pixels[y][x] = c;
    }

    pub fn pixel_at(&self, x : usize, y: usize) -> Tuple
    {
        return self.pixels[y][x];
    }

    pub fn canvas_to_ppm(&self) -> String
    {
        let max_value = 255;
        let mut ppm = format!("P3\n{} {}\n{}\n", self.width, self.height, max_value);
        for y in 0..self.height
        {
            let mut line = String::new();
            for x in 0..self.width
            {
                let v = self.pixel_at(x, y).get_vec();
                let r = (v[0] * f64::from(max_value)).clamp(0.0, 255.0).round();
                let g = (v[1] * f64::from(max_value)).clamp(0.0, 255.0).round();
                let b = (v[2] * f64::from(max_value)).clamp(0.0, 255.0).round();
                let pixel = format!("{} {} {}", r, g, b);
                if line.len() + pixel.len() > 70
                {
                    // Split long lines.
                    ppm.push_str(&line);
                    ppm.push_str("\n");
                    line = String::new();
                }
                else if x > 0
                {
                    // Need a space between pixel values.
                    line.push_str(" ");
                }
                line.push_str(&pixel);
            }
            if line.len() > 0
            {
                ppm.push_str(&line);
                ppm.push_str("\n");
            }
        }
        ppm
    }
}

pub fn create_canvas(width: usize, height: usize) -> Canvas
{
    let color_black = create_color(0.0, 0.0, 0.0);
    let mut pixels = Vec::with_capacity(height);
    for _ in 0..height
    {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width
        {
            row.push(color_black);
        }
        pixels.push(row);
    }
    Canvas{width: width, height: height, pixels: pixels}
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_canvas_feature()
    {
        // p.19 Scenario: Creating a canvas
        let c1 = create_canvas(10, 20);
        assert_eq!(c1.width, 10);
        assert_eq!(c1.height, 20);
        for y in 0..c1.height
        {
            for x in 0..c1.width
            {
                assert!(equal(c1.pixel_at(x, y), create_color(0.0, 0.0, 0.0)));
            }
        }

        // p.19 Scenario: Writing pixels to a canvas
        let mut c2 = create_canvas(10, 20);
        let color_red = create_color(1.0, 0.0, 0.0);
        c2.write_pixel(2, 3, color_red);
        assert!(equal(c2.pixel_at(2, 3), color_red));

        // p.20 Scenario: Constructing the PPM header
        let c3 = create_canvas(5, 3);
        let ppm3 = c3.canvas_to_ppm();
        let mut lines3 = ppm3.lines();
        assert_eq!(lines3.next(), Some("P3"));
        assert_eq!(lines3.next(), Some("5 3"));
        assert_eq!(lines3.next(), Some("255"));

        // p.21 Scenario: Constructing the pixel data
        let mut c4 = create_canvas(5, 3);
        c4.write_pixel(0, 0, create_color(1.5, 0.0, 0.0));
        c4.write_pixel(2, 1, create_color(0.0, 0.5, 0.0));
        c4.write_pixel(4, 2, create_color(-0.5, 0.0, 1.0));
        let ppm4 = c4.canvas_to_ppm();
        let mut lines4 = ppm4.lines();
        lines4.next();
        lines4.next();
        lines4.next();
        assert_eq!(lines4.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(lines4.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(lines4.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }
}
