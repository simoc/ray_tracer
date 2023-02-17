use crate::tuple::*;

pub struct Canvas
{
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Tuple>>,
}

impl Canvas
{
    pub fn write_pixel(mut self, x : usize, y: usize, c: Tuple)
    {
        if x >= self.width || y >= self.height
        {
            return;
        }
        self.pixels[y][x] = c;
    }

    pub fn pixel_at(self, x : usize, y: usize) -> Tuple
    {
        return self.pixels[y][x];
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
        let c2 = create_canvas(10, 20);
        let color_red = create_color(1.0, 0.0, 0.0);
        c2.write_pixel(2, 3, color_red);
        assert!(equal(c2.pixel_at(2, 3), color_red));
    }
}
