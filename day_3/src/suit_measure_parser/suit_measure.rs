pub struct SuitMeasure {
    id: String,
    left_edge: i32,
    top_edge: i32,
    width: i32,
    height: i32,
}

impl std::fmt::Display for SuitMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(id: {}, left_edge: {}, top_edge: {}, width: {}, height: {})",
            self.id, self.left_edge, self.top_edge, self.width, self.height
        )
    }
}

fn parse_int(arg: &str) -> i32 { arg.trim().parse::< i32 > ().unwrap() }

impl SuitMeasure {
    pub fn from_raw_measure(raw_measure: &str) -> SuitMeasure {
        let split_at = raw_measure.split("@").collect::< Vec < & str > > ();
        let split_semicolon = split_at[1].split(":").collect::< Vec <& str > > ();
        let edges_split = split_semicolon[0].split(",").collect::< Vec < & str > > ();
        let size_split = split_semicolon[1].split("x").collect::< Vec <& str > > ();

        SuitMeasure {
            id: split_at[0].trim().to_string(),
            left_edge: parse_int(edges_split[0]),
            top_edge: parse_int(edges_split[1]),
            width: parse_int(size_split[0]),
            height: parse_int(size_split[1]),
        }
    }
}