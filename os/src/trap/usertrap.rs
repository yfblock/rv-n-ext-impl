const MAX_USER_TRAP_NUM: usize = 128;

use crate::mm::PhysPageNum;

pub struct UserTrapInfo {
    pub user_trap_buffer_ppn: PhysPageNum,
    pub user_trap_record_num: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UserTrapRecord {
    pub cause: usize,
    pub message: usize,
}

impl UserTrapInfo {
    // caller of this function should check wheter user interrupt is enabled
    pub unsafe fn push_trap_record(&mut self, trap_record: &UserTrapRecord) {
        if self.user_trap_record_num < MAX_USER_TRAP_NUM {
            let head_ptr: *mut UserTrapRecord =
                self.user_trap_buffer_ppn.get_mut::<UserTrapRecord>();
            let tail_ptr = head_ptr.offset(self.user_trap_record_num as isize);
            tail_ptr.write(*trap_record);
            self.user_trap_record_num += 1;
        }
    }
}
