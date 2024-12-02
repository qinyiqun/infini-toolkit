use crate::DeviceType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Device {
    pub ty: DeviceType,
    pub id: u32,
}

impl Device {
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtDeviceSynchronize(self.ty, self.id))
    }
}
