use std::collections::HashMap;
use crate::tuple::*;
use crate::shape::*;

#[derive(Clone, Debug)]
pub struct ObjFile
{
    pub vertices: Vec<Tuple>,
    pub normals: Vec<Tuple>,
    pub default_group: Shape,
    pub groups: HashMap<String, Shape>,
}

pub fn parse_obj_file(lines: Vec<&str>) -> ObjFile
{
    let mut id = 1;
    let mut v = Vec::new();
    let mut vn = Vec::new();
    // Add unused entry at index 0, so we can used 1-based indexing
    v.push(create_point(0.0, 0.0, 0.0));
    vn.push(create_point(0.0, 0.0, 0.0));
    let mut default_group = Shape::new_group(id);
    let mut groups: HashMap<String, Shape> = HashMap::new();
    let mut current_groups: Vec<String> = Vec::new();
    for line in lines
    {
        let words: Vec<String> = line.split_ascii_whitespace().map(String::from).collect();
        if words.len() >= 2
        {
            if words[0] == "v" && words.len() == 4
            {
                let p = create_point(
                    words[1].parse::<f64>().unwrap(),
                    words[2].parse::<f64>().unwrap(),
                    words[3].parse::<f64>().unwrap());
                v.push(p);
            }
            else if words[0] == "vn" && words.len() == 4
            {
                let p = create_point(
                    words[1].parse::<f64>().unwrap(),
                    words[2].parse::<f64>().unwrap(),
                    words[3].parse::<f64>().unwrap());
                vn.push(p);
            }
            else if words[0] == "f" && words.len() >= 4
            {
                // Add single triangle if three vertices, or polygon
                // with fan triangulation if more than three vertices
                let last_index = words.len() - 1;
                for index in 2..last_index
                {
                    let j1 = words[1].parse::<usize>().unwrap();
                    let j2 = words[index].parse::<usize>().unwrap();
                    let j3 = words[index + 1].parse::<usize>().unwrap();
                    id = id + 1;
                    let mut t = Shape::new_triangle(id, v[j1], v[j2], v[j3]);
                    let groups2 = current_groups.clone();
                    if !current_groups.is_empty()
                    {
                        for name in groups2
                        {
                            match groups.get(&name)
                            {
                                Some(g) =>
                                {
                                    let mut g2 = g.clone();
                                    g2.add_child(&mut t);
                                    groups.insert(name, g2);
                                },
                                _ =>
                                {
                                    id = id + 1;
                                    let mut group = Shape::new_group(id);
                                    group.add_child(&mut t);
                                    groups.insert(name, group);
                                },
                            }
                        }
                    }
                    else
                    {
                        default_group.add_child(&mut t);
                    }
                }
            }
            else if words[0] == "g"
            {
                current_groups.clear();
                for index in 1..words.len()
                {
                    current_groups.push(words[index].clone());
                }
            }
        }
    }
    ObjFile{vertices: v, normals: vn,
        default_group: default_group, groups: groups}
}

impl ObjFile
{
    pub fn obj_to_group(&self) -> Shape
    {
        let mut group = Shape::new_group(1);
        let mut g2 = self.default_group.clone();
        if !self.default_group.get_children().is_empty()
        {
            group.add_child(&mut g2);
        }
        for g in self.groups.values()
        {
            let mut g3 = g.clone();
            group.add_child(&mut g3);
        }
        group
    }
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

    #[test]
    fn test_objfile_feature12()
    {
        // p.217 Scenario: Triangles in groups
        let lines12 = vec!["v -1 1 0",
            "v -1 0 0",
            "v 1 0 0",
            "v 1 1 0",
            "g FirstGroup",
            "f 1 2 3",
            "g SecondGroup",
            "f 1 3 4"];
        let obj12 = parse_obj_file(lines12);
        assert_eq!(obj12.vertices.len(), 4 + 1);
        assert!(obj12.default_group.get_children().is_empty());
        assert!(obj12.groups.contains_key("FirstGroup"));
        let first_group = obj12.groups.get("FirstGroup");
        match first_group
        {
            Some (g112) =>
            {
                let children112 = g112.get_children();
                assert_eq!(children112.len(), 1);
                assert!(children112[0].is_triangle());
                let t112 = children112[0].get_triangle();
                assert_eq!(t112.p1, obj12.vertices[1]);
                assert_eq!(t112.p2, obj12.vertices[2]);
                assert_eq!(t112.p3, obj12.vertices[3]);
            },
            _ =>
            {
                panic!("FirstGroup not found");
            },
        }
        assert!(obj12.groups.contains_key("SecondGroup"));
        let second_group = obj12.groups.get("SecondGroup");
        match second_group
        {
            Some (g212) =>
            {
                let children212 = g212.get_children();
                assert_eq!(children212.len(), 1);
                assert!(children212[0].is_triangle());
                let t212 = children212[0].get_triangle();
                assert_eq!(t212.p1, obj12.vertices[1]);
                assert_eq!(t212.p2, obj12.vertices[3]);
                assert_eq!(t212.p3, obj12.vertices[4]);
            },
            _ =>
            {
                panic!("SecondGroup not found");
            },
        }
    }

    #[test]
    fn test_objfile_feature13()
    {
        // p.218 Scenario: Converting an OBJ file to a group
        let lines13 = vec!["v -1 1 0",
            "v -1 0 0",
            "v 1 0 0",
            "v 1 1 0",
            "g FirstGroup",
            "f 1 2 3",
            "g SecondGroup",
            "f 1 3 4"];
        let obj13 = parse_obj_file(lines13);
        let group13 = obj13.obj_to_group();
        let children13 = group13.get_children();
        assert_eq!(children13.len(), 2);
        let children113 = children13[0].get_children();
        assert_eq!(children113.len(), 1);
        assert!(children113[0].is_triangle());
        let children213 = children13[1].get_children();
        assert!(children213[0].is_triangle());
    }

    #[test]
    fn test_objfile_feature19()
    {
        // p.218 Scenario: Vertex normal records
        let lines19 = vec!["v -1 1 0",
            "vn 0 0 1",
            "vn 0.707 0 -0.707",
            "vn 1 2 3"];
        let obj19 = parse_obj_file(lines19);
        assert_eq!(obj19.normals.len(), 3 + 1);
        assert_eq!(obj19.normals[1], create_point(0.0, 0.0, 1.0));
        assert_eq!(obj19.normals[2], create_point(0.707, 0.0, -0.707));
        assert_eq!(obj19.normals[3], create_point(1.0, 2.0, 3.0));
    }
}
