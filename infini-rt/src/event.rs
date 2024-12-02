use std::ptr::null_mut;

use crate::{
    bindings::{infinirtEventQuery, infinirtEvent_t, infinirtStatus_t as Status},
    AsRaw, Device, Stream,
};

#[repr(transparent)]
pub struct Event(infinirtEvent_t);

impl Device {
    pub fn event(&self) -> Event {
        let mut event = null_mut();
        infini!(infinirtEventCreate(&mut event, self.ty, self.id));
        Event(event)
    }
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Drop for Event {
    fn drop(&mut self) {
        infini!(infinirtEventDestroy(self.0))
    }
}

impl AsRaw for Event {
    type Raw = infinirtEvent_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl Event {
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtEventSynchronize(self.0))
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        match unsafe { infinirtEventQuery(self.0) } {
            Status::INFINIRT_STATUS_SUCCESS => true,
            Status::INFINIRT_STATUS_NOT_READY => false,
            _ => unreachable!(),
        }
    }
}

impl Stream {
    #[inline]
    pub fn record(&self, event: &Event) {
        infini!(infinirtEventRecord(event.0, self.as_raw()))
    }

    #[inline]
    pub fn wait(&self, event: &Event) {
        infini!(infinirtStreamWaitEvent(event.0, self.as_raw()))
    }
}
