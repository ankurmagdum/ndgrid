//! Gmsh I/O
use crate::traits::{Builder, Grid};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub trait GmshExport: Grid {
    //! Grid export for Gmsh

    /// Generate the Gmsh string for a grid
    fn to_gmsh_string(&self) -> String;

    /// Export as Gmsh
    fn export_as_gmsh(&self, filename: &str) {
        let gmsh_s = self.to_gmsh_string();
        fs::write(filename, gmsh_s).expect("Unable to write file");
    }
}

pub trait GmshImport: Builder {
    //! Grid import for Gmsh

    /// Generate grid from a Gmsh v1
    fn import_from_v1(&mut self, reader: BufReader<File>);

    /// Generate grid from a Gmsh v2 and v4
    fn import_from_v2_v4(
        &mut self,
        reader: BufReader<File>,
        version: &str,
        binary_mode: &str,
        data_size: &str,
    );

    /// Generate grid from Gmsh
    fn import_from_gmsh(&mut self, filename: &str) {
        let f = File::open(filename).expect("Unable to open file");
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        reader.read_line(&mut line).expect("Unable to read header");

        if line.starts_with("$NOD") {
            self.import_from_v1(reader);
            return;
        }

        line.clear();
        reader.read_line(&mut line).expect("Unable to read header");

        let [version, binary_mode, data_size] = line.trim().split(" ").collect::<Vec<_>>()[..]
        else {
            panic!("Unrecognised format");
        };

        self.import_from_v2_v4(reader, version, binary_mode, data_size);
    }
}
