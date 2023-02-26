use crate::tuple::*;

pub struct Canvas
{
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Tuple>>,
}

impl Canvas
{
	pub fn new(width: usize, height: usize) -> Self
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
                let mut rgb = self.pixel_at(x, y).get_vec();
                rgb.resize(3, 0.0); // want only RGB components
                for p1 in rgb
                {
                    let p2 = (p1 * f64::from(max_value)).clamp(0.0, 255.0).round();
                    let p3 = format!("{}", p2);
                    if line.len() + 1 + p3.len() > 70
                    {
                        // Split long lines.
                        ppm.push_str(&line);
                        ppm.push_str("\n");
                        line = String::new();
                    }
                    else if line.len() > 0
                    {
                        // Need a space between pixel values.
                        line.push_str(" ");
                    }
                    line.push_str(&p3);
                }
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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_canvas_feature()
    {
        // p.19 Scenario: Creating a canvas
        let c1 = Canvas::new(10, 20);
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
        let mut c2 = Canvas::new(10, 20);
        let color_red = create_color(1.0, 0.0, 0.0);
        c2.write_pixel(2, 3, color_red);
        assert!(equal(c2.pixel_at(2, 3), color_red));

        // p.20 Scenario: Constructing the PPM header
        let c3 = Canvas::new(5, 3);
        let ppm3 = c3.canvas_to_ppm();
        let mut lines3 = ppm3.lines();
        assert_eq!(lines3.next(), Some("P3"));
        assert_eq!(lines3.next(), Some("5 3"));
        assert_eq!(lines3.next(), Some("255"));

        // p.21 Scenario: Constructing the pixel data
        let mut c4 = Canvas::new(5, 3);
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

        // p.22 Scenario: Splitting long lines in PPM files
        let mut c5 = Canvas::new(10, 2);
        for y in 0..c5.height
        {
            for x in 0..c5.width
            {
                c5.write_pixel(x, y, create_color(1.0, 0.8, 0.6));
            }
        }
        let ppm5 = c5.canvas_to_ppm();
        let mut lines5 = ppm5.lines();
        lines5.next();
        lines5.next();
        lines5.next();
        assert_eq!(lines5.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(lines5.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));
        assert_eq!(lines5.next(), Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(lines5.next(), Some("153 255 204 153 255 204 153 255 204 153 255 204 153"));

        // p.22 Scenario: PPM files are terminated by a newline character
        let c6 = Canvas::new(5, 3);
        let ppm6 = c6.canvas_to_ppm();
        assert!(ppm6.ends_with("\n"));
    }
}
