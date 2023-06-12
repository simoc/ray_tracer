use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct ObjFile
{
    pub vertices: Vec<Tuple>,
}

pub fn parse_obj_file(lines: Vec<&str>) -> ObjFile
{
    let mut v = Vec::new();
    for line in lines
    {
        let mut iter = line.split_ascii_whitespace();
        match iter.next()
        {
            Some("v") =>
            {
                match iter.next()
                {
                    Some(x) =>
                    {
                        match iter.next()
                        {
                            Some(y) =>
                            {
                                match iter.next()
                                {
                                    Some(z) =>
                                    {
                                        let p = create_point(
                                            x.parse::<f64>().unwrap(),
                                            y.parse::<f64>().unwrap(),
                                            z.parse::<f64>().unwrap());
                                        v.push(p);
                                    }
                                    None => (),
                                }
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            },
            Some(&_) => (),
            None => (),
        }
    }
    ObjFile{vertices: v}
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_objfile_feature8()
    {
        // p.213 Scenario: Ignoring unrecognized lines
        let lines8 = vec!["There was a young lady named Bright",
            "who traveled much fast than light.",
            "She set out one day",
            "in a relative way",
            "and came back the previous night."];
        let obj8 = parse_obj_file(lines8);
        assert!(obj8.vertices.is_empty());
    }

    #[test]
    fn test_objfile_feature9()
    {
        // p.214 Scenario: Vertex records
        let lines9 = vec!["v -1 1 0",
            "v -1.0000 0.5000 0.0000",
            "v 1 0 0",
            "v 1 1 0"];
        let obj9 = parse_obj_file(lines9);
        assert_eq!(obj9.vertices.len(), 4);
        assert_eq!(obj9.vertices[0], create_point(-1.0, 1.0, 0.0));
    }
}
