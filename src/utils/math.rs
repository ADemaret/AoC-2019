///
/// On reçoit les paramètres a et b de l'équation y = ax² + bx + c
/// On renvoie x_min et x_max
///
pub fn equation_du_second_degre(a: f64, b: f64, c: f64) -> (f64, f64) {
    let delta = (b * b) - 4.0 * a * c; // b^2-4.a.c
    let rac_delta = delta.sqrt();
    ((-b + rac_delta) / (2.0 * a), (-b - rac_delta) / (2.0 * a))
}

///
/// lcm = least common multiple = ppcm en français
///
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
///
/// gcd = greatest common divisor = pgcd en français
///
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

///
/// Shoelace algorithm
/// to calculate the survace of a polygon
///
pub fn shoelace(coords: Vec<(f64, f64)>, thickness: bool) -> Result<f64, i8> {
    if coords.len() < 4 {
        println!(
            "Pas assez de coordonnées ({}) pour calculer une surface",
            coords.len()
        );
        return Err(0);
    }
    if coords[0] != coords[coords.len() - 1] {
        println!("Il faut répéter le premier point en dernière coordonnée");
        return Err(0);
    }

    let mut longueur = 0.0;
    let mut predec = coords[0];
    let mut total1: f64 = 0.0;
    let mut total2: f64 = 0.0;
    for (i, point) in coords.iter().enumerate() {
        if i == 0 {
            predec = *point;
        } else {
            total1 += point.1 * predec.0;
            total2 += point.0 * predec.1;
            if thickness {
                longueur += (point.0 - predec.0).abs() + (point.1 - predec.1).abs();
            }
            predec = *point;
        }
    }
    let mut surface = (total1 - total2).abs() * 0.5;
    if thickness {
        surface += (longueur / 2.0) + 1.0;
    }

    Ok(surface)
}

///
/// get angle between two points of a grid (line, column), in degrees (0-360)
///
pub fn get_angle(station: (usize, usize), aster: (usize, usize)) -> f32 {
    let f_st = (station.0 as f32, station.1 as f32);
    let f_as = (aster.0 as f32, aster.1 as f32);
    let a: f32 = (-f_as.0 + f_st.0) / (f_as.1 - f_st.1);
    let mut atan = a.atan() * 180.0 / (std::f32::consts::PI);
    if aster.0 < station.0 && aster.1 < station.1 {
        // println!("quadrant 2");
        atan += 180.0;
    } else if aster.0 >= station.0 && aster.1 < station.1 {
        // println!("quadrant 3");
        atan += 180.0;
    } else if aster.0 > station.0 && aster.1 >= station.1 {
        // println!("quadrant 4");
        atan += 360.0;
    } else {
        // println!("quadrant 1");
    }
    // println!("{:?} and {:?} => {} {}", station, aster, a, atan);
    atan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        assert_eq!(get_angle((1,1), (1,2)),0.0);
        assert_eq!(get_angle((1,1), (0,2)),45.0);
        assert_eq!(get_angle((1,1), (0,1)),90.0);
        assert_eq!(get_angle((1,1), (0,0)),135.0);
        assert_eq!(get_angle((1,1), (1,0)),180.0);
        assert_eq!(get_angle((1,1), (2,0)),225.0);
        assert_eq!(get_angle((1,1), (2,1)),270.0);
        assert_eq!(get_angle((1,1), (2,2)),315.0);
    }
}