use std::env;
use std::path::Path;

fn main() {
    if let [_, image_path, direction, degree] = &env::args().collect::<Vec<String>>()[..] {
        let path = Path::new(&image_path);
        let img = image::open(path).unwrap();
        let mut true_degree: i32 = degree.parse::<i32>().unwrap();
        if *direction == "LEFT".to_string() {
            true_degree = 360 - true_degree;
        }

        if ![0, 90, 180, 270, 360].contains(&true_degree) {
            println!("Non cardinal degree detected! Please use one of the following degree rotations as an argument: 90, 180, 270");
            return;
        } else {
            let rotated = match true_degree {
                90 => img.rotate90(),
                180 => img.rotate180(),
                270 => img.rotate270(),
                _ => img,
            };
            rotated.save(path).unwrap();
        }
    } else {
        println!("Error: Arguments should be passed as follows <IMAGE_PATH> <DIRECTION> <DEGREE>\ne.g imgtool path/to/your/image RIGHT 90");
    }
}
