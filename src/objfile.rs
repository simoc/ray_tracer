use crate::tuple::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct ObjFile
{
    pub vertices: Vec<Tuple>,
    pub default_group: Shape,
}

pub fn parse_obj_file(lines: Vec<&str>) -> ObjFile
{
    let mut id = 1;
    let mut v = Vec::new();
    // Add unused entry at index 0, so we can used 1-based indexing
    v.push(create_point(0.0, 0.0, 0.0));
    let mut default_group = Shape::new_group(id);
    for line in lines
    {
        let words: Vec<String> = line.split_ascii_whitespace().map(String::from).collect();
        if words.len() >= 4
        {
            if words[0] == "v"
            {
                let p = create_point(
                    words[1].parse::<f64>().unwrap(),
                    words[2].parse::<f64>().unwrap(),
                    words[3].parse::<f64>().unwrap());
                v.push(p);
            }
            else if words[0] == "f"
            {
                // Add single triangle if three vertices, or polygon
                // with fan triangulation if more than three vertices
                let last_index = words.len() - 1;
                for index in 2..last_index
                {
                    let j1 = words[1].parse::<usize>().unwrap();
                    let j2 = words[index].parse::<usize>().unwrap();
                    let j3 = words[index + 1].parse::<usize>().unwrap();
                    let id = id + 1;
                    let mut t = Shape::new_triangle(id, v[j1], v[j2], v[j3]);
                    default_group.add_child(&mut t);
                }
            }
        }
    }
    ObjFile{vertices: v, default_group: default_group}
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
        assert_eq!(obj8.vertices.len(), 0 + 1);
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
        assert_eq!(obj9.vertices.len(), 4 + 1);
        assert_eq!(obj9.vertices[1], create_point(-1.0, 1.0, 0.0));
        assert_eq!(obj9.vertices[2], create_point(-1.0, 0.5, 0.0));
        assert_eq!(obj9.vertices[3], create_point(1.0, 0.0, 0.0));
        assert_eq!(obj9.vertices[4], create_point(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_objfile_feature10()
    {
        // p.214 Scenario: Parsing triangle faces
        let lines10 = vec!["v -1 1 0",
            "v -1 0 0",
            "v 1 0 0",
            "v 1 1 0",
            "f 1 2 3",
            "f 1 3 4"];
        let obj10 = parse_obj_file(lines10);
        assert_eq!(obj10.vertices.len(), 4 + 1);
        let children10 = obj10.default_group.get_children();
        assert_eq!(children10.len(), 2);
        assert!(children10[0].is_triangle());
        assert!(children10[1].is_triangle());
        let t110 = children10[0].get_triangle();
        let t210 = children10[1].get_triangle();
        assert_eq!(t110.p1, obj10.vertices[1]);
        assert_eq!(t110.p2, obj10.vertices[2]);
        assert_eq!(t110.p3, obj10.vertices[3]);

        assert_eq!(t210.p1, obj10.vertices[1]);
        assert_eq!(t210.p2, obj10.vertices[3]);
        assert_eq!(t210.p3, obj10.vertices[4]);
    }

    #[test]
    fn test_objfile_feature11()
    {
        // p.214 Scenario: Triangulating polygons
        let lines11 = vec!["v -1 1 0",
            "v -1 1 0",
            "v -1 0 0",
            "v 1 0 0",
            "v 1 1 0",
            "v 0 2 0",
            "f 1 2 3 4 5"];
        let obj11 = parse_obj_file(lines11);
        assert_eq!(obj11.vertices.len(), 6 + 1);
        let children11 = obj11.default_group.get_children();
        assert!(children11.len() >= 3);
        assert!(children11[0].is_triangle());
        assert!(children11[1].is_triangle());
        assert!(children11[2].is_triangle());
        let t111 = children11[0].get_triangle();
        let t211 = children11[1].get_triangle();
        let t311 = children11[2].get_triangle();
        assert_eq!(t111.p1, obj11.vertices[1]);
        assert_eq!(t111.p2, obj11.vertices[2]);
        assert_eq!(t111.p3, obj11.vertices[3]);

        assert_eq!(t211.p1, obj11.vertices[1]);
        assert_eq!(t211.p2, obj11.vertices[3]);
        assert_eq!(t211.p3, obj11.vertices[4]);

        assert_eq!(t311.p1, obj11.vertices[1]);
        assert_eq!(t311.p2, obj11.vertices[4]);
        assert_eq!(t311.p3, obj11.vertices[5]);
    }
}
