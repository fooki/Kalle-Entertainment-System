pub trait Figure {
    fn deltas(&self) -> [(usize,usize);4];
    fn rotate(&self) -> Box<dyn Figure>;
}

pub struct NorthO();
pub struct EastO();
pub struct SouthO();
pub struct WestO();

impl Figure for NorthO {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (0, 1), (1, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastO())
    }
}

impl Figure for EastO {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (0, 1), (1, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthO())
    }
}

impl Figure for SouthO {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (0, 1), (1, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestO())
    }
}

impl Figure for WestO {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (0, 1), (1, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthO())
    }
}

pub struct NorthI();
pub struct EastI();
pub struct SouthI();
pub struct WestI();

impl Figure for NorthI {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (0, 2), (0, 3)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastI())
    }
}

impl Figure for EastI {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (2, 0), (3, 0)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthI())
    }
}

impl Figure for SouthI {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (0, 2), (0, 3)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestI())
    }
}

impl Figure for WestI {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (2, 0), (3, 0)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthI())
    }
}

pub struct NorthT();
pub struct EastT();
pub struct SouthT();
pub struct WestT();

impl Figure for NorthT {
    fn deltas(&self) -> [(usize,usize);4] {
        [(1, 0), (0, 1), (1, 1), (2, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastT())
    }
}

impl Figure for EastT {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (1, 1), (0, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthT())
    }
}

impl Figure for SouthT {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (2, 0), (1, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestT())
    }
}

impl Figure for WestT {
    fn deltas(&self) -> [(usize,usize);4] {
        [(1, 0), (1, 1), (1, 2), (0, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthT())
    }
}

pub struct NorthL();
pub struct EastL();
pub struct SouthL();
pub struct WestL();

impl Figure for NorthL {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (0, 2), (1, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastL())
    }
}

impl Figure for EastL {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (2, 0), (0, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthL())
    }
}

impl Figure for SouthL {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (1, 1), (1, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestL())
    }
}

impl Figure for WestL {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 1), (1, 1), (2, 1), (2, 0)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthL())
    }
}

pub struct NorthJ();
pub struct EastJ();
pub struct SouthJ();
pub struct WestJ();

impl Figure for NorthJ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(1, 0), (1, 1), (1, 2), (0, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastJ())
    }
}

impl Figure for EastJ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (1, 1), (2, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthJ())
    }
}

impl Figure for SouthJ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (0, 1), (0, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestJ())
    }
}

impl Figure for WestJ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (2, 0), (2, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthJ())
    }
}

pub struct NorthS();
pub struct EastS();
pub struct SouthS();
pub struct WestS();

impl Figure for NorthS {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 1), (1, 1), (1, 0), (2, 0)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastS())
    }
}

impl Figure for EastS {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (1, 1), (1, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthS())
    }
}

impl Figure for SouthS {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 1), (1, 1), (1, 0), (2, 0)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestS())
    }
}

impl Figure for WestS {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (0, 1), (1, 1), (1, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthS())
    }
}

pub struct NorthZ();
pub struct EastZ();
pub struct SouthZ();
pub struct WestZ();

impl Figure for NorthZ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (1, 1), (2, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(EastZ())
    }
}

impl Figure for EastZ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(1, 0), (1, 1), (0, 1), (0, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(SouthZ())
    }
}

impl Figure for SouthZ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(0, 0), (1, 0), (1, 1), (2, 1)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(WestZ())
    }
}

impl Figure for WestZ {
    fn deltas(&self) -> [(usize,usize);4] {
        [(1, 0), (1, 1), (0, 1), (0, 2)]
    }

    fn rotate(&self) -> Box<dyn Figure> {
        Box::new(NorthZ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_deltas() {
        NorthO().deltas();
        EastO().deltas();
        SouthO().deltas();
        WestO().deltas();
    }

    #[test]
    fn test_i_deltas() {
        NorthI().deltas();
        EastI().deltas();
        SouthI().deltas();
        WestI().deltas();
    }

    #[test]
    fn test_t_deltas() {
        NorthT().deltas();
        EastT().deltas();
        SouthT().deltas();
        WestT().deltas();
    }

    #[test]
    fn test_l_deltas() {
        NorthL().deltas();
        EastL().deltas();
        SouthL().deltas();
        WestL().deltas();
    }

    #[test]
    fn test_j_deltas() {
        NorthJ().deltas();
        EastJ().deltas();
        SouthJ().deltas();
        WestJ().deltas();
    }

    #[test]
    fn test_s_deltas() {
        NorthS().deltas();
        EastS().deltas();
        SouthS().deltas();
        WestS().deltas();
    }

    #[test]
    fn test_z_deltas() {
        NorthZ().deltas();
        EastZ().deltas();
        SouthZ().deltas();
        WestZ().deltas();
    }
}
