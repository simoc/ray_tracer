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
}

pub fn create_canvas(width: usize, height: usize) -> Canvas
{
    let mut pixels = Vec::with_capacity(height);
    for _ in 0..height
    {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width
        {
            row.push(create_color(0.0, 0.0, 0.0));
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
                assert!(equal(c1.pixels[y][x], create_color(0.0, 0.0, 0.0)));
            }
        }
    }
}
